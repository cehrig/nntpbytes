use std::fmt::{Debug, Formatter};

pub mod connection;
pub mod messages;
pub mod server;

type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    ServerNameNotFound,
    ServerPortInvalid(String),
    InvalidSni,
    TcpError(std::io::Error),
    TlsError(std::io::Error),
    ReadError(std::io::Error),
    WriteError(std::io::Error),
    Eof,
    UnexpectedResponseCode(u16),
    DecodeNeedMoreBytes,
    DecodeError(Box<dyn std::error::Error>),
    EncodeError(Box<dyn std::error::Error>),
}

impl Error {
    fn decode<E>(e: E) -> Error
    where
        E: std::error::Error + 'static,
    {
        Error::DecodeError(Box::new(e))
    }

    fn encode<E>(e: E) -> Error
    where
        E: std::error::Error + 'static,
    {
        Error::EncodeError(Box::new(e))
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let e = match self {
            Error::ServerNameNotFound => format!("Server must not be empty"),
            Error::ServerPortInvalid(e) => format!("Port {} is invalid", e),
            Error::InvalidSni => format!("SNI is invalid"),
            Error::TcpError(e) => format!("Unable to TCP connect {}", e),
            Error::TlsError(e) => format!("Unable to TLS handshake {}", e),
            Error::ReadError(e) => format!("Read failed with error {}", e),
            Error::WriteError(e) => format!("Write failed with error {}", e),
            Error::Eof => format!("Stream closed"),
            Error::UnexpectedResponseCode(c) => format!("Unexpected response code {}", c),
            Error::DecodeNeedMoreBytes => format!("Need more bytes"),
            Error::DecodeError(e) => format!("Decode error {}", e),
            Error::EncodeError(e) => format!("Encode error {}", e),
        };

        write!(f, "{}", e)
    }
}

trait Pipe {
    fn pipe<F, U>(self, cb: F) -> U
    where
        F: Fn(Self) -> U,
        Self: Sized;
}

impl<T> Pipe for T {
    fn pipe<F, U>(self, cb: F) -> U
    where
        F: Fn(T) -> U,
    {
        cb(self)
    }
}
