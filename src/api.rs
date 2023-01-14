use self::rest::{ExchangeAPI, MarketAPI, PairsAPI};

pub mod markets;

pub mod rest {
    use std::collections::HashMap;

    use async_trait::async_trait;
    use reqwest::IntoUrl;

    use self::models::{
        Allowance, Candle, Cursor, Exchange, Market, MarketSummary, Orderbook, Pair, Price, Trade,
    };
    use serde::de::DeserializeOwned;

    static CW_BASE_URL: &str = "https://api.cryptowat.ch";

    pub mod models {

        include!(concat!(env!("OUT_DIR"), "/cryptowatch.rest.models.rs"));

        impl Orderbook {
            /// ```
            /// use cryptowatch::api::rest::models::{Level, Orderbook};
            /// let ob = Orderbook {
            ///     seq_num: 0,
            ///     asks: vec![Level { price: 1000.,  amount: 1. }],
            ///     bids: vec![Level { price: 999.,   amount:   1. }]
            /// };
            /// assert_eq!(ob.spread(), Some(1.));
            pub fn spread(&self) -> Option<f32> {
                let ask = self.top_ask().map(|a| a.price)?;
                let bid = self.top_bid().map(|a| a.price)?;

                Some(ask - bid)
            }

            /// The ask.
            /// ```
            /// use cryptowatch::api::rest::models::Orderbook;
            /// let ob = Orderbook { seq_num: 0, asks: vec![], bids: vec![] };
            /// assert!(ob.top_ask().is_none());
            /// ```
            pub fn top_ask(&self) -> Option<Level> {
                self.asks.get(0).cloned()
            }

            /// The bid.
            /// ```
            /// use cryptowatch::api::rest::models::Orderbook;
            /// let ob = Orderbook { seq_num: 0, asks: vec![], bids: vec![] };
            /// assert!(ob.top_bid().is_none());
            /// ```
            pub fn top_bid(&self) -> Option<Level> {
                self.bids.get(0).cloned()
            }
        }
    }

    pub trait HasURI {
        fn uri(&self) -> String;
    }

    #[async_trait::async_trait]
    pub trait List<T>: HasURI
    where
        T: DeserializeOwned,
    {
        /// Return a paged collection of results, optionally starting from the given cursor
        async fn list(&self, cursor: Option<Cursor>) -> Result<PagedResult<Vec<T>>, String> {
            let url = if let Some(c) = cursor {
                format!("{}/{}?cursor={}", CW_BASE_URL, self.uri(), c.last)
            } else {
                format!("{}/{}", CW_BASE_URL, self.uri())
            };

            let resp = http_get(url).await?;

            Ok(PagedResult {
                cursor: resp.cursor.clone(),
                result: convert_response(resp)?,
            })
        }
    }

    #[async_trait::async_trait]
    pub trait Details<T>: HasURI
    where
        T: DeserializeOwned,
    {
        async fn details(&self, value: &str) -> Result<T, String> {
            request(format!("{}/{}/{}", CW_BASE_URL, self.uri(), value)).await
        }
    }

    #[derive(serde::Deserialize)]
    #[serde()]
    pub struct RESTResponse {
        pub allowance: Allowance,
        pub cursor: Option<Cursor>,
        pub error: Option<String>,
        pub result: Option<serde_json::Value>,
    }

    pub struct PagedResult<T> {
        pub cursor: Option<Cursor>,
        pub result: T,
    }

    pub struct AssetsAPI {}

    /// A wrapper for the Pair resource and its operations
    pub struct PairsAPI {}

    /// A wrapper for the Market resource and its operations
    pub struct MarketAPI {}

    impl HasURI for PairsAPI {
        fn uri(&self) -> String {
            "pairs".to_string()
        }
    }

    impl List<Pair> for PairsAPI {}

    impl Details<Pair> for PairsAPI {}

    impl HasURI for MarketAPI {
        fn uri(&self) -> String {
            "markets".to_string()
        }
    }

    impl List<Market> for MarketAPI {}

    impl Details<Market> for MarketAPI {}

    #[derive(Debug)]
    pub struct MarketsResultPage {
        pub cursor: Option<Cursor>,
        pub markets: Vec<Market>,
    }

