use crate::HashMap;
pub struct HTTPRequest {
    pub method: String,
    pub uri: String,
    pub http_version: String,
    pub header_fields: HashMap<String, String>,
    pub body: String,
}
