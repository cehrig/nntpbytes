use crate::decoder::decoder::{Encode, ExpectedResponse};
use crate::decoder::ExpectedResponseCode;
use crate::messages::head::{HeadResponse, HeadType};
use crate::messages::{Decode, Decoder, ResponseCodeTuples};
use crate::{Error, Result};
use bytes::{BufMut, BytesMut};
use std::collections::HashMap;
use std::io::Write;

pub struct ArticleRequest {
    _type: HeadType,
}

#[derive(Default)]
pub struct ArticleResponse {
    header: HeadResponse,
    body: Vec<u8>,
}

impl ArticleRequest {
    pub fn new(_type: HeadType) -> Self {
        Self { _type }
    }
}

impl ArticleResponse {
    pub fn number(&self) -> usize {
        self.header.number()
    }

    pub fn id(&self) -> &str {
        &self.header.id()
    }

    pub fn headers(&self) -> &HashMap<String, Vec<String>> {
        &self.header.headers()
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }
}

impl Encode for ArticleRequest {
    fn encoder(&self, bytes: &mut BytesMut) -> Result<()> {
        let parameter = match &self._type {
            HeadType::MessageId(i) => format!("<{}>", i.to_string()),
            HeadType::MessageNumber(n) => n.to_string(),
            HeadType::Empty => "".to_string(),
        };

        write!(bytes.writer(), "ARTICLE {}", parameter).map_err(Error::encode)
    }
}

impl ExpectedResponse for ArticleRequest {
    type Response = ArticleResponse;
}

impl ExpectedResponseCode for ArticleResponse {
    const CODES: ResponseCodeTuples = &[(220, true, true), (430, false, false)];
}

impl Decode for ArticleResponse {
    fn decoder(&mut self, bytes: &mut Decoder, code: u16) -> Result<()>
    where
        Self: Sized,
    {
        let mut header = HeadResponse::default();
        header.decoder(bytes, code)?;

        self.header = header;
        self.body = bytes.as_slice().iter().cloned().collect();

        Ok(())
    }
}
