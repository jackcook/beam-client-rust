/// A chat user object.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamChatUser {
    /// The user's roles in the channel.
    pub userRoles: Vec<String>,
    /// The user's identifier.
    pub userId: i32,
    /// The user's name.
    pub userName: String
}
