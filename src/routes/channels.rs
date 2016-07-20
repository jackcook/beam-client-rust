extern crate rustc_serialize;

use rustc_serialize::json;

use ::{Beam, HttpMethod};
use error::Error as Error;

use ::models::channel::BeamChannel;

pub type BeamChannelResult = Result<BeamChannel, Error>;
pub type BeamChannelsResult = Result<Vec<BeamChannel>, Error>;

pub enum ChannelsRequestType {
    All,
    Interactive,
    Rising,
    Fresh
}

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
        match self.beam.request(endpoint, HttpMethod::Get) {
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

    pub fn get_channels(&self, request_type: ChannelsRequestType, page: i32) -> BeamChannelsResult {
        let endpoint = match request_type {
            ChannelsRequestType::All => "order=online:desc,viewersCurrent:desc,viewersTotal:desc&where=suspended.eq.0,online.eq.1",
            ChannelsRequestType::Interactive => "order=online:desc,viewersCurrent:desc,viewersTotal:desc&where=suspended.eq.0,online.eq.1,interactive.eq.1",
            ChannelsRequestType::Rising => "order=online:desc,rising&where=suspended.eq.0,online.eq.1",
            ChannelsRequestType::Fresh => "order=online:desc,fresh&where=suspended.eq.0,online.eq.1"
        };

        let endpoint = String::from(format!("/channels?{}&page={}", endpoint, page));

        match self.beam.request(endpoint, HttpMethod::Get) {
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
