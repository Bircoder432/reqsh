use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

const REQUESTS_FILE: &str = "requests.txt";

pub fn get_request(url: &str) -> String {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(REQUESTS_FILE)
        .unwrap();

    writeln!(file, "GET {url}").unwrap();
    let response = Command::new("curl")
        .arg(url)
        .output()
        .map_err(|err| err.to_string())
        .unwrap();

    if response.status.success() {
        String::from_utf8(response.stdout).unwrap().to_string()
    } else {
        String::from_utf8(response.stderr).unwrap().to_string()
    }
}
