use cryptowatch::api::rest::models::Market;
use cryptowatch::api::{Cryptowatch, CryptowatchAPI};

#[tokio::test]
async fn test_markets() {
    let api = Cryptowatch::default();
    let markets: Vec<Market> = api.market().list().await.unwrap();
    assert_ne!(0, markets.len());
    let pairs: Vec<String> = markets.into_iter().map(|m| m.pair).collect();

    for pair in vec!["btcgbp", "btceur", "btcusd"].into_iter() {
        assert!(pairs.contains(&pair.to_owned()), "Missing {}", pair);
    }
}
