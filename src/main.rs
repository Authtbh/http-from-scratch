use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let first_line = request.lines().next().unwrap();
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap();
    let path = parts.next().unwrap();

    println!("Request: {} {}", method, path);

    let response = match (method, path) {
        ("GET", "/") => {
            let contents = fs::read_to_string("index.html").unwrap_or_else(|_| String::from("404 Not Found"));
            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
                contents.len(),
                contents
            )
        }
        ("GET", path) => {
            let filename = format!(".{}", path);
            match fs::read_to_string(&filename) {
                Ok(contents) => {
                    let content_type = if filename.ends_with(".css") {
                        "text/css"
                    } else if filename.ends_with(".js") {
                        "application/javascript"
                    } else {
                        "text/plain"
                    };
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
                        contents.len(),
                        content_type,
                        contents
                    )
                }
                Err(_) => {
                    let contents = "404 Not Found";
                    format!(
                        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                        contents.len(),
                        contents
                    )
                }
            }
        }
        ("POST", "/echo") => {
            let body = request.split("\r\n\r\n").nth(1).unwrap_or_default();
            format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                body.len(),
                body
            )
        }
        _ => {
            let contents = "404 Not Found";
            format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                contents.len(),
                contents
            )
        }
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Server listening on port 8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
