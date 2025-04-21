use chrono::Utc;
use const_format::concatcp;
use open_f1_client_macros::client_request;
use reqwest::StatusCode;

use serde::{Deserialize, Serialize};

use crate::{BASE_URL, OpenF1ClientError};

const URL: &str = concatcp!(BASE_URL, "team_radio");

#[api_request]
#[derive(Debug, Default, Serialize)]
pub struct TeamRadioRequest {
    meeting_key: Option<usize>,
    session_key: Option<usize>,
    driver_number: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct TeamRadioResponse {
    pub date: chrono::DateTime<Utc>,
    pub driver_number: usize,
    pub meeting_key: usize,
    pub recording_url: String,
    pub session_key: usize,
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_race_control() {
        let client = Client::new();
        let _ = client
            .team_radio()
            .session_key(9158)
            .driver_number(11)
            .send()
            .await
            .unwrap();
    }
}
