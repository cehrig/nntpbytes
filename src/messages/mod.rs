use crate::{Error, Pipe, Result};
use bytes::{BufMut, BytesMut};
use std::io::Write;
use std::ops::Deref;

pub mod auth;
pub mod greeting;

pub use greeting::*;

type ResponseCode = u16;

type IsMultiLineResponse = bool;

type ResponseCodeTuples = &'static [(ResponseCode, IsMultiLineResponse)];

const SINGLE_LINE_TERMINATION: &[u8] = b"\r\n";

const MULTI_LINE_TERMINATION: &[u8] = b".\r\n";

#[derive(Default)]
pub struct GenericMessage<T> {
    code: u16,
    kind: T,
}

pub(crate) trait Decode {
    const CODES: ResponseCodeTuples;

    fn decode(&mut self, bytes: &mut BytesMut, code: u16) -> Result<()>
    where
        Self: Sized,
    {
        // Check if response code is expected and the type of line termination
        let term = if Self::CODES
            .iter()
            .find(|(c, _)| *c == code)
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
        if code > 0 {
            bytes.truncate(bytes.len() - term.len());
        }

        // Decode the message
        self.decoder(bytes, code)
    }

    fn decoder(&mut self, bytes: &mut BytesMut, code: u16) -> Result<()>
    where
        Self: Sized;
}

impl<T> GenericMessage<T> {
    pub fn code(&self) -> u16 {
        self.code
    }
}

impl<T> Decode for GenericMessage<T>
where
    T: Decode,
{
    const CODES: ResponseCodeTuples = &[(0, false)];

    fn decoder(&mut self, bytes: &mut BytesMut, code: u16) -> Result<()> {
        if bytes.len() < 3 {
            return Err(Error::DecodeNeedMoreBytes);
        };

        self.code = bytes
            .split_to(4)
            .as_ref()
            .pipe(|b| str::from_utf8(&b[0..3]).map_err(Error::decode))?
            .parse()
            .map_err(Error::decode)?;

        self.kind.decode(bytes, self.code)
    }
}

impl<T> Deref for GenericMessage<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl Decode for () {
    const CODES: ResponseCodeTuples = &[];

    fn decoder(&mut self, bytes: &mut BytesMut, code: u16) -> Result<()>
    where
        Self: Sized,
    {
        Ok(())
    }
}

pub(crate) trait Encode {
    fn encode(&self, bytes: &mut BytesMut) -> Result<()> {
        self.encoder(bytes)?;

        write!(bytes.writer(), "\r\n").map_err(Error::encode)
    }

    fn encoder(&self, bytes: &mut BytesMut) -> Result<()>;
}

pub trait ExpectedResponse {
    type Response;
}
