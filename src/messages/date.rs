use crate::messages::{Decode, Encode, ExpectedResponse, ResponseCodeTuples};
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
        DateRequest
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

impl Decode for DateResponse {
    const CODES: ResponseCodeTuples = &[(111, false)];

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
