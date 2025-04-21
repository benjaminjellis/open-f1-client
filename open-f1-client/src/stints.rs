use const_format::concatcp;
use open_f1_client_macros::api_request;
use reqwest::StatusCode;

use serde::{Deserialize, Serialize};

use crate::{BASE_URL, OpenF1ClientError, types::TyreCompound};

const URL: &str = concatcp!(BASE_URL, "stints");

#[api_request]
#[derive(Debug, Default, Serialize)]
pub struct StintsRequest {
    meeting_key: Option<usize>,
    session_key: Option<usize>,
    driver_number: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct StintsResponse {
    pub compound: Option<TyreCompound>,
    pub driver_number: Option<usize>,
    pub lap_end: usize,
    pub lap_start: usize,
    pub meeting_key: usize,
    pub session_key: usize,
    pub stint_number: usize,
    pub tyre_age_at_start: Option<usize>,
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_race_control() {
        let client = Client::new();
        let _ = client.stints().send().await.unwrap();
    }
}
