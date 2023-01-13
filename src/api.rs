use self::rest::{ExchangeAPI, MarketAPI, PairsAPI};

pub mod markets;

pub mod rest {
    use std::collections::HashMap;

    use reqwest::IntoUrl;

    use self::models::{Allowance, Cursor, Exchange, Market, MarketSummary, Orderbook, Price, Trade, Candle, Pair};
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

    pub struct PagedResult<T> {
        pub cursor: Option<Cursor>,
        pub result: T,
    }

    /// A wrapper for the Pair resource and its operations
    pub struct PairsAPI {
        pub(crate) base_url: &'static str,
    }

    impl PairsAPI {

        pub async fn list(&self, cursor: Option<Cursor>) -> Result<PagedResult<Vec<Pair>>, String> {
            let url = if let Some(c) = cursor {
                format!("{}/pairs?cursor={}", self.base_url, c.last)
            } else {
                format!("{}/pairs", self.base_url,)
            };

            let resp = http_get(url).await?;

            Ok(PagedResult {
                cursor: resp.cursor.clone(),
                result: convert_response(resp)?,
            })
        }
    }

    /// A wrapper for the Market resource and its operations
    pub struct MarketAPI {
        base_url: &'static str,
    }

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
            let resp = http_get(url).await?;
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

        pub async fn prices(&self, cursor: Option<Cursor>) -> Result<PricesResultPage, String> {
            let url = if let Some(c) = cursor {
                format!("{}/markets/prices?cursor={}", self.base_url, c.last)
            } else {
                format!("{}/markets/prices", self.base_url,)
            };

            let resp = http_get(url).await?;

            Ok(PricesResultPage {
                cursor: resp.cursor.clone(),
                prices: convert_response(resp)?,
            })
        }

        pub async fn trades(&self, exchange: &str, pair: &str) -> Result<Vec<Trade>, String> {
            let url = format!("{}/markets/{}/{}/trades", self.base_url, exchange, pair);
            request(url).await
        }

        pub async fn orderbook(&self, exchange: &str, pair: &str) -> Result<Orderbook, String> {
            let url = format!("{}/markets/{}/{}/orderbook", self.base_url, exchange, pair);
            request(url).await
        }

        pub async fn ohlc(&self, exchange: &str, pair: &str) -> Result<HashMap<String, Vec<Candle>>, String> {
            let url = format!("{}/markets/{}/{}/ohlc", self.base_url, exchange, pair);
            request(url).await
        }
    }

    /// A wrapper for the Exchange resource and its operations
    pub struct ExchangeAPI {
        base_url: &'static str,
    }

    impl ExchangeAPI {
        pub(crate) fn new(base_url: &'static str) -> Self {
            Self { base_url }
        }

        pub async fn list(&self) -> Result<Vec<Exchange>, String> {
            let url = format!("{}/exchanges", self.base_url);
            request(url).await
        }

        pub async fn detail(&self, name: &str) -> Result<Exchange, String> {
            let url = format!("{}/exchanges/{}", self.base_url, name);
            request(url).await
        }

        pub async fn markets(&self, name: &str) -> Result<Vec<Market>, String> {
            let url = format!("{}/markets/{}", self.base_url, name);
            request(url).await
        }
    }

    async fn http_get<U: IntoUrl>(url: U, ) -> Result<RESTResponse, String> {
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
    async fn request<U: IntoUrl, T: DeserializeOwned>(url: U, ) -> Result<T, String> {
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
        PairsAPI { base_url: self.base_url.clone() }
    }

    fn market(&self) -> MarketAPI {
        MarketAPI::new(self.base_url.clone())
    }

    fn exchange(&self) -> ExchangeAPI {
        ExchangeAPI::new(self.base_url.clone())
    }
}
