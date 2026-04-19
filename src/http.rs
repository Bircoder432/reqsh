use reqwest::blocking::Client;

use crate::request::{Request, RequestMethod};

pub fn fetch(request: &Request, base_url: Option<&str>) -> String {
    let client = Client::new();

    let full_url = match base_url {
        Some(base) => format!("{}{}", base, request.url),
        None => format!("{}", request.url),
    };

    match request.method {
        RequestMethod::GET => {
            let res = client.get(full_url).send();
            if let Ok(res) = res {
                return res.text().unwrap();
            } else {
                return format!("Error fetching response");
            }
        }
        RequestMethod::POST => {
            let res = client.post(full_url).send();
            if let Ok(res) = res {
                return res.text().unwrap();
            } else {
                return format!("Error fetching response");
            }
        }
    }
}
