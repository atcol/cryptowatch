use cryptowatch::api::rest::models::{Exchange, Market};

use cryptowatch::api::{Cryptowatch, CryptowatchAPI};

#[tokio::test]
async fn test_exchanges() {
    let api = Cryptowatch::default();
    let exchanges: Vec<Exchange> = api.exchange().list().await.unwrap();
    assert_ne!(0, exchanges.len());
    let exch_names: Vec<String> = exchanges.into_iter().map(|e| e.name).collect();

    for exchange in vec!["Kraken", "Bitfinex", "Binance"].into_iter() {
        assert!(
            exch_names.contains(&exchange.to_string()),
            "Missing {}",
            exchange
        );
    }
}

#[tokio::test]
async fn test_exchange_details() {
    let api = Cryptowatch::default();
    for exchange in vec!["kraken", "bitfinex", "binance"].into_iter() {
        let ex: Exchange = api.exchange().detail(&exchange).await.unwrap();
        assert_eq!(ex.symbol, exchange);
    }
}

#[tokio::test]
async fn test_exchange_markets() {
    let api = Cryptowatch::default();
    for exchange in vec!["kraken", "bitfinex", "binance"].into_iter() {
        let markets: Vec<Market> = api.exchange().markets(&exchange).await.unwrap();
        for market in markets.into_iter() {
            assert_eq!(market.exchange, exchange, "{:?} doesn't match {}", market, exchange);
        }
    }
}
