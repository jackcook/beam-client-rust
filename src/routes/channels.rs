extern crate rustc_serialize;

use rustc_serialize::json;

use ::{Beam, HttpMethod};
use error::Error as Error;

use ::models::channel::BeamChannel;

pub type BeamChannelResult = Result<BeamChannel, Error>;

pub struct ChannelsRoutes {}

impl ChannelsRoutes {
    pub fn get_channel(&self, id: u32) -> BeamChannelResult {
        let endpoint = format!("/channels/{}", id);
        self.get_channel_by_endpoint(endpoint)
    }

    pub fn get_channel_with_token(&self, token: String) -> BeamChannelResult {
        let endpoint = format!("/channels/{}", token);
        self.get_channel_by_endpoint(endpoint)
    }

    fn get_channel_by_endpoint(&self, endpoint: String) -> BeamChannelResult {
        let beam = Beam::new();
        let res = beam.request(endpoint, HttpMethod::Get);

        match res {
            Ok(ref raw_body) => {
                let decoded: BeamChannel = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(_) => return Err(Error::Api(0, raw_body.to_string())),
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json),
        }
    }
}
