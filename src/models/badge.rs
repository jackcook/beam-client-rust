use std::collections::BTreeMap;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamBadge {
    pub meta: BTreeMap<String, Vec<i32>>,
    pub id: i32,
    // pub type: String,
    pub relid: i32,
    pub url: String,
    pub store: String,
    pub remotePath: String,
    pub createdAt: String,
    pub updatedAt: String
}
