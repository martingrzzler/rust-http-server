use super::StatusCode;
use std::io::{Result as IoResult, Write};

#[derive(Debug)]
pub struct Response {
  status_code: StatusCode,
  body: Option<Vec<u8>>,
}

impl Response {
  pub fn new(status_code: StatusCode, body: Option<Vec<u8>>) -> Self {
    Self { status_code, body }
  }

  pub fn send(&self, stream: &mut impl Write) -> IoResult<usize> {


    let mut response = format!(
      "HTTP/1.1 {} {}\r\n\r\n",
      self.status_code,
      self.status_code.reason_phrase(),
    ).into_bytes();
    if let Some(body) = &self.body {
      response.extend(body)
    }

    stream.write(&response)
  }
}
