/// A type object. Currently having issues due to the fact that "type" is a Rust keyword.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamType {
    /// The type's identifier.
    pub id: i32,
    /// The name of the type.
    pub name: String,
    /// The type's parent category.
    pub parent: String,
    /// A short description of the type.
    pub description: String,
    /// Where the type was found.
    pub source: String,
    /// The number of viewers watching channels playing this type.
    pub viewersCurrent: i32,
    /// The type's cover image URL string.
    pub coverUrl: String,
    /// The number of channels currently playing this type.
    pub online: i32
}
