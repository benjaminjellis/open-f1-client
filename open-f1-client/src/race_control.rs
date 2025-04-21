use chrono::Utc;
use const_format::concatcp;
use open_f1_client_macros::client_request;
use reqwest::StatusCode;

use serde::{Deserialize, Serialize};

use crate::{
    BASE_URL, OpenF1ClientError,
    types::{Flag, RaceControlEvent, RaceControlEventScope},
};

const URL: &str = concatcp!(BASE_URL, "race_control");

#[client_request]
#[derive(Debug, Default, Serialize)]
pub struct RaceControlRequest {
    meeting_key: Option<usize>,
    session_key: Option<usize>,
    driver_number: Option<usize>,
    flatg: Option<Flag>,
}

#[derive(Deserialize, Debug)]
pub struct RaceControlResponse {
    pub category: RaceControlEvent,
    pub date: chrono::DateTime<Utc>,
    pub driver_number: Option<usize>,
    pub flag: Option<Flag>,
    pub lap_number: Option<usize>,
    pub meeting_key: usize,
    pub message: String,
    pub scope: Option<RaceControlEventScope>,
    pub sector: Option<usize>,
    pub session_key: usize,
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_race_control() {
        let client = Client::new();
        let _ = client
            .race_control()
            .meeting_key(1217)
            .send()
            .await
            .unwrap();
    }
}
