use ::models::badge::BeamBadge;
use ::models::cover::BeamCover;
use ::models::thumbnail::BeamThumbnail;
use ::models::user::BeamUser;

/// A channel object. Currently missing the `type`, `cache` and `preferences` properties.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamChannel {
    /// The channel's identifier.
    pub id: i32,
    /// The identifier of the channel's user.
    pub userId: i32,
    /// The channel's token/username.
    pub token: String,
    /// True if the channel is online.
    pub online: bool,
    /// True if the channel is featured by Beam.
    pub featured: bool,
    /// True if the channel is partnered with Beam.
    pub partnered: bool,
    /// The identifier of the channel's transcoding profile.
    pub transcodingProfileId: Option<i32>,
    /// True if the channel is suspended.
    pub suspended: bool,
    /// The name of the channel's stream.
    pub name: String,
    /// A one-word description of the channel's recommended audience (e.g. teen).
    pub audience: String,
    /// The total number of viewers the channel has ever had.
    pub viewersTotal: i32,
    /// The current number of viewers watching the channel.
    pub viewersCurrent: i32,
    /// The channel's number of followers.
    pub numFollowers: i32,
    /// A description, in HTML, filled out by the channel's owner.
    pub description: Option<String>,
    /// The identifier of the type being played by the channel.
    pub typeId: Option<i32>,
    /// True if the channel's content is compatible with Tetris.
    pub interactive: bool,
    /// The identifier of the Tetris game being played by the channel.
    pub tetrisGameId: Option<i32>,
    /// FTL is enabled if this value is >0.
    pub ftl: i32,
    /// True if this channel has a stored VoD.
    pub hasVod: bool,
    /// The identifier of the language used by the channel.
    pub languageId: Option<i32>,
    /// The identifier of the channel's cover image.
    pub coverId: Option<i32>,
    /// The identifier of the channel's thumbnail.
    pub thumbnailId: Option<i32>,
    /// The identifier of the channel's badge.
    pub badgeId: Option<i32>,
    /// The identifier of a channel being hosted by the channel.
    pub hosteeId: Option<i32>,
    /// The date string, in UTC, on which the channel was created.
    pub createdAt: Option<String>,
    /// The date string, in UTC, on which the channel was last updated.
    pub updatedAt: Option<String>,
    /// The date string, in UTC, on which the channel was deleted.
    pub deletedAt: Option<String>,
    /// The channel's thumbnail.
    pub thumbnail: Option<BeamThumbnail>,
    /// The channel's cover image.
    pub cover: Option<BeamCover>,
    /// The channel's chat badge.
    pub badge: Option<BeamBadge>,
    /// The channel's user.
    pub user: Option<Box<BeamUser>>
}
