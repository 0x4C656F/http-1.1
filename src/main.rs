use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
mod body;
mod content_type;
mod method;
use method::HttpMethod;
#[derive(Debug)]
struct Header(String, String);
impl Header {
    fn from_header_str(s: &str) -> Option<Header> {
        let pair: Vec<&str> = s.split(':').map(|str| str.trim()).collect();
        if pair.len() != 2 {
            return None;
        }

        Some(Header(pair[0].to_owned(), pair[1].to_owned()))
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).expect("Unluck in reading");

    let req = String::from_utf8_lossy(&buf);

    let entries: Vec<&str> = req.split('\n').collect();
    for entry in entries.iter() {
        let h = Header::from_header_str(entry);
        if let Some(header) = h {
            println!("{:?}", header);
        } else {
            println!("Header not found")
        }
    }

    let body = body::parse_body(entries[entries.len() - 1]).unwrap_or_default();

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
