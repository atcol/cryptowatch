[![Rust](https://github.com/atcol/cryptowatch/actions/workflows/rust.yml/badge.svg)](https://github.com/atcol/cryptowatch/actions/workflows/rust.yml)

# Cryptowatch

Unofficial Cryptowatch Rust SDK



## Examples

#### Kraken Orderbook

```
use cryptowatch::api::{Cryptowatch, CryptowatchAPI};

#[tokio::main]
async fn main() {
    let cw = Cryptowatch::default();
    let results = cw.market().orderbook("kraken","btcusd").await.unwrap();
    // Do something with results...
}

```


