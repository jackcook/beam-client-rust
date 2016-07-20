extern crate rustc_serialize;

use rustc_serialize::json;

use ::request::{BeamRequest, HttpMethod};
use error::Error;

use ::models::channel::BeamChannel;
use ::models::emoticon_pack::BeamEmoticonPack;
use ::models::recording::BeamRecording;
use ::models::user::BeamUser;

/// Result from a call returning one channel.
pub type BeamChannelResult = Result<BeamChannel, Error>;

/// Result from a call returning multiple channels.
pub type BeamChannelsResult = Result<Vec<BeamChannel>, Error>;

/// Result from a call returning a channel's followers.
pub type BeamFollowersResult = Result<Vec<BeamUser>, Error>;

/// Result from a call returning a channel's emoticon pack.
pub type BeamEmoticonPackResult = Result<BeamEmoticonPack, Error>;

/// Result from a call returning a channel's recordings.
pub type BeamRecordingsResult = Result<Vec<BeamRecording>, Error>;

/// Result from a call returning a channel's viewers.
pub type BeamViewersResult = Result<Vec<BeamUser>, Error>;

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
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
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
    pub fn get_channels(&self, request_type: ChannelsRequestType, page: u32) -> BeamChannelsResult {
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
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }

    /// Retrieves the followers of a channel, paginated.
    ///
    /// # Arguments
    ///
    /// * `id` The id of the channel followers are being retrieved from.
    /// * `page` The page in the range [0, inf] of followers being requested.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.channels.get_followers_of_channel(252, 0);
    ///
    /// match res {
    ///     Ok(followers) => println!("jack has {} followers.", followers.len()),
    ///     Err(_) => println!("error retrieving followers :(")
    /// }
    /// ```
    pub fn get_followers_of_channel(&self, id: u32, page: u32) -> BeamFollowersResult {
        let endpoint = String::from(format!("/channels/{}/follow?page={}", id, page));
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: Vec<BeamUser> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }

    /// Retrieves the emoticons of a channel, paginated.
    ///
    /// # Arguments
    ///
    /// * `id` The id of the channel emoticons are being retrieved from.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.channels.get_emoticons_of_channel(252);
    ///
    /// match res {
    ///     Ok(pack) => println!("tlovetech's emoticon spritesheet can be found at {}.", pack.url),
    ///     Err(_) => println!("error retrieving emoticons :(")
    /// }
    /// ```
    pub fn get_emoticons_of_channel(&self, id: u32) -> BeamEmoticonPackResult {
        let endpoint = String::from(format!("/channels/{}/emoticons", id));
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: BeamEmoticonPack = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }

    /// Retrieves the recordings of a channel.
    ///
    /// # Arguments
    ///
    /// * `id` The id of the channel recordings are being retrieved from.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.channels.get_recordings_of_channel(3181);
    ///
    /// match res {
    ///     Ok(recordings) => println!("tlovetech has {} available recordings.", recordings.len()),
    ///     Err(_) => println!("error retrieving recordings :(")
    /// }
    /// ```
    pub fn get_recordings_of_channel(&self, id: u32) -> BeamRecordingsResult {
        let endpoint = String::from(format!("/channels/{}/recordings", id));
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: Vec<BeamRecording> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }

    /// Retrieves the users currently watching a channel.
    ///
    /// # Arguments
    ///
    /// * `id` The id of the channel viewers are being retrieved from.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.channels.get_viewers_of_channel(3181, 0);
    ///
    /// match res {
    ///     Ok(viewers) => println!("{} people are currently watching tlovetech.", viewers.len()),
    ///     Err(_) => println!("error retrieving viewers :(")
    /// }
    /// ```
    pub fn get_viewers_of_channel(&self, id: u32, page: u32) -> BeamViewersResult {
        let endpoint = String::from(format!("/channels/{}/users?page={}", id, page));
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: Vec<BeamUser> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }
}
