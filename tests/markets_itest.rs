use cryptowatch::api::rest::models::{Market, MarketSummary, Price, Trade, Candle};
use cryptowatch::api::rest::MarketsResultPage;
use cryptowatch::api::{Cryptowatch, CryptowatchAPI};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn test_markets() {
    let api = Cryptowatch::default();
    let page1: MarketsResultPage = api.market().list(None).await.unwrap();
    assert_ne!(0, page1.markets.len());
    assert!(page1.cursor.as_ref().map(|c| c.has_more).unwrap_or(false));
    let pairs1: Vec<String> = page1.markets.into_iter().map(|m| m.pair).collect();

    for pair in vec!["btcgbp", "btceur", "btcusd"].into_iter() {
        assert!(pairs1.contains(&pair.to_owned()), "Missing {}", pair);
    }

    let page2: MarketsResultPage = api.market().list(page1.cursor).await.unwrap();
    assert_ne!(0, page2.markets.len());
    assert_eq!(
        page2.cursor.as_ref().map(|c| c.has_more).unwrap_or(false),
        false
    );
}

#[tokio::test]
async fn test_summary() {
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

#[tokio::test]
async fn test_details() {
    let api = Cryptowatch::default();
    let summary: Market = api.market().details("kraken", "btcgbp").await.unwrap();
    assert_eq!(88, summary.id);
    assert_eq!("kraken", summary.exchange);
    assert_eq!("btcgbp", summary.pair);
    assert_eq!(true, summary.active);
}

#[tokio::test]
async fn test_price() {
    let api = Cryptowatch::default();
    let price: Price = api.market().price("kraken", "btcgbp").await.unwrap();
    assert!(price.price > 0.);
}

#[tokio::test]
async fn test_trades() {
    let api = Cryptowatch::default();
    let trades: Vec<Trade> = api.market().trades("kraken", "btcgbp").await.unwrap();
    assert!(trades.len() > 0);

    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap().as_millis();

    for trade in trades.iter() {
        assert!(trade.id >= 0.);
        assert!(trade.price > 0.);
        assert!(u128::from(trade.timestamp) <= since_the_epoch);
        assert!(trade.amount > 0.);
    }
}

#[tokio::test]
async fn test_ohlc() {
    let api = Cryptowatch::default();
    let ohlc: HashMap<String, Vec<Candle>> = api.market().ohlc("kraken", "btcgbp").await.unwrap();
    assert!(ohlc.len() > 0);

    assert!(ohlc.get("60").unwrap().len() > 0); //  1m
    assert!(ohlc.get("180").unwrap().len() > 0); // 3m
    assert!(ohlc.get("300").unwrap().len() > 0); // 5m
    assert!(ohlc.get("900").unwrap().len() > 0); // 15m
    assert!(ohlc.get("1800").unwrap().len() > 0); // 30m
    assert!(ohlc.get("3600").unwrap().len() > 0); // 1h
    assert!(ohlc.get("7200").unwrap().len() > 0); // 2h
    assert!(ohlc.get("14400").unwrap().len() > 0); // 4h
    assert!(ohlc.get("21600").unwrap().len() > 0); // 6h
    assert!(ohlc.get("43200").unwrap().len() > 0); // 12h
    assert!(ohlc.get("86400").unwrap().len() > 0); // 1d
    assert!(ohlc.get("259200").unwrap().len() > 0); // 3d
    assert!(ohlc.get("604800").unwrap().len() > 0); // 1w
    assert!(ohlc.get("604800_Monday").unwrap().len() > 0); // 1w (weekly start Monday)
}
