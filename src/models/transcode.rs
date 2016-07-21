#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamTranscode {
    /// The transcode's identifier.
    pub id: i32,
    /// The identifier of the profile the transcode belongs to.
    pub profileId: i32,
    /// The name of the transcode.
    pub name: String,
    /// The transcode's title.
    pub title: String,
    /// True if you need to be a partner to have access to this transcode.
    pub requiresPartner: bool,
    /// The width of a stream with this transcode.
    pub width: i32,
    /// The height of a stream with this transcode.
    pub height: i32,
    /// The bitrate of a stream with this transcode.
    pub bitrate: i32,
    /// The frames per second of a stream with this transcode.
    pub fps: i32
}
