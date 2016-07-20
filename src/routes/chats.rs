extern crate rustc_serialize;

use rustc_serialize::json;

use ::request::{BeamRequest, HttpMethod};
use error::Error;

use ::models::chat_details::BeamChatDetails;
use ::models::chat_user::BeamChatUser;

/// Result from a call returning chat details.
pub type BeamChatDetailsResult = Result<BeamChatDetails, Error>;

/// Result from a call returning a chat user.
pub type BeamChatUserResult = Result<BeamChatUser, Error>;

/// Result from a call returning chat users.
pub type BeamChatUsersResult = Result<Vec<BeamChatUser>, Error>;

/// Routes that can be used to interact with and retrieve chat data.
pub struct ChatsRoutes {}

impl ChatsRoutes {
    /// Creates a new chats routes instance.
    pub fn new() -> Self {
        ChatsRoutes {}
    }

    /// Retrieves chat connection details for a specified channel.
    ///
    /// # Arguments
    ///
    /// * `id` The identifier of the channel.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.chats.get_chat_details(252);
    ///
    /// match res {
    ///     Ok(chat_details) => println!("There are {} endpoints available.", chat_details.endpoints.len()),
    ///     Err(_) => println!("error retrieving chat details :(")
    /// }
    /// ```
    pub fn get_chat_details(&self, id: u32) -> BeamChatDetailsResult {
        let endpoint = String::from(format!("/chats/{}", id));
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: BeamChatDetails = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }

    /// Retrieves a user who is currently participating in a channel's chat.
    ///
    /// # Arguments
    ///
    /// * `channel_id` The identifier of the channel.
    /// * `user_id` The identifier of the user.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.chats.get_chat_user(252, 278);
    ///
    /// match res {
    ///     Ok(chat_user) => println!("{} has {} roles in the chat.", chat_user.userName, chat_user.userRoles.len()),
    ///     Err(_) => println!("error retrieving chat user :(")
    /// }
    /// ```
    pub fn get_chat_user(&self, channel_id: u32, user_id: u32) -> BeamChatUserResult {
        let endpoint = String::from(format!("/chats/{}/users/{}", channel_id, user_id));
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: BeamChatUser = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }

    /// Retrieves users who are currently participating in a channel's chat.
    ///
    /// # Arguments
    ///
    /// * `id` The identifier of the channel.
    /// * `page` The page of users to be retrieved.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.chats.get_chat_users(252, 0);
    ///
    /// match res {
    ///     Ok(chat_users) => println!("There are {} people participating in the chat.", chat_users.len()),
    ///     Err(_) => println!("error retrieving chat users :(")
    /// }
    /// ```
    pub fn get_chat_users(&self, id: u32, page: u32) -> BeamChatUsersResult {
        let endpoint = String::from(format!("/chats/{}/users?page={}", id, page));
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: Vec<BeamChatUser> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }

    /// Searches for users who are currently participating in a channel's chat.
    ///
    /// # Arguments
    ///
    /// * `id` The identifier of the channel.
    /// * `query` The query used to search for usernames.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.chats.search_chat_users(252, String::from("aaaa"));
    ///
    /// match res {
    ///     Ok(chat_users) => println!("There are {} users who match this query.", chat_users.len()),
    ///     Err(_) => println!("error searching chat users :(")
    /// }
    /// ```
    pub fn search_chat_users(&self, id: u32, query: String) -> BeamChatUsersResult {
        let endpoint = String::from(format!("/chats/{}/users/search?username={}", id, query));
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: Vec<BeamChatUser> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }
}
