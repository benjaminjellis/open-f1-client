use chrono::Utc;
use const_format::concatcp;
use open_f1_client_macros::api_request;
use reqwest::StatusCode;

use crate::{BASE_URL, OpenF1ClientError};
use serde::{Deserialize, Serialize};

const URL: &str = concatcp!(BASE_URL, "car_data");

#[api_request]
#[derive(Debug, Default, Serialize)]
pub struct CarDataRequest {
    meeting_key: Option<usize>,
    session_key: Option<usize>,
    driver_number: Option<usize>,
}

#[derive(Default, Deserialize, Debug)]
pub struct CarDataResponse {
    pub brake: usize,
    pub date: chrono::DateTime<Utc>,
    pub driver_number: usize,
    pub drs: usize,
    pub meeting_key: usize,
    pub n_gear: usize,
    pub rpm: usize,
    pub session_key: usize,
    pub speed: usize,
    pub throttle: usize,
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_car_data() {
        let client = Client::new();
        let _ = client
            .car_data()
            .driver_number(55)
            .session_key(9159)
            .send()
            .await
            .unwrap();
    }
}
