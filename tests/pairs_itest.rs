use cryptowatch::api::rest::models::Pair;
use cryptowatch::api::rest::{Details, List, PagedResult};
use cryptowatch::api::{Cryptowatch, CryptowatchAPI};

#[tokio::test]
async fn test_pairs() {
    let api = Cryptowatch::default();
    let page1: PagedResult<Vec<Pair>> = api.pairs().list(None).await.unwrap();
    let pair1 = page1.result.get(0).unwrap();
    assert!(page1.cursor.is_some());
    assert!(pair1.symbol == "0neweth");

    let base1 = pair1.base.as_ref().unwrap();
    assert_eq!(base1.sid, "stone");
    assert_eq!(base1.symbol, "0ne");
    assert_eq!(base1.name, "Stone");
    assert!(!base1.fiat);

    let quote1 = pair1.quote.as_ref().unwrap();
    assert_eq!(quote1.sid, "weth");
    assert_eq!(quote1.symbol, "weth");
    assert_eq!(quote1.name, "Wrapped Ether");
    assert!(!quote1.fiat);

    let page2: PagedResult<Vec<Pair>> = api.pairs().list(page1.cursor).await.unwrap();
    let pair2 = page2.result.get(0).unwrap();
    assert!(page2.cursor.is_some());
    assert_ne!(pair2.symbol, "0neweth");

    let base2 = pair2.base.as_ref().unwrap();
    assert_ne!(base2.sid, "stone");
    assert_ne!(base2.symbol, "0ne");
    assert_ne!(base2.name, "Stone");
    assert!(!base2.fiat);

    let quote2 = pair2.quote.as_ref().unwrap();
    assert_ne!(quote2.sid, "weth");
    assert_ne!(quote2.symbol, "weth");
    assert_ne!(quote2.name, "Wrapped Ether");
    assert!(!quote2.fiat);
}

#[tokio::test]
async fn test_pair() {
    let api = Cryptowatch::default();
    let pair: Pair = api.pairs().details("btcgbp").await.unwrap();
    assert!(pair.symbol == "btcgbp");

    let base1 = pair.base.as_ref().unwrap();
    assert_eq!(base1.sid, "bitcoin");
    assert_eq!(base1.symbol, "btc");
    assert_eq!(base1.name, "Bitcoin");
    assert!(!base1.fiat);

    let quote1 = pair.quote.as_ref().unwrap();
    assert_eq!(quote1.sid, "british-pound");
    assert_eq!(quote1.symbol, "gbp");
    assert_eq!(quote1.name, "British Pound");
    assert!(quote1.fiat);
}
