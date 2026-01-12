use crate::decoder::decoder::{Encode, ExpectedResponse};
use crate::decoder::ExpectedResponseCode;
use crate::messages::{Decode, Decoder, ResponseCodeTuples};
use crate::{Error, Result};
use bytes::{BufMut, BytesMut};
use chrono::Utc;
use std::io::Write;

pub struct NewsgroupsRequest {
    datetime: chrono::DateTime<Utc>,
}

#[derive(Default)]
pub struct NewsgroupsResponse {
    groups: Vec<String>,
}

impl NewsgroupsRequest {
    pub fn new(datetime: chrono::DateTime<Utc>) -> Self {
        Self { datetime }
    }
}

impl NewsgroupsResponse {
    pub fn groups(&self) -> &Vec<String> {
        &self.groups
    }
}

impl Encode for NewsgroupsRequest {
    fn encoder(&self, bytes: &mut BytesMut) -> Result<()> {
        let date = self.datetime.format("%Y%m%d");
        let time = self.datetime.format("%H%M%S");

        write!(bytes.writer(), "NEWGROUPS {} {}", date, time).map_err(Error::encode)
    }
}

impl ExpectedResponse for NewsgroupsRequest {
    type Response = NewsgroupsResponse;
}

impl ExpectedResponseCode for NewsgroupsResponse {
    const CODES: ResponseCodeTuples = &[(231, true, true)];
}

impl Decode for NewsgroupsResponse {
    fn decoder(&mut self, bytes: &mut Decoder, _: u16) -> Result<()>
    where
        Self: Sized,
    {
        // Discard first line
        let _ = bytes.line();

        while let Ok(Some(line)) = bytes.get_line() {
            self.groups.push(line);
        }

        Ok(())
    }
}
