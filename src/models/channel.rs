use ::models::badge::BeamBadge;
use ::models::cover::BeamCover;
use ::models::thumbnail::BeamThumbnail;
use ::models::btype::BeamType;
use ::models::user::BeamUser;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamChannel {
    pub id: i32,
    pub userId: i32,
    pub token: String,
    pub online: bool,
    pub featured: bool,
    pub partnered: bool,
    pub transcodingProfileId: Option<i32>,
    pub suspended: bool,
    pub name: String,
    pub audience: String,
    pub viewersTotal: i32,
    pub viewersCurrent: i32,
    pub numFollowers: i32,
    pub description: Option<String>,
    pub typeId: Option<i32>,
    pub interactive: bool,
    pub tetrisGameId: Option<i32>,
    pub ftl: i32,
    pub hasVod: bool,
    pub languageId: Option<i32>,
    pub coverId: Option<i32>,
    pub thumbnailId: Option<i32>,
    pub badgeId: Option<i32>,
    pub hosteeId: Option<i32>,
    pub createdAt: Option<String>,
    pub updatedAt: Option<String>,
    pub deletedAt: Option<String>,
    pub thumbnail: Option<BeamThumbnail>,
    pub cover: Option<BeamCover>,
    pub badge: Option<BeamBadge>,
    pub btype: Option<BeamType>,
    // pub cache: Vec<?>,
    // pub preferences: BTreeMap<String, ?>,
    pub user: Option<Box<BeamUser>>
}
