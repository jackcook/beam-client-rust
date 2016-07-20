use ::models::channel::BeamChannel;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamUser {
    pub level: i32,
    // pub social: BTreeMap<String, ?>,
    pub id: i32,
    pub username: String,
    pub verified: bool,
    pub experience: i32,
    pub sparks: i32,
    pub avatarUrl: Option<String>,
    pub bio: Option<String>,
    pub primaryTeam: Option<i32>,
    pub createdAt: String,
    pub updatedAt: String,
    pub deletedAt: Option<String>,
    // pub groups: Vec<?>,
    pub channel: Option<Box<BeamChannel>>
}
