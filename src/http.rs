use anyhow::{Result, anyhow};
use httparse;
use std::str;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String, // `GET`, `POST` etc
    pub path: String,   // /index.html or /api/users etc.
    pub version: u8,    // 1 or 0 denoting HTTP 1.1 and 1.0
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub version: u8,
    pub code: u16,      // 200, 400, 500 etc
    pub reason: String, // OK, NOT FOUND, etc
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

// Each has these core methods:
// - parse() - Parse from bytes
// - to_bytes() - Convert back to bytes
// - should_keep_alive() - Check Connection header
// - set_header() - Add/modify headers
impl HttpRequest {
    // Parse an HTTP request from a buffer
    pub fn parse(buffer: &[u8]) -> Result<(HttpRequest, usize)> {
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);
        let status = req.parse(buffer)?;

        let headers_len = match status {
            httparse::Status::Complete(len) => len,
            httparse::Status::Partial => return Err(anyhow!("Partial Request")),
        };

        let method = req.method.ok_or_else(|| anyhow!("No method"))?.to_string();
        let path = req.path.ok_or_else(|| anyhow!("No path"))?.to_string();
        let version = req.version.ok_or_else(|| anyhow!("No version"))?;

        let headers = req
            .headers
            .iter()
            .map(|h| {
                (
                    h.name.to_string(),
                    String::from_utf8_lossy(h.value).to_string(),
                )
            })
            .collect::<Vec<(String, String)>>();

        // Determine body length
        let content_length = headers
            .iter()
            .find_map(|(name, value)| {
                if name.eq_ignore_ascii_case("content-length") {
                    value.parse().ok()
                } else {
                    None
                }
            })
            .unwrap_or(0);
        // Check if we have complete body
        let total_length = headers_len + content_length;
        if buffer.len() < total_length {
            return Err(anyhow!("Incomplete response body"));
        }
        // Extract body
        let body = buffer[headers_len..total_length].to_vec();
        // Return parsed request
        Ok((
            HttpRequest {
                method,
                path,
                version,
                headers,
                body,
            },
            total_length,
        ))
    }

    // Convert request back to bytes
    pub fn to_bytes(self) -> Vec<u8> {
        let mut buffer = Vec::new();
        // Step 1: Request line
        buffer.extend_from_slice(
            format!("{} {} HTTP/1.{}\r\n", self.method, self.path, self.version).as_bytes(),
        );
        // Step 2: Headers
        for (header, value) in self.headers {
            buffer.extend_from_slice(format!("{}: {}\r\n", header, value).as_bytes());
        }
        // Step 3: Empty line
        buffer.extend_from_slice(b"\r\n");
        // Step 4: Add body
        buffer.extend_from_slice(&self.body);
        buffer
    }
    // Check if connection should be kept alive
    // Add or update a header
}

impl HttpResponse {
    // Parse an HTTP response from a buffer
    // Convert response back to bytes
    // Check if connection should be kept alive
}
