use crate::decoder::decoder::{Encode, ExpectedResponse};
use crate::decoder::ExpectedResponseCode;
use crate::messages::{Decode, Decoder, ResponseCodeTuples};
use crate::{Error, Result};
use bytes::{BufMut, BytesMut};
use std::collections::HashMap;
use std::io::Write;

pub enum HeadType {
    MessageId(String),
    MessageNumber(usize),
    Empty,
}

pub struct HeadRequest {
    _type: HeadType,
}

#[derive(Default)]
pub struct HeadResponse {
    number: usize,
    id: String,
    headers: HashMap<String, Vec<String>>,
}

impl HeadRequest {
    pub fn new(_type: HeadType) -> Self {
        Self { _type }
    }
}

impl HeadResponse {
    pub fn number(&self) -> usize {
        self.number
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn headers(&self) -> &HashMap<String, Vec<String>> {
        &self.headers
    }
}

impl Encode for HeadRequest {
    fn encoder(&self, bytes: &mut BytesMut) -> Result<()> {
        let parameter = match &self._type {
            HeadType::MessageId(i) => format!("<{}>", i.to_string()),
            HeadType::MessageNumber(n) => n.to_string(),
            HeadType::Empty => "".to_string(),
        };

        write!(bytes.writer(), "HEAD {}", parameter).map_err(Error::encode)
    }
}

impl ExpectedResponse for HeadRequest {
    type Response = HeadResponse;
}

impl ExpectedResponseCode for HeadResponse {
    const CODES: ResponseCodeTuples = &[(221, true, true), (430, true, false)];
}

impl Decode for HeadResponse {
    fn decoder(&mut self, bytes: &mut Decoder, _: u16) -> Result<()>
    where
        Self: Sized,
    {
        self.number = bytes.get()?;
        self.id = bytes.get()?;

        let mut current_key = None;

        while let Some(line) = bytes.get_line()? {
            if line.is_empty() {
                break;
            }

            // Header start
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim().to_string();
                let value = value.trim().to_string();

                self.headers.entry(key.clone()).or_default().push(value);

                current_key = Some(key);
            }

            // Folded header
            if line.starts_with(' ') || line.starts_with('\t') {
                if let Some(key) = &current_key {
                    if let Some(values) = self.headers.get_mut(key) {
                        if let Some(last) = values.last_mut() {
                            last.push(' ');
                            last.push_str(line.trim_start());
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
