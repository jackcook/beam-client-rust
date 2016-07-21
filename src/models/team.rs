use std::collections::BTreeMap;

/// A team object.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamTeam {
    /// The team's social media profile URLs.
    pub social: Option<BTreeMap<String, String>>,
    /// The team's identifier.
    pub id: i32,
    /// The identifier of the user who owns the team.
    pub ownerId: i32,
    /// The team's token.
    pub token: String,
    /// The name of the team.
    pub name: String,
    /// An HTML string representing the team's description.
    pub description: Option<String>,
    /// The URL string of the team's logo image.
    pub logoUrl: Option<String>,
    /// The URL string of the team's background image.
    pub backgroundUrl: Option<String>,
    /// The number of people watching channels on this stream team.
    pub totalViewersCurrent: i32
}
