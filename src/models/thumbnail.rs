use std::collections::BTreeMap;

/// A thumbnail object. Currently missing the `type` property.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamThumbnail {
    /// Metadata containing the thumbnail's size.
    pub meta: BTreeMap<String, Vec<i32>>,
    /// The thumbnail's identifier.
    pub id: i32,
    /// The identifier of the thumbnail's channel.
    pub relid: i32,
    /// The thumbnail's image URL string.
    pub url: String,
    /// Where the thumbnail image is stored.
    pub store: String,
    /// The path at which the thumbnail image can be found on Beam.
    pub remotePath: String,
    /// The date string, in UTC, at which the thumbnail was created.
    pub createdAt: String,
    /// The date string, in UTC, at which the thumbnail was last updated.
    pub updatedAt: String
}
