// arbitrum/connectors.rs
// author: steinkirch

use std::env;


////////////////////////////
// Public functions
////////////////////////////

pub async fn arbitrum_connect_ws() {

    let url = &env::var("ARBITRUM_URL_WS").expect("⛔️ No ARBITRUM_URL_WS on .env file");

}

pub async fn arbitrum_connect_http() {

    let url = &env::var("ARBITRUM_URL_HTTP").expect("⛔️ No ARBITRUM_URL_HTTP on .env file");

}

pub async fn arbitrum_get_account(account_address: &str) {

    println!("✅ retrieving balances...");

}
