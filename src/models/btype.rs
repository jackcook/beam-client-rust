#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamType {
    pub id: i32,
    pub name: String,
    pub parent: String,
    pub description: String,
    pub source: String,
    pub viewersCurrent: i32,
    pub coverUrl: String,
    pub online: i32
}
