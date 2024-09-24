use std::io::Read;
use std::net::TcpListener;
use std::thread;
mod body;
mod content_type;
mod method;
mod request;
mod response;

pub const HTTP_VERSION: &str = "HTTP/1.1";

fn main() {
    let socket = TcpListener::bind("127.0.0.1:8080").expect("Shit");
    for stream in socket.incoming() {
        match stream {
            Ok(mut s) => {
                thread::spawn(move || {
                    let req = request::handle_request(&mut s);
                    dbg!(req);
                    let res = response::handle_response(&mut s);
                    dbg!(res);
                });
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
