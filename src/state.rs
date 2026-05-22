use std::collections::HashMap;

pub struct ShellState {
    base_url: Option<String>,
    headers: HashMap<String, String>,
}

impl ShellState {
    pub fn new() -> Self {
        ShellState {
            base_url: None,
            headers: HashMap::new(),
        }
    }

    pub fn get_base_url(&self) -> Option<&str> {
        self.base_url.as_deref()
    }

    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = Some(url.to_string());
    }

    pub fn set_header(&mut self, key: String, value: String) -> Option<String> {
        self.headers.insert(key, value)
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}
