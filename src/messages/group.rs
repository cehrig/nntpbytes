use crate::decoder::decoder::{Encode, ExpectedResponse};
use crate::decoder::ExpectedResponseCode;
use crate::messages::{Decode, Decoder, ResponseCodeTuples};
use crate::{Error, Result};
use bytes::{BufMut, BytesMut};
use std::io::Write;

pub struct GroupRequest {
    name: String,
}

#[derive(Default)]
pub struct GroupResponse {
    number: usize,
    low: usize,
    high: usize,
    group: String,
}

impl GroupRequest {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl GroupResponse {
    pub fn number(&self) -> usize {
        self.number
    }

    pub fn low(&self) -> usize {
        self.low
    }

    pub fn high(&self) -> usize {
        self.high
    }

    pub fn group(&self) -> &str {
        &self.group
    }
}

impl Encode for GroupRequest {
    fn encoder(&self, bytes: &mut BytesMut) -> Result<()> {
        write!(bytes.writer(), "GROUP {}", self.name).map_err(Error::encode)
    }
}

impl ExpectedResponse for GroupRequest {
    type Response = GroupResponse;
}

impl ExpectedResponseCode for GroupResponse {
    const CODES: ResponseCodeTuples = &[(211, false, true), (411, false, false)];
}

impl Decode for GroupResponse {
    fn decoder(&mut self, bytes: &mut Decoder, code: u16) -> Result<()>
    where
        Self: Sized,
    {
        if code == 411 {
            return Ok(());
        }

        self.number = bytes.get()?;
        self.low = bytes.get()?;
        self.high = bytes.get()?;
        self.group = bytes.get()?;

        Ok(())
    }
}
