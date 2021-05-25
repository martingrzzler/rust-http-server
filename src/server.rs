use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

pub struct Server {
  addr: String,
}

impl Server {
  pub fn new(addr: String) -> Self {
    Self { addr }
  }
  pub fn run(self) {
    let listener = TcpListener::bind(&self.addr).unwrap();
    println!("Server listening on {}...", self.addr);
    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 1024];
          match stream.read(&mut buffer) {
            Ok(_) => {
              println!("Received a request: {}", String::from_utf8_lossy(&buffer));
              let response = match Request::try_from(&buffer[..]) {
                Ok(request) => {
                  dbg!(request);
                  let response =
                    Response::new(StatusCode::Ok, Some("<h1>Hello there!</h1>".to_string()));
                  response.send(&mut stream);
                }
                Err(e) => {
                  println!("ParseError: {}", e);
                  Response::new(StatusCode::BadRequest, None).send(&mut stream);
                }
              };
              if let Err(e) = response.send(&mut stream) {
                println!("Failed to send response: {}", e);
              }
            }
          }
        }
        Err(e) => println!("Failed to establish connection: {}", e),
      }
    }
  }
}
