use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter, Read, prelude::*},
    net::{SocketAddr, TcpListener, TcpStream},
    time::SystemTime,
    usize,
};

mod http;
use http::request::HTTPRequest;
use http::response::HTTPResponse;

fn main() -> std::io::Result<()> {
    let addrs = SocketAddr::from(([127, 0, 0, 1], 7878));

    // faz o bind do endereco do socket, se a porta estiver disponivel
    let listener = TcpListener::bind(&addrs)?;

    // aloca vetores dinamicos
    let response: Vec<u8>;
    let mut response_body: Vec<u8> = Vec::new();
    // tenta abrir arquivo com conteudo html
    let content_path = "src/content/HelloWorld.html".to_string();

    let mut file = File::open(&content_path)?;

    // Content-Length:
    let content_length: usize = file.read_to_end(&mut response_body)?;

    // Last-Modified:
    let metadata = file.metadata()?;
    let last_modified: SystemTime = metadata.modified()?;

    let mut http_response = HTTPResponse::new(content_length, last_modified, &content_path);

    response = http_response.full_response_u8(response_body);

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
