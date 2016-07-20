use std::collections::BTreeMap;

use ::models::emoticon::BeamEmoticon;

#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamEmoticonPack {
    /// The spritesheet's image URL string.
    pub url: String,
    /// A tree map containing the pack's emoticons.
    pub emoticons: BTreeMap<String, BeamEmoticon>
}
