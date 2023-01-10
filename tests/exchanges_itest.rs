use cryptowatch::api::rest::models::Exchange;

use cryptowatch::api::{Cryptowatch, CryptowatchApi};

#[tokio::test]
async fn test_exchanges() {
    let api = Cryptowatch::default();
    let exchanges: Vec<Exchange> = api.exchanges().await.unwrap();
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
        let ex: Exchange = api.exchange(&exchange).await.unwrap();
        assert_eq!(ex.symbol, exchange);
    }
}
