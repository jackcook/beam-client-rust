use ::models::vod_data::BeamVoDData;

/// A VoD (video on demand) object.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamVoD {
    /// Metadata containing the VoD's size/video info.
    pub data: Option<BeamVoDData>,
    /// The VoD's identifier.
    pub id: i32,
    /// The VoD's storage node.
    pub storageNode: String,
    /// The last URL component of the VoD.
    pub mainUrl: String,
    /// The format of the VoD (e.g. hls, thumbnail).
    pub format: String,
    /// The date string, in UTC, at which this VoD was created.
    pub createdAt: String,
    /// The date string, in UTC, at which this VoD was last updated.
    pub updatedAt: String,
    /// The identifier of the recording this VoD belongs to.
    pub recordingId: i32
}
