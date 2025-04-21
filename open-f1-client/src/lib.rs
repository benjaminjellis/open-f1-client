mod car_data;
mod error;
mod meetings;
mod sessions;

pub use meetings::*;

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
