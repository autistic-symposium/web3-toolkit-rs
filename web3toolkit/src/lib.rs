// lib.rs - author: steinkirch
// libraries don't have entry points, but used for functionality sharing
// https://doc.rust-lang.org/rust-by-example/crates/lib.html

use std::env;

pub mod accounts;
pub mod helpers;


pub async fn run() {
    
    // TODO: get an array of addresses instead of a str
    let addresses = &env::var("ACCOUNTS_LIST").unwrap();
    
    accounts::web3_connect(&env::var("PROVIDER_URL_WS").unwrap(), addresses).await;
    accounts::web3_connect(&env::var("PROVIDER_URL_HTTP").unwrap(), addresses).await;

}
