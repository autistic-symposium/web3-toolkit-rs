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

    //let _accounts_ws = web3stuff::web3_connect(provider_ws, addresses).await;
    let accounts_http = web3stuff::web3_connect(provider_http, addresses).await;


}
