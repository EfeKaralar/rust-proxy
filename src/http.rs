#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: u8,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub version: u8,
    pub code: u16, 
    pub reason: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    // TODO: Implement the following
    // Parse an HTTP request from a buffer
    // Convert request back to bytes
    // Check if connection should be kept alive
    // Add or update a header   
}

impl HttpResponse {
    /// Parse an HTTP response from a buffer
    // Convert response back to bytes
    // Check if connection should be kept alive
}


