extern crate rustc_serialize;
use rustc_serialize::json;

use ::request::{BeamRequest, HttpMethod};
use error::Error;

use ::models::recording::BeamRecording;

/// Result from a call returning a channel's recordings.
pub type BeamRecordingResult = Result<BeamRecording, Error>;

/// Routes that can be used to retrieve recording data.
pub struct RecordingsRoutes {}

impl RecordingsRoutes {
    /// Creates a new recordings routes instance.
    pub fn new() -> Self {
        RecordingsRoutes {}
    }

    /// Retrieves a recording with the specified identifier.
    ///
    /// # Arguments
    ///
    /// * `id` The identifier of the recording being retrieved.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.recordings.get_recording(420);
    ///
    /// match res {
    ///     Ok(recording) => println!("{} people have viewed this recording.", recording.viewsTotal),
    ///     Err(_) => println!("error retrieving recording :(")
    /// }
    /// ```
    pub fn get_recording(&self, id: u32) -> BeamRecordingResult {
        let endpoint = String::from(format!("/recordings/{}", id));
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: BeamRecording = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }
}
