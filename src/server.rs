use crate::http::{ParseError, Request, Response, StatusCode};
use crate::thread_pool::ThreadPool;
use std::convert::TryFrom;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

pub trait Handler: Send + Sync {
  fn handle_request(&self, request: &Request) -> Response;
  fn handle_bad_request(&self, e: &ParseError) -> Response {
    println!("Failed to parse request: {}", e);
    Response::new(StatusCode::BadRequest, None)
  }
}

#[derive(Clone)]
pub struct Server {
  addr: String,
}

impl Server {
  pub fn new(addr: String) -> Self {
    Self { addr }
  }

  fn handle_connection(&self, mut stream: TcpStream, handler: Arc<impl Handler + 'static>) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
      Ok(_) => {
        let response = match Request::try_from(&buffer[..]) {
          Ok(request) => handler.handle_request(&request),
          Err(e) => handler.handle_bad_request(&e),
        };

        if let Err(e) = response.send(&mut stream) {
          println!("Failed to send response: {}", e);
        }
      }
      Err(e) => println!("Failed to read from connection: {}", e),
    }
  }

  pub fn run(self, handler: impl Handler + 'static) -> ! {
    let listener = TcpListener::bind(&self.addr).unwrap();
    let pool = ThreadPool::new(6);
    println!("Server listening on {}...", self.addr);
    let server = Arc::new(self);
    let handler = Arc::new(handler);
    loop {
      match listener.accept() {
        Ok((stream, _)) => {
          let h = Arc::clone(&handler);
          let s = Arc::clone(&server);
          pool.execute(move || s.handle_connection(stream, h))
        }
        Err(e) => println!("Failed to establish connection: {}", e),
      }
    }
  }
}
