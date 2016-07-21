extern crate rustc_serialize;
use rustc_serialize::json;

use ::request::{BeamRequest, HttpMethod};
use error::Error;

use ::models::transcode_profile::BeamTranscodeProfile;

/// Result from a call returning multiple transcode profiles.
pub type BeamTranscodeProfilesResult = Result<Vec<BeamTranscodeProfile>, Error>;

/// Routes that can be used to retrieve transcode data.
pub struct TranscodesRoutes {}

impl TranscodesRoutes {
    /// Creates a new transcodes routes instance.
    pub fn new() -> Self {
        TranscodesRoutes {}
    }

    /// Retrieves all available transcodes.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.transcodes.get_transcode_profiles();
    ///
    /// match res {
    ///     Ok(profiles) => println!("There are {} transcode profiles available.", profiles.len()),
    ///     Err(_) => println!("error retrieving transcode profiles :(")
    /// }
    /// ```
    pub fn get_transcode_profiles(&self) -> BeamTranscodeProfilesResult {
        let endpoint = String::from("/transcodes");
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: Vec<BeamTranscodeProfile> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }
}
