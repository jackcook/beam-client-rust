use ::models::vod::BeamVoD;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamRecording {
    /// The recording's identifier.
    pub id: i32,
    /// The name of the recording.
    pub name: Option<String>,
    /// The type id being played in the recording.
    pub typeId: Option<i32>,
    /// The state of the recording (e.g. available).
    pub state: String,
    /// The number of views on the recording.
    pub viewsTotal: i32,
    /// The duration of the recording, in seconds.
    pub duration: f32,
    /// The date string, in UTC, at which the recording will expire.
    pub expiresAt: String,
    /// The date string, in UTC, at which the recording was created.
    pub createdAt: String,
    /// The date string, in UTC, at which the recording was last updated.
    pub updatedAt: String,
    /// The VoD formats that the recording is stored as.
    pub vods: Vec<BeamVoD>
}
