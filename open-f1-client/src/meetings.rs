use chrono::Utc;
use const_format::concatcp;
use open_f1_client_macros::client_request;
use reqwest::StatusCode;

use crate::{BASE_URL, OpenF1ClientError};
use serde::{Deserialize, Serialize};

const URL: &str = concatcp!(BASE_URL, "meetings");

#[api_request]
#[derive(Default, Serialize)]
pub struct MeetingsRequest {
    year: Option<u8>,
    country_name: Option<String>,
}

#[derive(Default, Deserialize)]
pub struct MeetingsResponse {
    pub circuit_key: usize,
    pub circuit_short_name: String,
    pub country_code: String,
    pub country_key: usize,
    pub country_name: String,
    pub date_start: chrono::DateTime<Utc>,
    pub gmt_offset: String,
    pub location: String,
    pub meeting_key: usize,
    pub meeting_name: String,
    pub meeting_official_name: String,
    pub year: u8,
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_meetings() {
        let client = Client::new();
        let _ = client.meetings().year(10).send().await.unwrap();
    }
}
