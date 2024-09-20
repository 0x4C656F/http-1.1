use std::fmt::Display;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

enum HttpMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

impl TryFrom<&str> for HttpMethod {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PATCH" => Ok(HttpMethod::Patch),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            _ => Err("HTTP Method not found"),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).expect("Unluck in reading");

    let req = String::from_utf8_lossy(&buf);

    let entries: Vec<&str> = req.split('\n').collect();
    let method: Result<HttpMethod, &str> = HttpMethod::try_from(entries[0]);

    match method {
        Ok(m) => println!("Got method"),
        Err(e) => eprintln!("Error: {}", e),
    }

    for entr in entries {
        println!("{}", entr);
    }
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
