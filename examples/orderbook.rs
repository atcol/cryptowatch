use cryptowatch::api::{Cryptowatch, CryptowatchAPI, rest::models::Price};

#[tokio::main]
async fn main() {
    let cw = Cryptowatch::default();
    let results = futures::future::join_all(vec![
        cw.market().orderbook("kraken","btcusd"),
        cw.market().orderbook("binance-us", "btcusdt"),
        cw.market().orderbook("okx", "btcusdt")
    ]).await;


    for result in results.into_iter().map(|r| r.unwrap()) {
        let top_ask = &result.asks[0];
        let top_bid = &result.bids[0];
        println!("Result is {:?} {:?}", top_ask, top_bid);
    }
}
