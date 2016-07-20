#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamVoDData {
    /// The width of the VoD.
    pub Width: i32,
    /// The height of the VoD.
    pub Height: i32,
    /// The FPS (frames per second) of the VoD.
    pub Fps: Option<f32>,
    /// The bitrate of the VoD.
    pub Bitrate: Option<i32>
}
