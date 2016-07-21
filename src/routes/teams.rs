extern crate rustc_serialize;
use rustc_serialize::json;

use ::request::{BeamRequest, HttpMethod};
use error::Error;

use ::models::team::BeamTeam;
use ::models::user::BeamUser;

/// Result from a call returning a stream team.
pub type BeamTeamResult = Result<BeamTeam, Error>;

/// Result from a call returning multiple stream teams.
pub type BeamTeamsResult = Result<Vec<BeamTeam>, Error>;

/// Result from a call returning members of a team.
pub type BeamUsersResult = Result<Vec<BeamUser>, Error>;

/// Routest that can be used to retrieve team data.
pub struct TeamsRoutes {}

impl TeamsRoutes {
    /// Creates a new teams routes instance.
    pub fn new() -> Self {
        TeamsRoutes {}
    }

    /// Retrieves a team with the specified identifier.
    ///
    /// # Arguments
    ///
    /// * `id` The identifier of the team being retrieved.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.teams.get_team_with_id(111);
    ///
    /// match res {
    ///     Ok(team) => println!("{} has {} viewers.", team.name, team.totalViewersCurrent),
    ///     Err(_) => println!("error retrieving team :(")
    /// }
    /// ```
    pub fn get_team_with_id(&self, id: u32) -> BeamTeamResult {
        let endpoint = format!("/teams/{}", id);
        self.get_team_by_endpoint(endpoint)
    }

    /// Retrieves a team with the specified token.
    ///
    /// # Arguments
    ///
    /// * `token` The token of the team being retrieved.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.teams.get_team_with_token(String::from("partners"));
    ///
    /// match res {
    ///     Ok(team) => println!("{} has {} viewers.", team.name, team.totalViewersCurrent),
    ///     Err(_) => println!("error retrieving team :(")
    /// }
    /// ```
    pub fn get_team_with_token(&self, token: String) -> BeamTeamResult {
        let endpoint = format!("/teams/{}", token);
        self.get_team_by_endpoint(endpoint)
    }

    fn get_team_by_endpoint(&self, endpoint: String) -> BeamTeamResult {
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: BeamTeam = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }

    /// Retrieves all stream teams, paginated and descending by viewer count.
    ///
    /// # Arguments
    ///
    /// * `page` The page of teams to retrieve.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// # use beam::models::team::BeamTeam;
    /// let beam = Beam::new();
    /// let res = beam.teams.get_teams();
    ///
    /// match res {
    ///     Ok(teams) => println!("{} teams have >20 viewers.", teams.iter().filter(|&x| x.totalViewersCurrent > 20).collect::<Vec<&BeamTeam>>().len()),
    ///     Err(_) => println!("error retrieving teams :(")
    /// }
    /// ```
    pub fn get_teams(&self) -> BeamTeamsResult {
        let endpoint = String::from("/teams?order=totalViewersCurrent:desc");
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: Vec<BeamTeam> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }

    /// Retrieves the members of a team.
    ///
    /// # Arguments
    ///
    /// * `id` The identifier of the team.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.teams.get_members_of_team(111);
    ///
    /// match res {
    ///     Ok(users) => println!("There are {} people on this team.", users.len()),
    ///     Err(_) => println!("error retrieving members :(")
    /// }
    /// ```
    pub fn get_members_of_team(&self, id: u32) -> BeamUsersResult {
        let endpoint = String::from(format!("/teams/{}/users", id));
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: Vec<BeamUser> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }
}
