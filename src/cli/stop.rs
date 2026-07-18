use crate::api::routes::PORT;
use colored::Colorize;
use core::result::Result::Ok;
use log::info;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::language;

pub fn stop(quiet: bool) -> Result<(), std::io::Error> {
    if !quiet {
        info!("{}", language::en_us::AUTOPILOT_SHUTDOWN.yellow());
    }
    let mut stream = match TcpStream::connect(format!("localhost:{}", PORT)) {
        Ok(stream) => stream,
        Err(err) => return Err(err),
    };

    let request = format!(
        "GET /shutdown HTTP/1.1\r\nHost: localhost:{}\r\nConnection: close\r\n\r\n",
        PORT
    );

    if stream.write_all(request.as_bytes()).is_err() {
        eprintln!("Warning: failed to stop auto_pilot");
    }

    Ok(())
    // if let Err(e) =  {
    //     eprintln!("Warning: failed to stop auto_pilot: {}", e);
    // }
}
