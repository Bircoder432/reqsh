use reqwest::blocking::Response;

pub fn display_response(res: Response) -> String {
    let mut output = String::new();

    // Status Line
    let reason = res.status().canonical_reason().unwrap_or_else(|| "");
    let status_line = format!("{:?} {} {}\n", res.version(), res.status().as_str(), reason);
    output.push_str(&status_line);

    // Headers
    for (k, v) in res.headers() {
        let line = format!("{}: {:?}\n", k, v);
        output.push_str(&line);
    }
    output.push_str("\n");

    // Body
    let body = res.text().unwrap_or_default();
    output.push_str(&body);
    output.push_str("\n");

    output
}
