use chrono::Utc;
use const_format::concatcp;
use open_f1_client_macros::api_request;
use reqwest::StatusCode;

use crate::{BASE_URL, OpenF1ClientError};
use serde::{Deserialize, Serialize};

const URL: &str = concatcp!(BASE_URL, "position");

#[api_request]
#[derive(Debug, Default, Serialize)]
pub struct PositionRequest {
    meeting_key: Option<usize>,
    session_key: Option<usize>,
    driver_number: Option<usize>,
}

#[derive(Default, Deserialize, Debug)]
pub struct PositionResponse {
    pub date: chrono::DateTime<Utc>,
    pub driver_number: usize,
    pub meeting_key: usize,
    pub position: usize,
    pub session_key: usize,
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_position() {
        let client = Client::new();
        let _ = client
            .position()
            .meeting_key(1217)
            .driver_number(44)
            .send()
            .await
            .unwrap();
    }
}
