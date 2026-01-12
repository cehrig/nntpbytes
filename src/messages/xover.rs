use crate::decoder::decoder::{Encode, ExpectedResponse};
use crate::decoder::ExpectedResponseCode;
use crate::messages::{Decode, Decoder, ResponseCodeTuples};
use crate::{Error, Result};
use bytes::{BufMut, BytesMut};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::io::Write;

pub enum RangeType {
    Single(usize),
    Start(usize),
    StartEnd(usize, usize),
}

pub struct XoverRequest {
    _type: RangeType,
}

#[derive(Debug)]
pub struct XoverMessage {
    number: usize,
    subject: String,
    author: String,
    date: chrono::DateTime<Utc>,
}

#[derive(Default)]
pub struct XoverResponse {
    messages: Vec<XoverMessage>,
}

impl XoverRequest {
    pub fn new(_type: RangeType) -> Self {
        Self { _type }
    }
}

impl XoverMessage {
    fn new(number: usize, subject: String, author: String, date: chrono::DateTime<Utc>) -> Self {
        Self {
            number,
            subject,
            author,
            date,
        }
    }

    pub fn number(&self) -> usize {
        self.number
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn date(&self) -> chrono::DateTime<Utc> {
        self.date
    }
}

impl XoverResponse {
    pub fn messages(&self) -> &Vec<XoverMessage> {
        &self.messages
    }
}

impl Encode for XoverRequest {
    fn encoder(&self, bytes: &mut BytesMut) -> Result<()> {
        let parameter = match self._type {
            RangeType::Single(start) => format!("{}", start),
            RangeType::Start(start) => format!("{}-", start),
            RangeType::StartEnd(start, end) => format!("{}-{}", start, end),
        };

        write!(bytes.writer(), "XOVER {}", parameter).map_err(Error::encode)
    }
}

impl ExpectedResponse for XoverRequest {
    type Response = XoverResponse;
}

impl ExpectedResponseCode for XoverResponse {
    const CODES: ResponseCodeTuples = &[(224, true, true)];
}

impl Decode for XoverResponse {
    fn decoder(&mut self, bytes: &mut Decoder, _: u16) -> Result<()>
    where
        Self: Sized,
    {
        let _ = bytes.line();

        while let Some(mut line) = bytes.line() {
            let number = line.get_with_delimiter(&[b'\t'])?;
            let subject = line.get_with_delimiter(&[b'\t'])?;
            let author = line.get_with_delimiter(&[b'\t'])?;
            let time = line.get_with_delimiter::<String>(&[b'\t'])?;

            let Some(dt) = parse_datetime(&time) else {
                continue;
            };

            self.messages
                .push(XoverMessage::new(number, subject, author, dt));
        }
        Ok(())
    }
}

fn parse_datetime(s: &str) -> Option<DateTime<Utc>> {
    if let Ok(dt) = DateTime::parse_from_rfc2822(s) {
        return Some(dt.with_timezone(&Utc));
    }

    let formats = [
        "%a, %d %b %y %H:%M:%S UTC",
        "%a, %d %b %y %H:%M:%S GMT",
        "%a, %d %b %Y %H:%M:%S UTC",
        "%a, %d %b %Y %H:%M:%S GMT",
    ];

    for fmt in formats {
        if let Ok(dt) = DateTime::parse_from_str(s, fmt) {
            return Some(dt.with_timezone(&Utc));
        }

        if let Ok(dt) = NaiveDateTime::parse_from_str(s, fmt) {
            return Some(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
        }
    }

    None
}
