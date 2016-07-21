use std::collections::BTreeMap;

/// An ingest object.
#[allow(non_snake_case)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct BeamIngest {
    /// The name of the ingest's location.
    pub name: String,
    /// The host string of the ingest's URL.
    pub host: String,
    /// A WSS URL string that can be used to test the ingest's ping.
    pub pingTest: String,
    /// A list of protocols supported by the ingest.
    pub protocols: Vec<BTreeMap<String, String>>
}
