use std::{
    fs::File,
    io::Read,
    io::{BufReader, BufWriter, prelude::*},
    net::{TcpListener, TcpStream},
};

// mod server;
// use server::Server;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut response: Vec<u8> = Vec::new();
    let response_header: &[u8] = "HTTP/1.1 200 OK\nDate: Mon, 27 Jul 2009 12:28:53 GMT\nServer: Apache/2.2.14 (Win32)\nLast-Modified: Wed, 22 Jul 2009 19:15:56 GMT\nContent-Type: text/html\nConnection: Closed\n\n".as_bytes();
    response.extend_from_slice(response_header);
    let mut file = File::open("src/index.html").unwrap();
    let mut response_content: Vec<u8> = Vec::new();
    file.read_to_end(&mut response_content).unwrap();
    response.extend_from_slice(&response_content);

    println!("Serving at: 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_conncection(stream, response.as_slice());
    }
}

fn handle_conncection(stream: TcpStream, response_bytes: &[u8]) {
    let mut buf_writer = BufWriter::new(&stream);

    _ = buf_writer.write_all(response_bytes);

    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");
}
