use chrono::Utc;
use const_format::concatcp;
use open_f1_client_macros::api_request;
use reqwest::StatusCode;

use crate::{BASE_URL, OpenF1ClientError};
use serde::{Deserialize, Serialize};

const URL: &str = concatcp!(BASE_URL, "laps");

#[api_request]
#[derive(Debug, Default, Serialize)]
pub struct LapsRequest {
    session_key: Option<usize>,
    driver_number: Option<usize>,
    lap_number: Option<usize>,
}

#[derive(Default, Deserialize, Debug)]
pub struct LapsResponse {
    pub date_start: chrono::DateTime<Utc>,
    pub driver_number: usize,
    pub duration_sector_1: f64,
    pub duration_sector_2: f64,
    pub duration_sector_3: f64,
    pub i1_speed: usize,
    pub i2_speed: usize,
    pub is_pit_out_lap: bool,
    pub lap_duration: f64,
    pub lap_number: u16,
    pub meeting_key: usize,
    pub segments_sector_1: Vec<f64>,
    pub segments_sector_2: Vec<f64>,
    pub segments_sector_3: Vec<f64>,
    pub session_key: usize,
    pub st_speed: usize,
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_laps() {
        let client = Client::new();
        let _ = client
            .laps()
            .driver_number(63)
            .session_key(9161)
            .lap_number(8)
            .send()
            .await
            .unwrap();
    }
}
