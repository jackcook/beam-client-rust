extern crate curl;
extern crate rustc_serialize;

use curl::http;
use std::str;

pub mod error;
use error::Error;

pub mod models;
pub mod routes;

use routes::channels::ChannelsRoutes;

/// The HTTP method being used in a URL request.
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete
}

/// Result from a call to the Beam API, containing the response text and any errors.
pub type BeamResult = Result<String, Error>;

/// The main class of the API client.
pub struct Beam {}

impl Beam {
    /// Creates a new Beam API client instance.
    ///
    /// # Example
    /// ```rust
    /// use beam::Beam;
    /// let beam = Beam::new();
    /// ```
    pub fn new() -> Self {
        Beam {}
    }

    /// The method through which all channel methods are accessed.
    ///
    /// # Example
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.channels_routes().get_channel_with_id(252);
    /// ```
    pub fn channels_routes(&self) -> ChannelsRoutes {
        ChannelsRoutes {
            beam: self
        }
    }

    fn get_url(&self, endpoint: String) -> String {
        format!("https://beam.pro/api/v1{}", endpoint)
    }

    fn request(&self, endpoint: String, request_type: HttpMethod) -> BeamResult {
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
            Ok(data) => data,
            Err(_) => return Err(Error::Json),
        };

        Ok(raw_body.to_string())
    }
}
