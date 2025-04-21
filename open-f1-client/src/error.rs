use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenF1ClientError {
    #[error("Encountered some error for endpoint: {endpoint} - {source}")]
    Request {
        endpoint: &'static str,
        source: reqwest::Error,
    },
    #[error("Encountered status code: {status_code} for endpoint: {endpoint} - {body}")]
    Non200StatusCode {
        status_code: reqwest::StatusCode,
        body: String,
        endpoint: &'static str,
    },

    #[error("Encountered dezerialization error for endpoin: {endpoint} - {source}")]
    Dezerialization {
        endpoint: &'static str,
        source: reqwest::Error,
    },
}
