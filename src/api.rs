use self::rest::{ExchangeAPI, MarketAPI};

pub mod markets;

pub mod rest {
    use reqwest::IntoUrl;

    use self::models::{Allowance, Cursor, Exchange, Market, MarketSummary, Price, Trade};
    use serde::de::DeserializeOwned;

    #[derive(serde::Deserialize)]
    #[serde()]
    pub struct RESTResponse {
        pub allowance: Allowance,
        pub cursor: Option<Cursor>,
        pub error: Option<String>,
        pub result: Option<serde_json::Value>,
    }

    pub mod models {
        include!(concat!(env!("OUT_DIR"), "/cryptowatch.rest.models.rs"));
    }

    pub struct MarketAPI {
        base_url: &'static str,
    }

    #[derive(Debug)]
    pub struct MarketsResultPage {
        pub cursor: Option<Cursor>,
        pub markets: Vec<Market>,
    }

    /// Generic helper for HTTP GET and JSON response parsing
    async fn request<U: IntoUrl, T: DeserializeOwned>(url: U, ) -> Result<T, String> {
        let resp: RESTResponse = reqwest::get(url)
            .await
            .expect("Failed get request")
            .json()
            .await
            .expect("Failed to serialise response to JSON");
        if let Some(trades) = resp.result {
            Ok(serde_json::from_value(trades).expect("Unexpected response"))
        } else if let Some(error) = resp.error {
            Err(error.clone())
        } else {
            Err("No normal or error response available".into())
        }
    }


    impl MarketAPI {
        pub(crate) fn new(base_url: &'static str) -> Self {
            Self { base_url }
        }

        pub async fn list(&self, cursor: Option<Cursor>) -> Result<MarketsResultPage, String> {
            let url = if let Some(c) = cursor {
                if c.has_more {
                    format!("{}/markets?cursor={}", self.base_url, c.last)
                } else {
                    format!("{}/markets", self.base_url)
                }
            } else {
                format!("{}/markets", self.base_url)
            };
            let resp: RESTResponse = reqwest::get(url)
                .await
                .expect("Failed to get markets")
                .json()
                .await
                .expect("Failed to serialise markets response to JSON");
            if let Some(markets) = resp.result {
                Ok(MarketsResultPage {
                    cursor: resp.cursor,
                    markets: serde_json::from_value(markets).expect("Not a markets response"),
                })
            } else if let Some(error) = resp.error {
                Err(error.clone())
            } else {
                Err("No normal or error response available".into())
            }
        }

        pub async fn summary(&self, exchange: &str, pair: &str) -> Result<MarketSummary, String> {
            let url = format!("{}/markets/{}/{}/summary", self.base_url, exchange, pair);
            request(url).await
        }

        pub async fn details(&self, exchange: &str, pair: &str) -> Result<Market, String> {
            let url = format!("{}/markets/{}/{}", self.base_url, exchange, pair);
            request(url).await
        }

        pub async fn price(&self, exchange: &str, pair: &str) -> Result<Price, String> {
            let url = format!("{}/markets/{}/{}/price", self.base_url, exchange, pair);
            request(url).await
        }

        pub async fn trades(&self, exchange: &str, pair: &str) -> Result<Vec<Trade>, String> {
            let url = format!("{}/markets/{}/{}/trades", self.base_url, exchange, pair);
            request(url).await
        }
    }

    pub struct ExchangeAPI {
        base_url: &'static str,
    }

    impl ExchangeAPI {
        pub(crate) fn new(base_url: &'static str) -> Self {
            Self { base_url }
        }

        pub async fn list(&self) -> Result<Vec<Exchange>, String> {
            let url = format!("{}/exchanges", self.base_url);
            let resp: RESTResponse = reqwest::get(url)
                .await
                .expect("Failed to get exchanges")
                .json()
                .await
                .expect("");
            if let Some(exchanges) = resp.result {
                Ok(serde_json::from_value(exchanges).expect("Not an exchanges response"))
            } else if let Some(ref error) = resp.error {
                Err(error.clone())
            } else {
                Err("No normal or error response available".into())
            }
        }

        pub async fn detail(&self, name: &str) -> Result<Exchange, String> {
            let url = format!("{}/exchanges/{}", self.base_url, name);
            let resp: RESTResponse = reqwest::get(url)
                .await
                .expect("Failed to get exchange detail")
                .json()
                .await
                .expect("Couldn't serialise exchange response to JSON");
            if let Some(exchange) = resp.result {
                Ok(serde_json::from_value(exchange).expect("Not an exchange response"))
            } else if let Some(ref error) = resp.error {
                Err(error.clone())
            } else {
                Err("No normal or error response available".into())
            }
        }

        pub async fn markets(&self, name: &str) -> Result<Vec<Market>, String> {
            let url = format!("{}/markets/{}", self.base_url, name);
            let resp: RESTResponse = reqwest::get(url)
                .await
                .expect("Failed to get markets")
                .json()
                .await
                .expect("Failed to serialise markets response to JSON");
            if let Some(markets) = resp.result {
                Ok(serde_json::from_value(markets).expect("Not a markets response"))
            } else if let Some(error) = resp.error {
                Err(error.clone())
            } else {
                Err("No normal or error response available".into())
            }
        }
    }
}

#[async_trait::async_trait]
pub trait CryptowatchAPI {
    fn market(&self) -> MarketAPI;

    fn exchange(&self) -> ExchangeAPI;
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Auth {}

/// The main struct for accessing the Cryptowatch API
/// # Examples
/// ```
/// let cw = cryptowatch::api::Cryptowatch::default();
/// assert_eq!(cw.base_url, "https://api.cryptowat.ch");
/// assert_eq!(cw.authenticated(), false);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Cryptowatch {
    pub base_url: &'static str,
    authentication: Option<Auth>,
}

impl Cryptowatch {
    pub fn authenticated(&self) -> bool {
        self.authentication.is_some()
    }
}

impl Default for Cryptowatch {
    fn default() -> Self {
        Self {
            base_url: "https://api.cryptowat.ch",
            authentication: None,
        }
    }
}

#[async_trait::async_trait]
impl CryptowatchAPI for Cryptowatch {
    fn market(&self) -> MarketAPI {
        MarketAPI::new(self.base_url.clone())
    }

    fn exchange(&self) -> ExchangeAPI {
        ExchangeAPI::new(self.base_url.clone())
    }
}
