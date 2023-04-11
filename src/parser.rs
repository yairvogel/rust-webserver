use std::{io::{BufReader, BufRead}, net::TcpStream, fs::read_link};

use super::{HttpRequest, HttpMethod};

pub fn read_request(stream: &mut TcpStream) -> Result<HttpRequest, &'static str> {
    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();
    
    match reader.read_line(&mut request_line) {
        Err(_) => return Err("failed to read from tcp stream"),
        Ok(0) => return Err("got empty content"),
        _ => {}
    }
    
    println!("first line: {request_line}");
    return match parse_method(&request_line)? {
        HttpRequest { path, method: HttpMethod::Get } => Ok(HttpRequest { path, method: HttpMethod::Get }),
        HttpRequest { method: HttpMethod::Post, .. } => Err("Can't support post yet"),
    }
}

fn parse_method(request_line: &str) -> Result<HttpRequest, &'static str> {
    let mut request_parts = request_line.split_whitespace();
    let (method, path, version) = (request_parts.next(), request_parts.next(), request_parts.next());
    if path.is_none() || version.is_none() {
        return Err("Invalid request")
    }
    let path = path.unwrap().to_string();

    return if let Some(method) = method { match method {
        "GET" => Ok(HttpRequest { path, method: HttpMethod::Get }),
        "POST" => Ok(HttpRequest { path, method: HttpMethod::Post }),
        _ => Err("Unsupported method")
        }
    } else {
        Err("Invalid request")
    }
}
