use std::{
    io::{BufReader, BufWriter, prelude::*},
    net::{TcpListener, TcpStream},
};

// mod server;
// use server::Server;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let response = "HTTP/1.1 200 OK\nDate: Mon, 27 Jul 2009 12:28:53 GMT\nServer: Apache/2.2.14 (Win32)\nLast-Modified: Wed, 22 Jul 2009 19:15:56 GMT\nContent-Length: 88\nContent-Type: text/html\nConnection: Closed\n\n<html><body><h1>Hello, World!</h1></body></html>";
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_conncection(stream, response);
    }
}

fn handle_conncection(stream: TcpStream, resp: &str) {
    let mut buf_writer = BufWriter::new(&stream);

    _ = buf_writer.write_all(resp.as_bytes());

    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
