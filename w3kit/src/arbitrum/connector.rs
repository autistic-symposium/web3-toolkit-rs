// arbitrum/connectors.rs
// author: steinkirch

use std::env;


pub async fn arbitrum_connect_ws(account_address: &str) {

    let url = &env::var("ARBITRUM_URL_WS").expect("⛔️ No ARBITRUM_URL_WS on .env file");

}

pub async fn arbitrum_connect_http(account_address: &str) {

    let url = &env::var("ARBITRUM_URL_HTTP").expect("⛔️ No ARBITRUM_URL_HTTP on .env file");

}

async fn get_accounts() {

    println!("✅ retrieving balances...");

}
