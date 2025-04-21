use chrono::Utc;
use const_format::concatcp;
use open_f1_client_macros::client_request;
use reqwest::StatusCode;

use crate::{BASE_URL, OpenF1ClientError};
use serde::{Deserialize, Serialize};

const URL: &str = concatcp!(BASE_URL, "location");

#[client_request]
#[derive(Debug, Default, Serialize)]
pub struct LocationRequest {
    meeting_key: Option<usize>,
    session_key: Option<usize>,
    driver_number: Option<usize>,
}

#[derive(Default, Deserialize, Debug)]
pub struct LocationResponse {
    pub date: chrono::DateTime<Utc>,
    pub driver_number: usize,
    pub meeting_key: usize,
    pub session_key: usize,
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_car_data() {
        let client = Client::new();
        let _ = client
            .location()
            .driver_number(81)
            .session_key(9161)
            .send()
            .await
            .unwrap();
    }
}
