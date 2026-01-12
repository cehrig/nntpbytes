use crate::decoder::decoder::{Decode, Decoder, Encode, ExpectedResponse};
use crate::decoder::ExpectedResponseCode;
use crate::messages::{GreetingResponse, Response};
use crate::server::NewsServer;
use crate::{Error, Result};
use bytes::{Buf, BytesMut};
use std::fmt::Display;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;
use tokio_rustls::{rustls, TlsConnector};

enum NewsConnectionKind {
    Plaintext(TcpStream),
    Tls(TlsStream<TcpStream>),
}

impl NewsConnectionKind {
    fn plaintext(stream: TcpStream) -> Self {
        NewsConnectionKind::Plaintext(stream)
    }

    fn tls(stream: TlsStream<TcpStream>) -> Self {
        NewsConnectionKind::Tls(stream)
    }

    async fn read(&mut self, bytes: &mut BytesMut) -> Result<usize> {
        let mut buffer = [0u8; 1024];

        match match self {
            NewsConnectionKind::Plaintext(s) => s.read(&mut buffer).await,
            NewsConnectionKind::Tls(s) => s.read(&mut buffer).await,
        } {
            Ok(n) if n == 0 => return Err(Error::Eof),
            Ok(n) => {
                bytes.extend_from_slice(&buffer[..n]);
                Ok(n)
            }
            Err(e) => Err(Error::ReadError(e)),
        }
    }

    async fn write(&mut self, bytes: &mut BytesMut) -> Result<usize> {
        match match self {
            NewsConnectionKind::Plaintext(s) => s.write(&bytes[..]).await,
            NewsConnectionKind::Tls(s) => s.write(&bytes[..]).await,
        } {
            Ok(n) => {
                bytes.advance(n);
                Ok(n)
            }
            Err(e) => Err(Error::WriteError(e)),
        }
    }
}

pub struct NewsConnection {
    server: NewsServer,
    inner: NewsConnectionKind,
}

impl NewsConnection {
    pub async fn connect(server: NewsServer, tls: bool) -> Result<Self> {
        let mut conn = match tls {
            true => Self::connect_tls(server).await,
            false => Self::connect_plaintext(server).await,
        }?;

        conn.read::<Response<GreetingResponse>>().await?;

        Ok(conn)
    }

    async fn connect_plaintext(server: NewsServer) -> Result<Self> {
        let stream = TcpStream::connect(server.addr())
            .await
            .map_err(Error::TcpError)?;

        Ok(Self {
            server,
            inner: NewsConnectionKind::plaintext(stream),
        })
    }

    async fn connect_tls(server: NewsServer) -> Result<Self> {
        let NewsConnection {
            server,
            inner: NewsConnectionKind::Plaintext(stream),
        } = Self::connect_plaintext(server).await?
        else {
            unreachable!()
        };

        let connector = tls_connector();
        let sni = rustls_pki_types::ServerName::try_from(server.name().to_string())
            .map_err(Error::InvalidSni)?;

        Ok(Self {
            server,
            inner: NewsConnectionKind::tls(
                connector
                    .connect(sni, stream)
                    .await
                    .map_err(Error::TlsError)?,
            ),
        })
    }

    pub fn fqdn(&self) -> impl Display {
        &self.server
    }

    pub async fn request<T>(&mut self, request: T) -> Result<Response<T::Response>>
    where
        T: Encode + ExpectedResponse,
        T::Response: Default + Decode + ExpectedResponseCode,
    {
        self.request_explicit::<T::Response, T>(request).await
    }

    pub async fn request_explicit<R, T>(&mut self, request: T) -> Result<Response<R>>
    where
        T: Encode + ExpectedResponse,
        R: Default + Decode + ExpectedResponseCode,
    {
        let mut buffer = BytesMut::new();
        request.encode(&mut buffer)?;

        self.write(buffer).await?;
        self.read::<Response<R>>().await
    }

    async fn read<T>(&mut self) -> Result<T>
    where
        T: Default + Decode + ExpectedResponseCode,
    {
        let mut data = T::default();
        let mut decoder = Decoder::new();

        loop {
            let _ = self.inner.read(&mut decoder).await?;

            match data.decode(&mut decoder, 0) {
                Ok(_) => return Ok(data),
                Err(Error::DecodeNeedMoreBytes) => continue,
                Err(e) => return Err(e),
            }
        }
    }

    async fn write(&mut self, mut data: BytesMut) -> Result<usize> {
        self.inner.write(&mut data).await
    }
}

fn tls_connector() -> TlsConnector {
    let mut store = rustls::RootCertStore::empty();
    store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = tokio_rustls::rustls::client::ClientConfig::builder()
        .with_root_certificates(store)
        .with_no_client_auth();

    TlsConnector::from(Arc::new(config))
}
