extern crate rustc_serialize;
use rustc_serialize::json;

use ::request::{BeamRequest, HttpMethod};
use error::Error;

use ::models::user::BeamUser;

/// Result from a call returning a user.
pub type BeamUserResult = Result<BeamUser, Error>;

/// Routes that can be used to retrieve user data.
pub struct UsersRoutes {}

impl UsersRoutes {
    /// Creates a new users routes instance.
    pub fn new() -> Self {
        UsersRoutes {}
    }

    /// Retrieves a user with the specified identifier.
    ///
    /// # Arguments
    ///
    /// * `id` The identifier of the user.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.users.get_user_with_id(278);
    ///
    /// match res {
    ///     Ok(user) => println!("{} has {} sparks.", user.username, user.sparks),
    ///     Err(_) => println!("error retrieving user :(")
    /// }
    /// ```
    pub fn get_user_with_id(&self, id: u32) -> BeamUserResult {
        let endpoint = String::from(format!("/users/{}", id));
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: BeamUser = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }
}
