extern crate hyper;
extern crate rustc_serialize;

pub mod error;
pub mod models;
pub mod request;
pub mod routes;

use routes::achievements::AchievementsRoutes;
use routes::channels::ChannelsRoutes;
use routes::chats::ChatsRoutes;
use routes::ingests::IngestsRoutes;
use routes::recordings::RecordingsRoutes;
use routes::types::TypesRoutes;

/// The main class of the API client.
pub struct Beam {
    /// The property through which all achievement methods are accessed.
    pub achievements: AchievementsRoutes,
    /// The property through which all channel routes are accessed.
    pub channels: ChannelsRoutes,
    /// The property through which all chat routes are accessed.
    pub chats: ChatsRoutes,
    /// The property through which all ingest routes are accessed.
    pub ingests: IngestsRoutes,
    /// The property through which all recording routes are accessed.
    pub recordings: RecordingsRoutes,
    /// The property through which all type routes are accessed.
    pub types: TypesRoutes
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
            channels: ChannelsRoutes::new(),
            chats: ChatsRoutes::new(),
            ingests: IngestsRoutes::new(),
            recordings: RecordingsRoutes::new(),
            types: TypesRoutes::new()
        }
    }
}
