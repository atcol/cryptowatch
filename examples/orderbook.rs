use cryptowatch::api::{Cryptowatch, CryptowatchAPI};

#[tokio::main]
async fn main() {
    let cw = Cryptowatch::default();
    let results = futures::future::join_all(vec![
        cw.market().orderbook("kraken","btcusd"),
        cw.market().orderbook("binance-us", "btcusdt"),
        cw.market().orderbook("okx", "btcusdt")
    ]).await;

    for orderbook in results.into_iter().map(|r| r.unwrap()) {
        let top_ask = orderbook.top_ask();
        let top_bid = orderbook.top_bid();
        let spread = orderbook.spread();
        println!("Result is {:?} {:?} {:?}", top_ask, top_bid, spread);
    }
}
