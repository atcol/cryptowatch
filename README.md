[![Rust](https://github.com/atcol/cryptowatch/actions/workflows/rust.yml/badge.svg)](https://github.com/atcol/cryptowatch/actions/workflows/rust.yml)

# Cryptowatch

Unofficial Cryptowatch Rust SDK

## Examples

#### Kraken Orderbook

```rust
use cryptowatch::api::{Cryptowatch, CryptowatchAPI};

#[tokio::main]
async fn main() {
    let cw = Cryptowatch::default();
    let results = cw.market().orderbook("kraken","btcusd").await.unwrap();
    let top_ask = results.top_ask();
    let top_bid = results.top_bid();
}
```

