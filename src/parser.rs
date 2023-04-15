use mio::net::TcpStream;
use std::io::{BufRead, BufReader, ErrorKind};
// use std::net::TcpStream;

use super::{HttpMethod, HttpRequest};

pub fn read_request(stream: &mut TcpStream) -> Result<HttpRequest, &'static str> {
    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();
    // we know that a connection is coming but the tcp stream may not be ready yet, so we loop until the connection is ready
    let mut i = 0;
    // we don't want to hang for ever, so we stop and return err after 100 iterations.
    loop {
        match reader.read_line(&mut request_line) {
            Err(err) if err.kind() == ErrorKind::WouldBlock && i < 100 => {
                i += 1;
                continue;
            }
            Err(err) if err.kind() == ErrorKind::WouldBlock => {
                return Err("found stale connection");
            },
            Err(_) => {
                return Err("failed to read tcp stream")
            }
            Ok(0) => return Err("got empty content"),
            _ => break,
        }
    }

    return match parse_method(&request_line)? {
        HttpRequest {
            path,
            method: HttpMethod::Get,
        } => Ok(HttpRequest {
            path,
            method: HttpMethod::Get,
        }),
        HttpRequest {
            method: HttpMethod::Post,
            ..
        } => Err("Can't support post yet"),
    };
}

fn parse_method(request_line: &str) -> Result<HttpRequest, &'static str> {
    let mut request_parts = request_line.split_whitespace();
    let (method, path, version) = (
        request_parts.next(),
        request_parts.next(),
        request_parts.next(),
    );
    if path.is_none() || version.is_none() {
        return Err("Invalid request");
    }
    let path = path.unwrap().to_string();

    return if let Some(method) = method {
        match method {
            "GET" => Ok(HttpRequest {
                path,
                method: HttpMethod::Get,
            }),
            "POST" => Ok(HttpRequest {
                path,
                method: HttpMethod::Post,
            }),
            _ => Err("Unsupported method"),
        }
    } else {
        Err("Invalid request")
    };
}
