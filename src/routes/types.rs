extern crate rustc_serialize;
use rustc_serialize::json;

use ::request::{BeamRequest, HttpMethod};
use error::Error;

use ::models::channel::BeamChannel;
use ::models::btype::BeamType;

/// Result from a call returning multiple channels.
pub type BeamChannelsResult = Result<Vec<BeamChannel>, Error>;

/// Result from a call returning a type.
pub type BeamTypeResult = Result<BeamType, Error>;

/// Result from a call returning multiple types.
pub type BeamTypesResult = Result<Vec<BeamType>, Error>;

/// Routes that can be used to retrieve type data.
pub struct TypesRoutes {}

impl TypesRoutes {
    /// Creates a new types routes instance.
    pub fn new() -> Self {
        TypesRoutes {}
    }

    /// Retrieves channels currently playing a type.
    ///
    /// # Arguments
    ///
    /// * `id` The identifier of the type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.types.get_channels_by_type(127929);
    ///
    /// match res {
    ///     Ok(channels) => println!("There are {} channels playing this type.", channels.len()),
    ///     Err(_) => println!("error retrieving channels :(")
    /// }
    /// ```
    pub fn get_channels_by_type(&self, id: u32) -> BeamChannelsResult {
        let endpoint = String::from(format!("/types/{}/channels", id));
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

    /// Retrieves a type with the specified identifier.
    ///
    /// # Arguments
    ///
    /// * `id` The identifier of the type being retrieved.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.types.get_type(127929);
    ///
    /// match res {
    ///     Ok(btype) => println!("The name of this type is {}.", btype.name),
    ///     Err(_) => println!("error retrieving type :(")
    /// }
    /// ```
    pub fn get_type(&self, id: u32) -> BeamTypeResult {
        let endpoint = String::from(format!("/types?where=id.eq.{}", id));
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let mut decoded: Vec<BeamType> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                let btype = match decoded.pop() {
                    Some(btype) => btype,
                    None => return Err(Error::Unknown(String::from("couldn't remove type from array")))
                };

                return Ok(btype);
            },
            Err(_) => return Err(Error::Json)
        }
    }

    /// Retrieves types that are being played by at least one channel.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.types.get_types();
    ///
    /// match res {
    ///     Ok(types) => println!("{} types have been retrieved.", types.len()),
    ///     Err(_) => println!("error retrieving types :(")
    /// }
    /// ```
    pub fn get_types(&self) -> BeamTypesResult {
        let endpoint = String::from(format!("/types?where=online.neq.0"));
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: Vec<BeamType> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }

    /// Searches for types with a specified query.
    ///
    /// # Arguments
    ///
    /// * `query` The query being used to search for types.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use beam::Beam;
    /// let beam = Beam::new();
    /// let res = beam.types.get_types_by_query(String::from("Minecra"));
    ///
    /// match res {
    ///     Ok(types) => println!("{} types matched the query.", types.len()),
    ///     Err(_) => println!("error retrieving types :(")
    /// }
    /// ```
    pub fn get_types_by_query(&self, query: String) -> BeamTypesResult {
        let endpoint = String::from(format!("/types?query={}", query));
        match BeamRequest::request(endpoint, HttpMethod::Get) {
            Ok(ref raw_body) => {
                let decoded: Vec<BeamType> = match json::decode(raw_body) {
                    Ok(data) => data,
                    Err(err) => return Err(Error::Unknown(format!("{}", err)))
                };

                return Ok(decoded);
            },
            Err(_) => return Err(Error::Json)
        }
    }
}
