use std::io::{Read, Write};
use std::net::TcpStream;
fn main() {
    fn make_api_call() -> Result<String, Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect("dummyjson.com:80")?;
        let request = "GET /users HTTP/1.1\r\nHost: dummyjson.com\r\nConnection: close\r\n\r\n";
        stream.write_all(request.as_bytes())?;

        let mut response = String::new();
        stream.read_to_string(&mut response)?;

        // Extract the body from the HTTP response
        if let Some(body_start) = response.find("\r\n\r\n") {
            let body = response[body_start + 4..].to_string();
            Ok(body)
        } else {
            Err("Failed to parse response".into())
        }
    }

    match make_api_call() {
        Ok(body) => println!("Response body: {}", body),
        Err(e) => eprintln!("Error: {}", e),
    }
}
