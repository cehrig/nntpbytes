use crate::decoder::{ExpectedResponseCode, ResponseCodeTuples};
use crate::messages::{Decode, Decoder};
use crate::Result;

#[derive(Default)]
pub struct GreetingResponse {
    greeting: String,
}

impl GreetingResponse {
    pub fn greeting(&self) -> &str {
        &self.greeting
    }
}

impl ExpectedResponseCode for GreetingResponse {
    const CODES: ResponseCodeTuples = &[(200, false, true)];
}

impl Decode for GreetingResponse {
    fn decoder(&mut self, bytes: &mut Decoder, _: u16) -> Result<()> {
        self.greeting = bytes.get_line()?.unwrap_or_default();

        Ok(())
    }
}
