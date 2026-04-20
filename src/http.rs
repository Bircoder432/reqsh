use crate::request::{Method, Request};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};

pub fn fetch(request: &Request, base_url: Option<&str>) -> String {
    let client = Client::new();
    let full_url = if (request.url.starts_with("/"))
        && let Some(base_url) = base_url
    {
        format!("{base_url}{}", request.url)
    } else {
        request.url.clone()
    };

    let mut headers = HeaderMap::new();
    for (key, value) in &request.headers {
        headers.insert(
            HeaderName::from_bytes(key.as_bytes()).unwrap(),
            HeaderValue::from_bytes(value.as_bytes()).unwrap(),
        );
    }

    match request.method {
        Method::GET => {
            let res = client.get(full_url).headers(headers).send();
            match res {
                Ok(response) => response.text().unwrap(),
                Err(e) => return format!("{}", e),
            }
        }
        Method::POST => {
            let res = if let Some(body) = &request.body {
                client
                    .post(full_url)
                    .headers(headers)
                    .body(body.clone())
                    .send()
            } else {
                client.post(full_url).send()
            };

            match res {
                Ok(response) => response.text().unwrap(),
                Err(e) => format!("{}", e),
            }
        }
    }
}