    #[derive(Debug)]
    pub struct PricesResultPage {
        pub cursor: Option<Cursor>,
        pub prices: HashMap<String, f32>,
    }

    impl MarketAPI {
        pub(crate) fn new() -> Self {
            Self {}
        }

        pub async fn summary(&self, exchange: &str, pair: &str) -> Result<MarketSummary, String> {
            let url = format!("{}/markets/{}/{}/summary", CW_BASE_URL, exchange, pair);
            request(url).await
        }

        pub async fn details(&self, exchange: &str, pair: &str) -> Result<Market, String> {
            let url = format!("{}/markets/{}/{}", CW_BASE_URL, exchange, pair);
            request(url).await
        }

        pub async fn price(&self, exchange: &str, pair: &str) -> Result<Price, String> {
            let url = format!("{}/markets/{}/{}/price", CW_BASE_URL, exchange, pair);
            request(url).await
        }

        pub async fn prices(&self, cursor: Option<Cursor>) -> Result<PricesResultPage, String> {
            let url = if let Some(c) = cursor {
                format!("{}/markets/prices?cursor={}", CW_BASE_URL, c.last)
            } else {
                format!("{}/markets/prices", CW_BASE_URL,)
            };

            let resp = http_get(url).await?;

            Ok(PricesResultPage {
                cursor: resp.cursor.clone(),
                prices: convert_response(resp)?,
            })
        }

        pub async fn trades(&self, exchange: &str, pair: &str) -> Result<Vec<Trade>, String> {
            let url = format!("{}/markets/{}/{}/trades", CW_BASE_URL, exchange, pair);
            request(url).await
        }

        pub async fn orderbook(&self, exchange: &str, pair: &str) -> Result<Orderbook, String> {
            let url = format!("{}/markets/{}/{}/orderbook", CW_BASE_URL, exchange, pair);
            request(url).await
        }

        pub async fn ohlc(
            &self,
            exchange: &str,
            pair: &str,
        ) -> Result<HashMap<String, Vec<Candle>>, String> {
            let url = format!("{}/markets/{}/{}/ohlc", CW_BASE_URL, exchange, pair);
            request(url).await
        }
    }

    /// A wrapper for the Exchange resource and its operations
    pub struct ExchangeAPI {}

    impl HasURI for ExchangeAPI {
        fn uri(&self) -> String {
            "exchanges".to_string()
        }
    }

    impl List<Exchange> for ExchangeAPI {}

    impl Details<Exchange> for ExchangeAPI {}

    impl ExchangeAPI {
        pub async fn markets(&self, name: &str) -> Result<Vec<Market>, String> {
            let url = format!("{}/markets/{}", CW_BASE_URL, name);
            request(url).await
        }
    }

    async fn http_get<U: IntoUrl>(url: U) -> Result<RESTResponse, String> {
        Ok(reqwest::get(url)
            .await
            .expect("Failed get request")
            .json()
            .await
            .expect("Failed to serialise response to JSON"))
    }

    /// Generic helper for parsing the response variants from the CW API
    fn convert_response<T: DeserializeOwned>(resp: RESTResponse) -> Result<T, String> {
        if let Some(v) = resp.result {
            Ok(serde_json::from_value(v).expect("Unexpected response"))
        } else if let Some(error) = resp.error {
            Err(error.clone())
        } else {
            Err("No normal or error response available".into())
        }
    }

    /// Generic helper for HTTP GET and JSON response parsing
    async fn request<U: IntoUrl, T: DeserializeOwned>(url: U) -> Result<T, String> {
        let resp = http_get(url).await?;
        convert_response(resp)
    }
}

/// Entrypoint for REST API resources
#[async_trait::async_trait]
pub trait CryptowatchAPI {
    /// Get the PairsAPI
    fn pairs(&self) -> PairsAPI;

    /// Get the MarketAPI
    fn market(&self) -> MarketAPI;

    /// Get the ExchangeAPI
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
    fn pairs(&self) -> PairsAPI {
        PairsAPI {}
    }

    fn market(&self) -> MarketAPI {
        MarketAPI {}
    }

    fn exchange(&self) -> ExchangeAPI {
        ExchangeAPI {}
    }
}
