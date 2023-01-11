use cryptowatch::api::rest::models::{Market, MarketSummary};
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

#[tokio::test]
async fn test_market_summary() {
    let api = Cryptowatch::default();
    let summary: MarketSummary = api.market().summary("kraken", "btcgbp").await.unwrap();

    let candle = summary.price.expect("Missing Candle");
    let delta = candle.change.expect("Missing delta");
    assert!(candle.last > 0.);
    assert!(candle.high > 0.);
    assert!(candle.low > 0.);
    assert!(delta.absolute > 0.);
    assert!(delta.percentage > 0.);
    assert!(summary.volume > 0.);
    assert!(summary.volume_quote > 0.);

}
