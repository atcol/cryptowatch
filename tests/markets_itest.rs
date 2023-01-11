use cryptowatch::api::rest::models::MarketSummary;
use cryptowatch::api::rest::MarketPage;
use cryptowatch::api::{Cryptowatch, CryptowatchAPI};

#[tokio::test]
async fn test_markets() {
    let api = Cryptowatch::default();
    let page1: MarketPage = api.market().list(None).await.unwrap();
    assert_ne!(0, page1.markets.len());
    assert!(page1.cursor.as_ref().map(|c| c.has_more).unwrap_or(false));
    let pairs1: Vec<String> = page1.markets.into_iter().map(|m| m.pair).collect();

    for pair in vec!["btcgbp", "btceur", "btcusd"].into_iter() {
        assert!(pairs1.contains(&pair.to_owned()), "Missing {}", pair);
    }

    let page2: MarketPage = api.market().list(page1.cursor).await.unwrap();
    assert_ne!(0, page2.markets.len());
    assert_eq!(page2.cursor.as_ref().map(|c| c.has_more).unwrap_or(false), false);

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
