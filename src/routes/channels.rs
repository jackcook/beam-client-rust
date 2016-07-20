extern crate rustc_serialize;

use rustc_serialize::json;

use ::{Beam, HttpMethod};
use error::Error as Error;

use ::models::channel::BeamChannel;

pub type BeamChannelResult = Result<BeamChannel, Error>;
pub type BeamChannelsResult = Result<Vec<BeamChannel>, Error>;

pub struct ChannelsRoutes<'a> {
    pub beam: &'a Beam
}

impl<'a> ChannelsRoutes<'a> {
    pub fn get_channel(&self, id: u32) -> BeamChannelResult {
        let endpoint = format!("/channels/{}", id);
        self.get_channel_by_endpoint(endpoint)
    }

    pub fn get_channel_with_token(&self, token: String) -> BeamChannelResult {
        let endpoint = format!("/channels/{}", token);
        self.get_channel_by_endpoint(endpoint)
    }

    fn get_channel_by_endpoint(&self, endpoint: String) -> BeamChannelResult {
        let res = self.beam.request(endpoint, HttpMethod::Get);

        match res {
            Ok(ref raw_body) => {
                let decoded: BeamChannel = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => {
                        let error = format!("{}", err);
                        return Err(Error::Api(error, raw_body.to_string()))
                    }
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json),
        }
    }

    pub fn get_channels(&self) -> BeamChannelsResult {
        let res = self.beam.request(String::from("/channels"), HttpMethod::Get);

        match res {
            Ok(ref raw_body) => {
                let decoded: Vec<BeamChannel> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => {
                        let error = format!("{}", err);
                        return Err(Error::Api(error, raw_body.to_string()))
                    }
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json),
        }
    }
}
