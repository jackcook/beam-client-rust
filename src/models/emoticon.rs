/// An emoticon object.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamEmoticon {
    /// The x coordinate of the emoticon in its spritesheet.
    pub x: i32,
    /// The y coordinate of the emoticon in its spritesheet.
    pub y: i32,
    /// The width of the emoticon in its spritesheet.
    pub width: i32,
    /// The height of the emoticon in its spritesheet.
    pub height: i32
}
