use chrono::Utc;
use const_format::concatcp;
use open_f1_client_macros::client_request;
use reqwest::StatusCode;

use crate::{BASE_URL, OpenF1ClientError};
use serde::{Deserialize, Serialize};

const URL: &str = concatcp!(BASE_URL, "sessions");

#[api_request]
#[derive(Default, Serialize)]
pub struct SessionsRequest {
    country_name: Option<String>,
    country_code: Option<String>,
    session_name: Option<String>,
    meeting_key: Option<usize>,
    year: Option<u16>,
}

#[derive(Default, Deserialize, Debug)]
pub struct SessionsResponse {
    pub circuit_key: usize,
    pub circuit_short_name: String,
    pub country_code: String,
    pub country_key: usize,
    pub country_name: String,
    pub date_end: chrono::DateTime<Utc>,
    pub date_start: chrono::DateTime<Utc>,
    pub gmt_offset: String,
    pub location: String,
    pub meeting_key: usize,
    pub session_key: usize,
    pub session_name: String,
    pub session_type: String,
    pub year: u16,
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_meetings() {
        let client = Client::new();
        let _ = client
            .sessions()
            .country_name("Belgium".into())
            .session_name("Sprint".into())
            .year(2023)
            .send()
            .await
            .unwrap();
    }
}
