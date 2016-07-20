use std::collections::BTreeMap;

/// A badge object. Currently missing the `type` property.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamBadge {
    /// Metadata containing the badge's size.
    pub meta: BTreeMap<String, Vec<i32>>,
    /// The badge's identifier.
    pub id: i32,
    /// The identifier of the badge's channel.
    pub relid: i32,
    /// The badge's image URL string.
    pub url: String,
    /// Where the badge image is stored.
    pub store: String,
    /// The path at which the badge image can be found on Beam.
    pub remotePath: String,
    /// The date string, in UTC, at which the badge was created.
    pub createdAt: String,
    /// The date string, in UTC, at which the badge was last updated.
    pub updatedAt: String
}
