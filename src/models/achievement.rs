/// An achievement object.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamAchievement {
    /// The achievement's slug/identifier.
    pub slug: String,
    /// The name of the achievement.
    pub name: String,
    /// A short description of the achievement.
    pub description: String,
    /// The number of sparks earned for this achievement.
    pub sparks: i32
}
