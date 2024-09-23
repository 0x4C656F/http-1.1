use std::net::TcpListener;
use std::thread;
mod body;
mod content_type;
mod method;
mod request;
fn main() {
    let socket = TcpListener::bind("127.0.0.1:8080").expect("Shit");
    for stream in socket.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(|| {
                    request::handle_request(s);
                });
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
