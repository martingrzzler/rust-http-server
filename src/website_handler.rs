use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;
use std::marker::{Send, Sync};

pub struct WebsiteHandler {
  public_path: String,
}

impl WebsiteHandler {
  pub fn new(public_path: String) -> Self {
    Self { public_path }
  }

  fn read_file(&self, file_path: &str) -> Option<Vec<u8>> {
    let path = format!("{}/{}", self.public_path, file_path);
    match fs::canonicalize(path) {
      Ok(path) => {
        if path
          .starts_with(fs::canonicalize(&self.public_path).unwrap_or(std::path::PathBuf::new()))
        {
          fs::read(path).ok()
        } else {
          println!("Directory Traversal Attack Attempt detected: {}", file_path);
          None
        }
      }
      Err(_) => None,
    }
  }
}

impl Handler for WebsiteHandler {
  fn handle_request(&self, request: &Request) -> Response {
    match request.method() {
      Method::GET => match request.path() {
        "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
        "/takes-time" => {
          for _ in 0..100000000 {
            let _ = 5 * 5 * 5 * 5;
          }
          Response::new(StatusCode::Ok, Some(b"Long simulation completed!".to_vec()))
        }
        path => match self.read_file(path) {
          Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
          None => Response::new(StatusCode::NotFound, self.read_file("404.html")),
        },
      },
      _ => Response::new(StatusCode::NotFound, None),
    }
  }
}

unsafe impl Send for WebsiteHandler {}
unsafe impl Sync for WebsiteHandler {}
