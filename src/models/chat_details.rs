/// An object containing details about connecting to chat.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamChatDetails {
    /// WSS URL strings that are available for connecting to.
    pub endpoints: Vec<String>
}
