use serde_json::Value;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
mod body;
mod method;
use method::HttpMethod;
fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).expect("Unluck in reading");

    let req = String::from_utf8_lossy(&buf);

    let entries: Vec<&str> = req.split('\n').collect();
    let method: Result<HttpMethod, &str> = HttpMethod::try_from(entries[0]);

    match method {
        Ok(m) => println!("Got method, {:?}", m),
        Err(e) => eprintln!("Error: {}", e),
    }
    let body = body::parse_body(entries[entries.len() - 1]).unwrap_or_default();
    println!("{}", body);

    let res = "Fuck you".as_bytes();
    stream.write(res).expect("Balls");
}
fn main() {
    let socket = TcpListener::bind("127.0.0.1:8080").expect("Shit");
    for stream in socket.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(|| handle_client(s));
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
