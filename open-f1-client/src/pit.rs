use chrono::Utc;
use const_format::concatcp;
use open_f1_client_macros::client_request;
use reqwest::StatusCode;

use crate::{BASE_URL, OpenF1ClientError};
use serde::{Deserialize, Serialize};

const URL: &str = concatcp!(BASE_URL, "pit");

#[client_request]
#[derive(Debug, Default, Serialize)]
pub struct PitRequest {
    meeting_key: Option<usize>,
    session_key: Option<usize>,
    driver_number: Option<usize>,
}

#[derive(Default, Deserialize, Debug)]
pub struct PitResponse {
    pub date: chrono::DateTime<Utc>,
    pub driver_number: usize,
    pub lap_number: usize,
    pub meeting_key: usize,
    pub pit_duration: f64,
    pub session_key: usize,
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_pit() {
        let client = Client::new();
        let _ = client
            .pit()
            .driver_number(63)
            .session_key(9158)
            .send()
            .await
            .unwrap();
    }
}
