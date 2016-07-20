use std::collections::BTreeMap;

/// A cover object. Currently missing the `type` property.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamCover {
    /// Metadata containing the cover's size.
    pub meta: BTreeMap<String, String>,
    /// The cover's identifier.
    pub id: i32,
    /// The identifier of the cover's channel.
    pub relid: Option<i32>,
    /// The cover's image URL string.
    pub url: String,
    /// Where the cover image is stored.
    pub store: String,
    /// The path at which the cover image can be found on Beam.
    pub remotePath: String,
    /// The date string, in UTC, at which the cover was created.
    pub createdAt: String,
    /// The date string, in UTC, at which the cover was last updated.
    pub updatedAt: String
}
