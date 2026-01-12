use crate::decoder::utility::{Pipe, PositionWithLength};
use crate::Error;
use bytes::{Buf, BufMut, BytesMut};
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

// Message Response COde
type ResponseCode = u16;

// Message Response Type
type IsMultiLineResponse = bool;

// Message Response Code indicates OK response
type IsOk = bool;

// Response Code to Response Type mappings
pub(crate) type ResponseCodeTuples = &'static [(ResponseCode, IsMultiLineResponse, IsOk)];

// Single Line Termination sequence
const SINGLE_LINE_TERMINATION: &[u8] = b"\r\n";

// Multi Line Termination sequence
const MULTI_LINE_TERMINATION: &[u8] = b"\r\n.\r\n";

pub struct Decoder {
    bytes: BytesMut,
}

pub trait Decode: ExpectedResponseCode {
    fn decode(&mut self, bytes: &mut Decoder, code: u16) -> crate::Result<()>
    where
        Self: Sized,
    {
        // Check if response code is expected and the type of line termination
        let term = if Self::CODES
            .iter()
            .find(|(c, _, _)| *c == code)
            .ok_or_else(|| Error::UnexpectedResponseCode(code))?
            .1
        {
            MULTI_LINE_TERMINATION
        } else {
            SINGLE_LINE_TERMINATION
        };

        // Check if - depending on line termination - we have enough bytes for the whole response
        if bytes.len() < term.len() || &bytes[bytes.len() - term.len()..] != term {
            return Err(Error::DecodeNeedMoreBytes);
        }

        // Remove line termination string if we have all we need
        let len = bytes.len();
        if code > 0 {
            bytes.truncate(len - term.len());
        }

        // Decode the message
        self.decoder(bytes, code)
    }

    fn decoder(&mut self, bytes: &mut Decoder, code: u16) -> crate::Result<()>
    where
        Self: Sized;
}

pub trait Encode {
    fn encode(&self, bytes: &mut BytesMut) -> crate::Result<()> {
        self.encoder(bytes)?;

        write!(bytes.writer(), "\r\n").map_err(Error::encode)
    }

    fn encoder(&self, bytes: &mut BytesMut) -> crate::Result<()>;
}

pub trait ExpectedResponse {
    type Response;
}

pub trait ExpectedResponseCode {
    const CODES: ResponseCodeTuples;

    fn ok(&self, code: ResponseCode) -> bool {
        Self::CODES.iter().any(|(c, _, ok)| *c == code && *ok)
    }
}

impl Deref for Decoder {
    type Target = BytesMut;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl DerefMut for Decoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

impl Decoder {
    pub(crate) fn new() -> Self {
        Self {
            bytes: BytesMut::new(),
        }
    }

    pub(crate) fn with_bytes(bytes: BytesMut) -> Self {
        Self { bytes }
    }

    pub(crate) fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    fn position_multi(&self, need: &[u8]) -> Option<PositionWithLength> {
        self.bytes
            .windows(need.len())
            .position(|window| window == need)
            .map(|p| PositionWithLength::new(p, need.len()))
    }

    fn line_end(&self) -> Option<usize> {
        self.position_multi(SINGLE_LINE_TERMINATION)
            .map(|p| p.position())
    }

    fn parse<T>(&mut self, to: usize) -> crate::Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + 'static,
    {
        self.bytes
            .split_to(to)
            .as_ref()
            .pipe(|b| str::from_utf8(&b[..]).map_err(Error::decode))?
            .parse()
            .map_err(Error::decode)
    }

    pub(crate) fn get_line(&mut self) -> crate::Result<Option<String>> {
        let Some(decoder) = self.line() else {
            return Ok(None);
        };

        Ok(Some(
            std::str::from_utf8(decoder.as_slice())
                .map_err(Error::decode)?
                .to_string(),
        ))
    }

    pub(crate) fn line(&mut self) -> Option<Decoder> {
        if self.bytes.is_empty() {
            return None;
        }

        let end = self.line_end();
        let buffer = self.bytes.split_to(end.unwrap_or(self.bytes.len()));

        if end.is_some() {
            self.bytes.advance(2);
        }

        Some(Decoder::with_bytes(buffer))
    }

    pub(crate) fn get_with_end<T>(&mut self, end: Option<PositionWithLength>) -> crate::Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + 'static,
    {
        let read = self.parse(
            end.as_ref()
                .map(|p| p.position())
                .unwrap_or(self.bytes.len()),
        );

        if let Some(end) = end {
            self.bytes.advance(end.length());
        }

        read
    }

    pub(crate) fn get_with_delimiter<T>(&mut self, delimiter: &[u8]) -> crate::Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + 'static,
    {
        let end = [
            self.position_multi(delimiter),
            self.position_multi(SINGLE_LINE_TERMINATION),
        ]
        .into_iter()
        .flatten()
        .min();

        self.get_with_end(end)
    }

    pub(crate) fn get<T>(&mut self) -> crate::Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + 'static,
    {
        self.get_with_delimiter(b" ")
    }

    pub(crate) fn all<T>(&mut self) -> crate::Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + 'static,
    {
        self.get_with_end(Some(PositionWithLength::new(self.len(), 0)))
    }
}

impl<T> Encode for &T
where
    T: Encode,
{
    fn encoder(&self, bytes: &mut BytesMut) -> crate::Result<()> {
        <T as Encode>::encoder(self, bytes)
    }
}

impl<T> ExpectedResponse for &T
where
    T: ExpectedResponse,
{
    type Response = T::Response;
}
