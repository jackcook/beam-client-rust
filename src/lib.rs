extern crate curl;
extern crate rustc_serialize;

use curl::http;
use rustc_serialize::json;

use std::str;

pub mod error;
use error::Error;

pub mod models;
use models::channel::BeamChannel;

pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete
}

pub type BeamResult = Result<BeamChannel, Error>;

pub struct Beam {}

impl Beam {
    pub fn new() -> Self {
        Beam {}
    }

    fn get_url(&self, endpoint: String) -> String {
        format!("https://beam.pro/api/v1{}", endpoint)
    }

    pub fn request(&self, endpoint: String, request_type: HttpMethod) -> BeamResult {
        let mut handle = http::handle();

        let request = match request_type {
            HttpMethod::Get => handle.get(self.get_url(endpoint)),
            HttpMethod::Post => handle.post(self.get_url(endpoint), ""),
            HttpMethod::Put => handle.put(self.get_url(endpoint), ""),
            HttpMethod::Patch => handle.patch(self.get_url(endpoint), ""),
            HttpMethod::Delete => handle.delete(self.get_url(endpoint)),
        };

        let response = match request.exec() {
            Ok(data) => data,
            Err(data) => return Err(Error::Http(data)),
        };

        let raw_body = match str::from_utf8(response.get_body()) {
            Ok(raw_body) => raw_body,
            Err(_) => return Err(Error::Json),
        };

        let decoded: BeamChannel = json::decode(raw_body).unwrap();
        Ok(decoded)
    }
}
