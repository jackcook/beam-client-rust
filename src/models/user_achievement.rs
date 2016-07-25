use ::models::achievement::BeamAchievement;

/// A user achievement object.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamUserAchievement {
    /// The identifier of this user's achievement.
    pub id: i32,
    /// True if the user has earned this achievement.
    pub earned: bool,
    /// How much progress has been made in earning this achievement, {0 <= x <= 1}.
    pub progress: f32,
    /// The date string, in UTC, on which the user began to earn this achievement.
    pub createdAt: String,
    /// The date string, in UTC, on which the user last made progress on this achievement.
    pub updatedAt: String,
    /// The achievement's slug/identifier.
    pub achievement: String,
    /// The identifier of the user earning this achievement.
    pub user: i32,
    /// The achievement being earned.
    pub Achievement: BeamAchievement
}
