use std::collections::BTreeMap;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamCover {
    pub meta: BTreeMap<String, String>,
    pub id: i32,
    // pub type: String,
    pub relid: Option<i32>,
    pub url: String,
    pub store: String,
    pub remotePath: String,
    pub createdAt: String,
    pub updatedAt: String
}
