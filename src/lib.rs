extern crate hyper;
extern crate rustc_serialize;

pub mod error;
pub mod models;
pub mod request;
pub mod routes;

use routes::achievements::AchievementsRoutes;
use routes::channels::ChannelsRoutes;

/// The main class of the API client.
pub struct Beam {
    /// The property through which all achievement methods are accessed.
    pub achievements: AchievementsRoutes,
    /// The property through which all channel routes are accessed.
    pub channels: ChannelsRoutes
}

impl Beam {
    /// Creates a new Beam API client instance.
    ///
    /// # Example
    /// ```rust
    /// use beam::Beam;
    /// let beam = Beam::new();
    /// ```
    pub fn new() -> Self {
        Beam {
            achievements: AchievementsRoutes::new(),
            channels: ChannelsRoutes::new()
        }
    }
}
