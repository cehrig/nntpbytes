use crate::decoder::{Decode, Decoder, ExpectedResponse, ExpectedResponseCode, ResponseCodeTuples};
use crate::{Error, Result};
use std::ops::Deref;

pub mod article;
pub mod auth;
pub mod capabilities;
pub mod date;
pub mod greeting;
pub mod group;
pub mod head;
pub mod list;
pub mod newsgroups;
pub mod xover;

pub use greeting::*;

#[derive(Default)]
pub struct Response<T> {
    code: u16,
    kind: T,
}

impl<T> Response<T> {
    pub fn code(&self) -> u16 {
        self.code
    }
}

impl<T> Response<T>
where
    T: ExpectedResponseCode,
{
    pub fn ok(&self) -> bool {
        self.kind.ok(self.code)
    }
}

impl<T> ExpectedResponse for Response<T> {
    type Response = T;
}

impl<T> ExpectedResponseCode for Response<T> {
    const CODES: ResponseCodeTuples = &[(0, false, false)];
}

impl<T> Decode for Response<T>
where
    T: Decode + ExpectedResponseCode,
{
    fn decoder(&mut self, bytes: &mut Decoder, _: u16) -> Result<()> {
        if bytes.len() < 3 {
            return Err(Error::DecodeNeedMoreBytes);
        };

        // We already have set a code and the buffer has advanced, do not attempt to read code
        // another time
        if self.code == 0 {
            self.code = bytes.get()?;
        }

        self.kind.decode(bytes, self.code)
    }
}

impl<T> Deref for Response<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}
