mod lib;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;
use lib::ThreadPool;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_request(stream);
        });
    }
}

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    let content = "Hello";
    let status_line = "HTTP/1.1 200 OK";
    let headers = format!("Content-Length: {}", content.len());
    let response = format!("{}\r\n{}\r\n\r\n{}", status_line, headers, content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
