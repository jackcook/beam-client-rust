use curl::http;
use std::str;

use ::error::Error;

/// The REST method being used in an HTTP request.
pub enum HttpMethod {
    /// Retrieves a resource.
    Get,
    /// Creates a resource.
    Post,
    /// Updates a resource.
    Put,
    /// Updates a resource.
    Patch,
    /// Deletes a resource.
    Delete
}

/// Result from a call to the Beam API, containing the response text and any errors.
pub type BeamResult = Result<String, Error>;

/// The most low-level class used to make requests to the Beam backend.
pub struct BeamRequest {}

impl BeamRequest {
    fn get_url(endpoint: String) -> String {
        format!("https://beam.pro/api/v1{}", endpoint)
    }

    /// Makes a request to Beam's backend.
    ///
    /// # Arguments
    ///
    /// * `endpoint` The endpoint of the request being made.
    /// * `request_type` The type of REST request being made.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::request::{BeamRequest, HttpMethod};
    /// match BeamRequest::request(String::from("/channels"), HttpMethod::Get) {
    ///     Ok(ref raw_body) => println!("returned {}", raw_body),
    ///     Err(err) => println!("error while making request: {}", err)
    /// }
    /// ```
    pub fn request(endpoint: String, request_type: HttpMethod) -> BeamResult {
        let mut handle = http::handle();

        let request = match request_type {
            HttpMethod::Get => handle.get(BeamRequest::get_url(endpoint)),
            HttpMethod::Post => handle.post(BeamRequest::get_url(endpoint), ""),
            HttpMethod::Put => handle.put(BeamRequest::get_url(endpoint), ""),
            HttpMethod::Patch => handle.patch(BeamRequest::get_url(endpoint), ""),
            HttpMethod::Delete => handle.delete(BeamRequest::get_url(endpoint)),
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
