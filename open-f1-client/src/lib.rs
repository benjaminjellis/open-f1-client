mod error;
mod meetings;

pub use error::OpenF1ClientError;
#[derive(Clone, Default)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

const BASE_URL: &str = "https://api.openf1.org/v1/";

pub struct RequestBuilder<'a, T> {
    client: &'a reqwest::Client,
    request: T,
}

#[macro_export]
macro_rules! impl_client_for_request {
    ($method:ident, $request:ty) => {
        impl $crate::Client {
            pub fn $method(&self) -> $crate::RequestBuilder<'_, $request> {
                $crate::RequestBuilder {
                    client: &self.client,
                    request: <$request>::default(),
                }
            }
        }
    };
}
