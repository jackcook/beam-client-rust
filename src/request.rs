use hyper::Client;
use hyper::status::StatusCode;

use std::io::Read;
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
        let client = Client::new();
        let request = match request_type {
            HttpMethod::Get => client.get(&BeamRequest::get_url(endpoint)),
            HttpMethod::Post => client.post(&BeamRequest::get_url(endpoint)),
            HttpMethod::Put => client.put(&BeamRequest::get_url(endpoint)),
            HttpMethod::Patch => client.patch(&BeamRequest::get_url(endpoint)),
            HttpMethod::Delete => client.delete(&BeamRequest::get_url(endpoint)),
        };

        let mut response = match request.send() {
            Ok(data) => data,
            Err(data) => return Err(Error::Http(data))
        };

        let mut buf = String::new();
        response.read_to_string(&mut buf).expect("Failed to read response.");

        match response.status {
            StatusCode::Ok => return Ok(buf),
            _ => return Err(Error::Api(response.status, buf))
        }
    }
}
