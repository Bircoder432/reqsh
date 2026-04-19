pub struct Request {
    pub method: RequestMethod,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

pub enum RequestMethod {
    GET,
    POST,
}

impl RequestMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            RequestMethod::GET => "GET",
            RequestMethod::POST => "POST",
        }
    }
}

impl Request {
    pub fn new(method: RequestMethod, url: String, body: Option<String>) -> Self {
        Self {
            method,
            url,
            headers: Vec::new(),
            body,
        }
    }
}
