// lib.rs - author: steinkirch
// libraries don't have entry points, but used for functionality sharing
// https://doc.rust-lang.org/rust-by-example/crates/lib.html

use std::env;

pub mod helpers;
pub mod web3stuff;

pub async fn run() {
    
    // TODO: get an array of addresses instead of a str
    let addresses = &env::var("ACCOUNTS_LIST").unwrap();
    let provider_ws = &env::var("PROVIDER_URL_WS").unwrap();
    let provider_http = &env::var("PROVIDER_URL_HTTP").unwrap();   

    // TEST either ws/http
    web3stuff::web3_connect_either(provider_http, addresses).await;

    // TEST ws
    web3stuff::web3_connect_ws(provider_ws, addresses).await;

    // TEST batches
    web3stuff::web3_connect_batch(provider_http, addresses).await;

    // TEST http
    web3stuff::web3_connect_http(provider_http, addresses).await;

}
