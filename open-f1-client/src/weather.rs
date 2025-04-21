use chrono::Utc;
use const_format::concatcp;
use open_f1_client_macros::api_request;
use reqwest::StatusCode;

use serde::{Deserialize, Serialize};

use crate::{BASE_URL, OpenF1ClientError};

const URL: &str = concatcp!(BASE_URL, "weather");

#[api_request]
#[derive(Debug, Default, Serialize)]
pub struct WeatherRequest {
    meeting_key: Option<usize>,
    session_key: Option<usize>,
    driver_number: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub air_temperature: f64,
    pub date: chrono::DateTime<Utc>,
    pub humidity: usize,
    pub meeting_key: usize,
    pub pressure: f64,
    pub rainfall: usize,
    pub session_key: usize,
    pub track_temperature: f64,
    pub wind_direction: usize,
    pub wind_speed: f64,
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_race_control() {
        let client = Client::new();
        let _ = client.weather().meeting_key(1208).send().await.unwrap();
    }
}
