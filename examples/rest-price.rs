use cryptowatch::api::{Cryptowatch, CryptowatchAPI, rest::models::Price};

#[tokio::main]
async fn main() {
    let results = futures::future::join_all(vec![
        get_price("kraken","btcusd"),
        get_price("binance-us", "btcusdt"),
        get_price("okx", "btcusdt")
    ]).await;


    for result in results.iter() {
        println!("{} price for BTCUSD[T] is {}", result.0, result.1.price);
    }
}

async fn get_price(exchange: &'static str, pair: &'static str) -> (&'static str, Price) {
    let cw = Cryptowatch::default();
    cw.market().price(exchange, pair).await.map(|p| (exchange, p)).unwrap()
}
