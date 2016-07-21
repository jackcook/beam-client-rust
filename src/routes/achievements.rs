extern crate rustc_serialize;
use rustc_serialize::json;

use ::request::{BeamRequest, HttpMethod};
use error::Error;

use ::models::achievement::BeamAchievement;

/// Result from a call returning multiple achievements.
pub type BeamAchievementsResult = Result<Vec<BeamAchievement>, Error>;

/// Routes that can be used to interact with and retrieve achievement data.
pub struct AchievementsRoutes {}

impl AchievementsRoutes {
    /// Creates a new achievements routes instance.
    pub fn new() -> Self {
        AchievementsRoutes {}
    }

    /// Retrieves all achievements available on Beam.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.achievements.get_achievements();
    ///
    /// match res {
    ///     Ok(achievements) => println!("There are {} achievements available", achievements.len()),
    ///     Err(_) => println!("error retrieving achievements :(")
    /// }
    /// ```
    pub fn get_achievements(&self) -> BeamAchievementsResult {
        let endpoint = String::from("/achievements");
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: Vec<BeamAchievement> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => {
                        let err = format!("{}", err);
                        return Err(Error::Unknown(format!("{}", err)))
                    }
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }
}
