use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter, Read, prelude::*},
    net::{SocketAddr, TcpListener, TcpStream},
};

use chrono::Utc;
mod http;
use http::request::HTTPRequest;

fn main() -> std::io::Result<()> {
    const SERVER_NAME: &str = "HTTPServer/0.0.1";
    let addrs = SocketAddr::from(([127, 0, 0, 1], 7878));

    let listener = TcpListener::bind(&addrs)?;
    let mut response: Vec<u8> = Vec::new();

    //datetime
    let utc_time = Utc::now().format("%a, %d %b %Y %H:%M:%S %Z");

    let mut file = File::open("src/content/HelloWorld.html").unwrap();
    let mut response_content: Vec<u8> = Vec::new();
    let content_length: usize = file.read_to_end(&mut response_content).unwrap();
    let response_header_f: String = format!(
        "HTTP/1.1 200 OK\r\nDate: {utc_time}\r\nServer: {SERVER_NAME}\r\nLast-Modified: Wed, 22 Jul 2009 19:15:56 GMT\r\nContent-Length: {content_length}\r\nContent-Type: text/html\r\nConnection: Closed\r\n\r\n"
    );
    print!("{:#?}",file.metadata());
    let response_header_b: &[u8] = response_header_f.as_bytes();
    response.extend_from_slice(response_header_b);
    response.extend_from_slice(&response_content);

    println!("\n- Address: http://{:#?}\n", &addrs);

    for stream in listener.incoming() {
        handle_conncection(stream?, response.as_slice())?;
    }

    Ok(())
}

fn handle_conncection(stream: TcpStream, response_bytes: &[u8]) -> std::io::Result<()> {
    let mut reader = BufReader::new(&stream);

    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    let request_line = request_line.trim();

    let mut headers: Vec<String> = Vec::new();
    let mut content_length: usize = 0;

    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            break; // linha em branco = fim dos headers
        }

        if trimmed.to_lowercase().starts_with("content-length:") {
            let val = trimmed.split(':').nth(1).unwrap_or("0").trim();
            content_length = val.parse().unwrap_or(0);
        }

        headers.push(trimmed.to_string());
    }

    let mut header_fields: HashMap<String, String> = HashMap::new();

    for fields in &headers {
        let line = fields
            .split_once(':')
            .map(|elem| (elem.0.trim(), elem.1.trim()))
            .unwrap_or(("", ""));

        header_fields.insert(line.0.to_string(), line.1.to_string());
    }

    let mut body_bytes = vec![0u8; content_length];

    if content_length > 0 {
        reader.read_exact(&mut body_bytes)?; // reader.read_exact continua leitura apos o \r\n
    }

    let body = String::from_utf8_lossy(&body_bytes);

    let req_line_parts: Vec<&str> = request_line.split_ascii_whitespace().collect();

    let http_req = HTTPRequest {
        method: req_line_parts[0].to_owned(),
        uri: req_line_parts[1].to_owned(),
        http_version: req_line_parts[2].to_owned(),
        header_fields: header_fields,
        body: body.to_string(),
    };

    println!(
        "Request Method: {:#?}, URI: {:#?}, HTTPVersion: {:#?}\nRequest Header: {:#?}\nRequest Body: {:#?}",
        http_req.method, http_req.uri, http_req.http_version, http_req.header_fields, http_req.body,
    );

    // envio de reposta apos leitura de requisicao
    let mut writer = BufWriter::new(&stream);
    writer.write_all(response_bytes)?;
    writer.flush()?;
    Ok(())
}
