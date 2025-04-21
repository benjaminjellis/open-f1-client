use const_format::concatcp;
use open_f1_client_macros::client_request;
use reqwest::StatusCode;

use crate::{BASE_URL, OpenF1ClientError};
use serde::{Deserialize, Serialize};

const URL: &str = concatcp!(BASE_URL, "drivers");

#[client_request]
#[derive(Debug, Default, Serialize)]
pub struct DriversRequest {
    session_key: Option<usize>,
    driver_number: Option<usize>,
}

#[derive(Default, Deserialize, Debug)]
pub struct DriversResponse {
    pub broadcast_name: String,
    pub country_code: Option<String>,
    pub driver_number: usize,
    pub first_name: Option<String>,
    pub full_name: Option<String>,
    pub headshot_url: Option<String>,
    pub last_name: Option<String>,
    pub meeting_key: usize,
    pub name_acronym: Option<String>,
    pub session_key: Option<usize>,
    pub team_colour: Option<String>,
    pub team_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[tokio::test]
    async fn test_drivers() {
        let client = Client::new();
        let _ = client
            .drivers()
            .driver_number(1)
            .session_key(9158)
            .send()
            .await
            .unwrap();
    }
}
