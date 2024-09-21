#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

impl TryFrom<&str> for HttpMethod {
    type Error = &'static str;
    fn try_from(v: &str) -> Result<Self, Self::Error> {
        let header_split: Vec<&str> = v.split('/').map(|val| val.trim()).collect();

        let value = header_split[0];
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
