use std::{path::Path, thread, time::Duration};

use super::HttpRequest;

pub fn handle_request(request: &Result<HttpRequest, &str>) -> String {
    let content = match request {
        Ok(request) => valid_response(request),
        Err(exception) => invalid_response(exception)
    };
    content
}

fn valid_response(request: &HttpRequest) -> String {
    let path = curate_path(&request.path);
    let (path, status_line) = if Path::exists(Path::new(&path)) 
    { 
        (path.as_str(), "HTTP/1.1 200 OK") 
    } else 
    { 
        ("/notfound", "HTTP/1.1 404 NOT FOUND")
    };

    let content = std::fs::read_to_string(&path)
        .expect(format!("Should be able to open file {}", path).as_str());

    craft_response(status_line, &content)
}

fn curate_path(request_path: &str) -> String {
    let path = request_path.strip_prefix("/").unwrap_or(request_path);
    match path {
        "/" | "" => String::from("index.html"),
        "sleep" => { 
            println!("sleeping");
            thread::sleep(Duration::from_secs(1));
            println!("woke up");
            format!("{path}.html")
        }
        path => { println!("got {path}"); format!("{path}.html") }
    }
}

fn invalid_response(exception: &str) -> String {
    let status_line = "HTTP/1.1 500 Internal Server Error";
    let content = exception;
    craft_response(status_line, content)
}

fn craft_response(status_line: &str, content: &str) -> String {
    let length = content.len();
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}")
}