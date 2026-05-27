use std::collections::HashMap;

pub struct ShellState {
    base_url: Option<String>,
    headers: HashMap<String, String>,
    variables: HashMap<String, String>,
}

impl Default for ShellState {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellState {
    pub fn new() -> Self {
        ShellState {
            base_url: None,
            headers: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = Some(url.to_string());
    }

    pub fn get_base_url(&self) -> Option<&str> {
        self.base_url.as_deref()
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn set_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }

    pub fn get_variables(&self) -> &HashMap<String, String> {
        &self.variables
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_state_starts_empty() {
        let ctx = ShellState::new();
        assert!(ctx.base_url.is_none());
        assert!(ctx.headers.is_empty());
        assert!(ctx.variables.is_empty());
    }

    #[test]
    fn set_base_url_updates_state() {
        let mut state = ShellState::new();

        state.set_base_url("https://example.com");

        assert_eq!(state.get_base_url(), Some("https://example.com"));
    }

    #[test]
    fn set_header_adds_header() {
        let mut state = ShellState::new();

        state.set_header("Content-Type".to_string(), "application/json".to_string());

        assert_eq!(
            state.get_headers().get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn set_variables_add_variable() {
        let mut state = ShellState::new();

        state.set_variable("Token".to_string(), "random-auth-token".to_string());

        assert_eq!(
            state.get_variable("Token"),
            Some(&"random-auth-token".to_string())
        );
    }
}
