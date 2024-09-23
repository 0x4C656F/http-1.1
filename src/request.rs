use crate::body::parse_body;
use crate::method::HttpMethod;
use std::io::{Read, Write};
use std::net::TcpStream;

use serde_json::Value;
struct Request {
    headers: Vec<Header>,
    raw_body: Option<String>,
    method: HttpMethod,
}

impl Request {
    fn body(&self) -> Option<Value> {
        if let Some(raw) = &self.raw_body {
            let body = parse_body(raw.as_str()).ok();
            body
        } else {
            None
        }
    }
}

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

pub fn handle_request(mut stream: TcpStream) -> Request {
    let mut buf = [0; 1024];
    stream.read(&mut buf).expect("Unluck in reading");

    let req = String::from_utf8_lossy(&buf);

    let entries: Vec<&str> = req.split('\n').collect();

    let method = HttpMethod::try_from(entries[0]).expect("Method invalid");

    let headers: Vec<Header> = entries
        .iter()
        .filter_map(|e| Header::from_header_str(e))
        .collect();

    let raw_body = entries.last().map(|val| val.to_string());

    Request {
        raw_body,
        headers,
        method,
    }
}
