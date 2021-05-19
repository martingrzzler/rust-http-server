use std::str::FromStr;

pub enum Method {
  GET,
  POST,
  PUT,
  PATCH,
  DELETE,
}

impl FromStr for Method {
  type Err = MethodError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "GET" => Ok(Self::GET),
      "POST" => Ok(Self::POST),
      "PUT" => Ok(Self::PUT),
      "PATCH" => Ok(Self::PATCH),
      "DELETE" => Ok(Self::DELETE),
      _ => Err(MethodError),
    }
  }
}

pub struct MethodError;
