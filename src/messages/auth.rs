use crate::decoder::decoder::{Encode, ExpectedResponse};
use crate::decoder::ExpectedResponseCode;
use crate::messages::{Decode, Decoder, ResponseCodeTuples};
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

impl ExpectedResponseCode for AuthinfoResponse {
    const CODES: ResponseCodeTuples =
        &[(281, false, true), (381, false, true), (502, false, false)];
}

impl Decode for AuthinfoResponse {
    fn decoder(&mut self, bytes: &mut Decoder, _: u16) -> Result<()>
    where
        Self: Sized,
    {
        self.text = bytes.get_line()?.unwrap_or_default();

        Ok(())
    }
}
