use std::io::{Read, Write};
use std::net::TcpStream;

use crate::api::handlers::{API_SERVICE_NAME, HealthResponse};
use crate::api::routes::PORT;

pub fn status() {
    match check_if_running() {
        true => {
            println!("Autopilot is running");
        }
        false => {
            println!("Autopilot is not running");
        }
    }
}

pub fn check_if_running() -> bool {
    let mut stream = match TcpStream::connect(format!("localhost:{}", PORT)) {
        Ok(stream) => stream,
        Err(_) => return false,
    };

    let request = format!(
        "GET /health HTTP/1.1\r\nHost: localhost:{}\r\nConnection: close\r\n\r\n",
        PORT
    );

    if stream.write_all(request.as_bytes()).is_err() {
        return false;
    }

    let mut response = String::new();
    if stream.read_to_string(&mut response).is_err() {
        return false;
    }

    let body = match response.split("\r\n\r\n").nth(1) {
        Some(b) => b.trim(),
        None => return false,
    };
    let json_body = serde_json::from_str::<HealthResponse>(body).expect("Failed to parse response");

    json_body.status == "ok" && json_body.service == API_SERVICE_NAME.to_string()
}
