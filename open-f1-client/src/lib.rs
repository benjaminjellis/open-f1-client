mod car_data;
mod drivers;
mod error;
mod laps;
mod location;
mod meetings;
mod pit;
mod position;
mod race_control;
mod sessions;
mod stints;
mod team_radio;
pub mod types;
mod weather;

pub use car_data::*;

pub use drivers::*;
pub use laps::*;
pub use location::*;
pub use meetings::*;
pub use pit::*;
pub use position::*;
pub use race_control::*;
pub use sessions::*;
pub use stints::*;
pub use team_radio::*;
pub use weather::*;

pub use error::OpenF1ClientError;

const BASE_URL: &str = "https://api.openf1.org/v1/";

#[derive(Clone, Default)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

pub struct RequestBuilder<'a, T> {
    client: &'a reqwest::Client,
    request: T,
}
