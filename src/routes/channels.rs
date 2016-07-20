extern crate rustc_serialize;

use rustc_serialize::json;

use ::request::{BeamRequest, HttpMethod};
use error::Error as Error;

use ::models::channel::BeamChannel;

/// Result from a call returning one channel.
pub type BeamChannelResult = Result<BeamChannel, Error>;

/// Result from a call returning multiple channels.
pub type BeamChannelsResult = Result<Vec<BeamChannel>, Error>;

/// The type of channels being requested.
pub enum ChannelsRequestType {
    /// All online channels, sorted by descending viewer count.
    All,
    /// All channels that support Tetris, sorted by descending viewer count.
    Interactive,
    /// Rising stars, channels that have some potential.
    Rising,
    /// Channels with streamers who are relatively new to Beam.
    Fresh
}

/// Routes that can be used to interact with and retrieve channel data.
pub struct ChannelsRoutes {}

impl ChannelsRoutes {
    /// Creates a new channels routes instance.
    pub fn new() -> Self {
        ChannelsRoutes {}
    }

    /// Retrieves a channel with the specified identifier.
    ///
    /// # Arguments
    ///
    /// * `id` The identifier of the channel being retrieved.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.channels.get_channel_with_id(252);
    ///
    /// match res {
    ///     Ok(channel) => println!("{} has {} viewers.", channel.token, channel.viewersCurrent),
    ///     Err(_) => println!("error retrieving channel :(")
    /// }
    /// ```
    pub fn get_channel_with_id(&self, id: u32) -> BeamChannelResult {
        let endpoint = format!("/channels/{}", id);
        self.get_channel_by_endpoint(endpoint)
    }

    /// Retrieves a channel with the specified token.
    ///
    /// # Arguments
    ///
    /// * `token` The token of the channel being retrieved.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.channels.get_channel_with_token(String::from("jack"));
    ///
    /// match res {
    ///     Ok(channel) => println!("{} has {} viewers.", channel.token, channel.viewersCurrent),
    ///     Err(_) => println!("error retrieving channel :(")
    /// }
    /// ```
    pub fn get_channel_with_token(&self, token: String) -> BeamChannelResult {
        let endpoint = format!("/channels/{}", token);
        self.get_channel_by_endpoint(endpoint)
    }

    fn get_channel_by_endpoint(&self, endpoint: String) -> BeamChannelResult {
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: BeamChannel = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => {
                        let error = format!("{}", err);
                        return Err(Error::Api(error, raw_body.to_string()));
                    }
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }

    /// Retrieves channels to be browsed with default parameters and pagination.
    ///
    /// # Arguments
    ///
    /// * `request_type` The type of channels to be retrieved.
    /// * `page` The page of channels to be retrieved.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// # use beam::models::channel::BeamChannel;
    /// # use beam::routes::channels::ChannelsRequestType;
    /// let beam = Beam::new();
    /// let res = beam.channels.get_channels(ChannelsRequestType::All, 0);
    ///
    /// match res {
    ///     Ok(channels) => println!("{} channels have >20 viewers.", channels.iter().filter(|&x| x.viewersCurrent > 20).collect::<Vec<&BeamChannel>>().len()),
    ///     Err(_) => println!("error retrieving channel :(")
    /// }
    /// ```
    pub fn get_channels(&self, request_type: ChannelsRequestType, page: i32) -> BeamChannelsResult {
        let endpoint = match request_type {
            ChannelsRequestType::All => "order=online:desc,viewersCurrent:desc,viewersTotal:desc&where=suspended.eq.0,online.eq.1",
            ChannelsRequestType::Interactive => "order=online:desc,viewersCurrent:desc,viewersTotal:desc&where=suspended.eq.0,online.eq.1,interactive.eq.1",
            ChannelsRequestType::Rising => "order=online:desc,rising&where=suspended.eq.0,online.eq.1",
            ChannelsRequestType::Fresh => "order=online:desc,fresh&where=suspended.eq.0,online.eq.1"
        };

        let endpoint = String::from(format!("/channels?{}&page={}", endpoint, page));

        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: Vec<BeamChannel> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => {
                        let error = format!("{}", err);
                        return Err(Error::Api(error, raw_body.to_string()));
                    }
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }
}
