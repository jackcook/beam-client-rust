/// A type object. Currently having issues due to the fact that "type" is a Rust keyword.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamType {
    /// The type's identifier.
    pub id: i32,
    /// The name of the type.
    pub name: String,
    /// The type's parent category.
    pub parent: Option<String>,
    /// A short description of the type.
    pub description: Option<String>,
    /// Where the type was found.
    pub source: Option<String>,
    /// The number of viewers watching channels playing this type.
    pub viewersCurrent: Option<i32>,
    /// The type's cover image URL string.
    pub coverUrl: Option<String>,
    /// The number of channels currently playing this type.
    pub online: Option<i32>
}
