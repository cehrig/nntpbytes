use crate::messages::{Decode, ResponseCodeTuples};
use crate::{Error, Result};
use bytes::BytesMut;

#[derive(Default)]
pub struct GreetingResponse {
    greeting: String,
}

impl GreetingResponse {
    pub fn greeting(&self) -> &str {
        &self.greeting
    }
}

impl Decode for GreetingResponse {
    const CODES: ResponseCodeTuples = &[(200, false)];

    fn decoder(&mut self, bytes: &mut BytesMut, _: u16) -> Result<()> {
        self.greeting = str::from_utf8(bytes.split_to(bytes.len()).as_ref())
            .map_err(Error::decode)?
            .to_string();

        Ok(())
    }
}
