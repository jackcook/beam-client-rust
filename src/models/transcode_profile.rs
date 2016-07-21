use ::models::transcode::BeamTranscode;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamTranscodeProfile {
    /// The identifier of the transcode profile.
    pub id: i32,
    /// The name of the transcode profile.
    pub name: String,
    /// The transcodes available with the profile.
    pub transcodes: Vec<BeamTranscode>
}
