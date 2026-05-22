use std::collections::HashMap;

use crate::{
    builtin::Builtin,
    request::{Method, Request},
};

pub enum Parsed {
    Builtin(Builtin),
    Request(Request),
    Exit,
}

pub fn parse(input: String) -> Result<Parsed, String> {
    let first_line = input.lines().next().unwrap();
    let tokens: Vec<&str> = first_line.split_whitespace().collect();

    match tokens[0] {
        "GET" | "POST" | "PUT" | "DELETE" => {
            let result = parse_request(input)?;
            return Ok(Parsed::Request(result));
        }

        "base" | "header" | "help" | "history" | "rerun" => {
            let result = parse_builtin(input)?;
            Ok(Parsed::Builtin(result))
        }

        "exit" => Ok(Parsed::Exit),

        _ => Err(format!("Reference Error: {} not defined", { tokens[0] })),
    }
}

fn parse_request(buffer: String) -> Result<Request, String> {
    if let Some((header_part, body_part)) = buffer.split_once("\n\n") {
        let header_lines: Vec<&str> = header_part.split('\n').collect();

        let req_parts: Vec<&str> = header_lines[0].split_whitespace().collect();
        if req_parts.len() != 2 {
            return Err(format!("usage: METHOD <url> \n[headers]\n[body]"));
        }

        let method = match req_parts[0].to_lowercase().as_str() {
            "get" => Method::GET,
            "post" => Method::POST,
            "put" => Method::PUT,
            "delete" => Method::DELETE,
            _ => panic!("Invalid Method"),
        };
        let path = req_parts[1];

        let mut headers = HashMap::new();
        if header_lines.len() > 1 {
            for line in header_lines.iter().skip(1) {
                if let Some((key, value)) = line.split_once(':') {
                    headers.insert(key.trim().to_string(), value.trim().to_string());
                } else {
                    return Err(format!("Invalid headers"));
                }
            }
        }

        let body = if body_part.trim().is_empty() {
            None
        } else {
            Some(body_part.trim().to_string())
        };

        Ok(Request {
            method,
            path: path.to_string(),
            headers,
            body,
        })
    } else {
        Err(format!("usage: METHOD <url> \n[headers]\n[body]"))
    }
}

fn parse_builtin(line: String) -> Result<Builtin, String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    match tokens[0] {
        "base" => {
            if tokens.len() != 2 {
                Err(format!("usage: base <url>"))
            } else {
                Ok(Builtin::Base(tokens[1].to_string()))
            }
        }
        "header" => {
            if tokens.len() != 3 {
                Err(format!("usage: header <key> <value>"))
            } else {
                Ok(Builtin::Header(
                    tokens[1].to_string(),
                    tokens[2].to_string(),
                ))
            }
        }
        "help" => Ok(Builtin::Help),
        "history" => Ok(Builtin::History),
        "rerun" => {
            if tokens.len() != 2 {
                Err(format!("usage: rerun <index>"))
            } else {
                let idx: usize = tokens[1].parse().unwrap();
                Ok(Builtin::Rerun(idx))
            }
        }
        _ => Err(format!("Invalid Command")),
    }
}
