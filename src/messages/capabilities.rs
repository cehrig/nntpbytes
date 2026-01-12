use crate::decoder::decoder::{Encode, ExpectedResponse};
use crate::decoder::{ExpectedResponseCode, ResponseCodeTuples};
use crate::messages::{Decode, Decoder};
use crate::{Error, Result};
use bytes::{BufMut, BytesMut};
use std::io::Write;

pub struct CapabilitiesRequest;

#[derive(Default)]
pub struct CapabilitiesResponse {
    caps: Vec<String>,
}

impl CapabilitiesRequest {
    pub fn new() -> Self {
        Self
    }
}

impl CapabilitiesResponse {
    pub fn text(&self) -> &Vec<String> {
        &self.caps
    }
}

impl Encode for CapabilitiesRequest {
    fn encoder(&self, bytes: &mut BytesMut) -> Result<()> {
        write!(bytes.writer(), "CAPABILITIES").map_err(Error::encode)
    }
}

impl ExpectedResponse for CapabilitiesRequest {
    type Response = CapabilitiesResponse;
}

impl ExpectedResponseCode for CapabilitiesResponse {
    const CODES: ResponseCodeTuples = &[(101, true, true)];
}

impl Decode for CapabilitiesResponse {
    fn decoder(&mut self, bytes: &mut Decoder, _: u16) -> Result<()>
    where
        Self: Sized,
    {
        // Discard first line
        let _ = bytes.line();

        while let Ok(Some(line)) = bytes.get_line() {
            self.caps.push(line);
        }

        Ok(())
    }
}
