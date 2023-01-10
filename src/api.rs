use self::rest::{models::{Market, Exchange}, RESTResponse};

pub mod markets;

pub mod rest {
    use self::models::Allowance;


    #[derive(serde::Deserialize)]
    #[serde()]
    pub struct RESTResponse {
        pub error: Option<String>,
        pub allowance: Allowance,
        pub result: Option<serde_json::Value>,
    }

    pub mod models {
        include!(concat!(env!("OUT_DIR"), "/cryptowatch.rest.models.rs"));
    }
}

#[async_trait::async_trait]
pub trait CryptowatchApi {
    /// List the markets available
    async fn markets(&self) -> Result<Vec<Market>, String>;
    //async fn market(self, name: &str) -> Result<Vec<Market>, &str>;

    /// List all supported exchanges
    async fn exchanges(&self) -> Result<Vec<Exchange>, String>;

    /// Get the exchange details
    async fn exchange(&self, name: &str) -> Result<Exchange, String>;
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
impl CryptowatchApi for Cryptowatch {
    async fn markets(&self) -> Result<Vec<Market>, String> {
        let url = format!("{}/markets", self.base_url);
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

    async fn exchanges(&self) -> Result<Vec<Exchange>, String> {
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

    async fn exchange(&self, name: &str) -> Result<Exchange, String> {
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
}
