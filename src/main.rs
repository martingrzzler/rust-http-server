#![allow(dead_code)]
use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_folder_path = env::var("PUBLIC_PATH".to_string()).unwrap_or(default_path);

    let handler = WebsiteHandler::new(public_folder_path);

    server.run(handler);
}
