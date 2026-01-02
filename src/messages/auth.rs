use crate::messages::{Decode, Encode, ExpectedResponse, ResponseCodeTuples};
use crate::{Error, Result};
use bytes::{BufMut, BytesMut};
use std::io::Write;

pub enum AuthinfoMode {
    Username,
    Password,
}

pub struct AuthinfoRequest {
    mode: AuthinfoMode,
    value: String,
}

#[derive(Default)]
pub struct AuthinfoResponse {
    text: String,
}

impl AuthinfoRequest {
    pub fn new(mode: AuthinfoMode, value: impl ToString) -> Self {
        Self {
            mode,
            value: value.to_string(),
        }
    }
}

impl AuthinfoResponse {
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Encode for AuthinfoRequest {
    fn encoder(&self, bytes: &mut BytesMut) -> Result<()> {
        let mode = match &self.mode {
            AuthinfoMode::Username => "USER",
            AuthinfoMode::Password => "PASS",
        };

        write!(bytes.writer(), "{} {} {}", "AUTHINFO", mode, self.value).map_err(Error::encode)
    }
}

impl ExpectedResponse for AuthinfoRequest {
    type Response = AuthinfoResponse;
}

impl Decode for AuthinfoResponse {
    const CODES: ResponseCodeTuples = &[(281, false), (381, false), (502, false)];

    fn decoder(&mut self, bytes: &mut BytesMut, _: u16) -> Result<()>
    where
        Self: Sized,
    {
        self.text = str::from_utf8(bytes.split_to(bytes.len()).as_ref())
            .map_err(Error::decode)?
            .to_string();

        Ok(())
    }
}
