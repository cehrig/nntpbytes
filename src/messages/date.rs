use crate::decoder::decoder::{Encode, ExpectedResponse};
use crate::decoder::ExpectedResponseCode;
use crate::messages::{Decode, Decoder, ResponseCodeTuples};
use crate::{Error, Result};
use bytes::{BufMut, BytesMut};
use std::io::Write;

pub struct DateRequest;

#[derive(Default)]
pub struct DateResponse {
    text: String,
}

impl DateRequest {
    pub fn new() -> Self {
        Self
    }
}

impl DateResponse {
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Encode for DateRequest {
    fn encoder(&self, bytes: &mut BytesMut) -> Result<()> {
        write!(bytes.writer(), "DATE").map_err(Error::encode)
    }
}

impl ExpectedResponse for DateRequest {
    type Response = DateResponse;
}

impl ExpectedResponseCode for DateResponse {
    const CODES: ResponseCodeTuples = &[(111, false, true)];
}

impl Decode for DateResponse {
    fn decoder(&mut self, bytes: &mut Decoder, _: u16) -> Result<()>
    where
        Self: Sized,
    {
        self.text = bytes.get_line()?.unwrap_or_default();

        Ok(())
    }
}
