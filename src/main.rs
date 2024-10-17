// Setting the server up...
mod lib;
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use lib::ThreadPool;
use std::io::prelude::*;
fn  main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // add a thread pool to allow multitasking
    let pool = ThreadPool::new(4);
    for stream in listener.incoming(){

        //let stream = stream.unwrap();

        pool.execute(|| {
            handle_request(stream.unwrap());
        })
       

    }
}
fn handle_request(mut stream : TcpStream){
    let mut container: [u8; 1024]  = [0;1024]; // This is the buffer that will hold the request content!!
    stream.read(&mut container).unwrap();
    // The format of an http response :
    // http:version status-code reason-phase CRLF
    let content = String::from("Hello");
    let status_line = "HTTP/1.1 200 PALESTINE";
    let headers = format!("Content-Length : {}", content.len());
    // Response formatting, properly aligning the output
    let response = format!("{}\r\n{}\r\n\r\n{}", status_line, headers, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

