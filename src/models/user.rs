use ::models::channel::BeamChannel;

/// A user object. Currently missing the `social` and `groups` properties.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamUser {
    /// The user's spark level.
    pub level: i32,
    /// The user's identifier.
    pub id: i32,
    /// The user's name.
    pub username: String,
    /// True if the user has verified their email address.
    pub verified: bool,
    /// The number of experience points earned by the user.
    pub experience: i32,
    /// The number of sparks earned by the user.
    pub sparks: i32,
    /// The user's avatar image URL string.
    pub avatarUrl: Option<String>,
    /// A short biography written by the user.
    pub bio: Option<String>,
    /// The identifier of the user's primary team.
    pub primaryTeam: Option<i32>,
    /// The date string, in UTC, at which the user registered.
    pub createdAt: String,
    /// The date string, in UTC, at which the user last updated.
    pub updatedAt: String,
    /// The date string, in UTC, at which the user was deleted.
    pub deletedAt: Option<String>,
    /// The user's associated channel.
    pub channel: Option<Box<BeamChannel>>
}
