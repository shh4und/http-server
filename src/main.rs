use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, prelude::*},
    net::{SocketAddr, TcpListener, TcpStream},
};

// enum RequestMethods {
//     GET,
//     HEAD,
//     POST,
//     PUT,
//     DELETE,
//     CONNECT,
//     OPTIONS,
//     TRACE,
// }

pub struct RequestLine {
    method: String,
    uri: String,
    http_version: String,
}

// pub struct HeaderFields {
//     host: String,            //127.0.0.1:7878",
//     acept_encoding: Vec<u8>, //gzip, deflate",
//     accept: String,          //*/*",
//     connection: String,      // keep-alive",
//     content_length: u32,     //9",
//     user_agent: String,      //
//     content_type: String,    // application/x-www-form-urlencoded; charset=utf-8",
// }

// pub struct MessageBody {
//     body: Vec<u8>,
// }

// pub struct HTTPRequest {
//     request_line: RequestLine,
//     header_fields: HeaderFields,
//     message_body: MessageBody,
// }

fn main() -> std::io::Result<()> {
    let addrs = SocketAddr::from(([127, 0, 0, 1], 7878));

    let listener = TcpListener::bind(&addrs)?;
    let mut response: Vec<u8> = Vec::new();
    let response_header: &[u8] = "HTTP/1.1 200 OK
        \nDate: Mon, 27 Jul 2009 12:28:53 GMT
        \nServer: Apache/2.2.14 (Win32)
        \nLast-Modified: Wed, 22 Jul 2009 19:15:56 GMT
        \nContent-Type: text/html
        \nConnection: Closed
        \n\n"
        .as_bytes();
    response.extend_from_slice(response_header);
    let mut file = File::open("src/content/HelloWorld.html").unwrap();
    let mut response_content: Vec<u8> = Vec::new();
    file.read_to_end(&mut response_content).unwrap();
    response.extend_from_slice(&response_content);

    println!("\n- SocketAddr: {:#?}", &addrs);

    for stream in listener.incoming() {
        handle_conncection(stream?, response.as_slice())?;
    }

    Ok(())
}

fn handle_conncection(stream: TcpStream, response_bytes: &[u8]) -> std::io::Result<()> {
    let mut buf_writer = BufWriter::new(&stream);

    _ = buf_writer.write_all(response_bytes);
    let mut buf = String::new();
    _ = BufReader::new(&stream).read_to_string(&mut buf)?;
    let mut request: Vec<&str> = buf.splitn(2, "\r\n").collect();

    let request_line: Vec<&str> = request.remove(0).split_ascii_whitespace().collect();

    let request_remainder: Vec<&str> = request[0].splitn(2, "\r\n\r\n").collect();
    let request_header: Vec<&str> = request_remainder[0].split("\r\n").collect();
    let request_body: &str = request_remainder[1];

    let http_req_line = RequestLine{
        method: request_line[0].to_owned(),
        uri: request_line[1].to_owned(),
        http_version: request_line[2].to_owned(),
    };

    println!(
        "Request's Method: {}, URI: {}, HTTPVersion: {}\nRequest Header: {request_header:#?}\nRequest Body: {request_body:#?}"
        , http_req_line.method, http_req_line.uri, http_req_line.http_version);
    Ok(())
}
