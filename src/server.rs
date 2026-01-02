use crate::Error;
use std::fmt::{Display, Formatter};

const DEFAULT_NNTP_PORT: u16 = 563;

pub struct NewsServer {
    name: String,
    port: u16,
}

impl NewsServer {
    pub fn new(name: impl ToString, port: u16) -> Self {
        Self {
            name: name.to_string(),
            port,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn port(&self) -> u16 {
        self.port
    }

    pub(crate) fn addr(&self) -> String {
        format!("{}:{}", self.name(), self.port())
    }
}

impl TryFrom<&str> for NewsServer {
    type Error = Error;

    fn try_from(server: &str) -> Result<NewsServer, Self::Error> {
        let mut parts = server.splitn(2, ':');

        let name = parts
            .next()
            .filter(|s| !s.is_empty()) // ensure name is not empty
            .ok_or(Error::ServerNameNotFound)?
            .to_string();

        let port = match parts.next() {
            Some(port_str) => port_str.parse::<u16>().map_err(Error::ServerPortInvalid)?,
            None => DEFAULT_NNTP_PORT,
        };

        Ok(NewsServer::new(name, port))
    }
}

impl TryFrom<String> for NewsServer {
    type Error = Error;

    fn try_from(server: String) -> Result<NewsServer, Self::Error> {
        NewsServer::try_from(server.as_str())
    }
}

impl Display for NewsServer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.addr())
    }
}
