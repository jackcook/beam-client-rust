extern crate rustc_serialize;
use rustc_serialize::json;

use ::request::{BeamRequest, HttpMethod};
use error::Error;

use ::models::ingest::BeamIngest;

/// Result from a call returning multiple ingests.
pub type BeamIngestsResult = Result<Vec<BeamIngest>, Error>;

/// Routes that can be used to retrieve ingest data.
pub struct IngestsRoutes {}

impl IngestsRoutes {
    /// Creates a new ingests routes instance.
    pub fn new() -> Self {
        IngestsRoutes {}
    }

    /// Retrieves all ingest servers.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.ingests.get_ingests();
    ///
    /// match res {
    ///     Ok(ingests) => println!("There are {} ingests available.", ingests.len()),
    ///     Err(_) => println!("error retrieving ingests :(")
    /// }
    /// ```
    pub fn get_ingests(&self) -> BeamIngestsResult {
        let endpoint = String::from("/ingests");
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: Vec<BeamIngest> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }
}
