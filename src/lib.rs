use std::io::Write;

use mio::net::TcpStream;
// use std::net::TcpStream;
mod parser;
mod handler;

pub fn handle_request(mut stream: TcpStream) {
    let request = parser::read_request(&mut stream);
    if let Err(error) = request { eprintln!("{}", error) }
    let content = handler::handle_request(&request);
    stream.write_all(content.as_bytes()).unwrap();
}

pub struct HttpRequest {
    path: String,
    method: HttpMethod
}

enum HttpMethod {
    Get,
    Post
}