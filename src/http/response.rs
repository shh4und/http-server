use chrono::{DateTime, Utc};
use std::time::SystemTime;

pub struct HTTPResponse {
    pub http_version: String,
    pub status_code: String,
    pub reason_phrase: String,
    pub header_fields: Vec<(String, String)>,
    pub body: String,
    pub f_header: String,
}

impl HTTPResponse {
    pub fn new(content_length: usize, last_modified: SystemTime, content_path: &str) -> Self {
        let http_version = "HTTP/1.1".to_string();
        let status_code = "200".to_string();
        let reason_phrase = "OK".to_string();

        let server_response_time = Utc::now().to_rfc2822();
        let server_name = "rustServer/0.0.1".to_string();
        let last_modified_datetime: DateTime<Utc> = last_modified.clone().into();
        let last_modified_string: String = last_modified_datetime.to_rfc2822();

        let mut header_fields: Vec<(String, String)> = Vec::new();

        header_fields.push(("Date".to_string(), server_response_time));
        header_fields.push(("Server".to_string(), server_name));
        header_fields.push(("Last-Modified".to_string(), last_modified_string));
        header_fields.push(("Content-Length".to_string(), content_length.to_string()));
        header_fields.push(("Content-Type".to_string(), "text/html".to_string()));
        header_fields.push(("Connection".to_string(), "Closed".to_string()));

        Self {
            http_version: http_version,
            status_code: status_code,
            reason_phrase: reason_phrase,
            header_fields: header_fields,
            body: content_path.to_string(),
            f_header: "".to_string(),
        }
    }

    fn format_header(&mut self) {
        let mut f_header = format!(
            "{} {} {}\r\n",
            self.http_version, self.status_code, self.reason_phrase
        );

        for (k, v) in self.header_fields.clone().into_iter() {
            f_header += format!("{k}: {v}\r\n").as_str();
        }
        f_header += "\r\n";
        self.f_header = f_header;
    }

    pub fn full_response_u8(&mut self, response_body: Vec<u8>) -> Vec<u8> {
        self.format_header();

        let mut response: Vec<u8> = Vec::new();

        // concatenando header fields com content
        response.extend_from_slice(self.f_header.as_bytes());
        response.extend_from_slice(&response_body);
        response
    }
}
