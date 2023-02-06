// near/connectors.rs
// author: steinkirch

use std::env;


pub async fn near_connect_ws(account_address: &str) {

    let url = &env::var("NEAR_URL_WS").expect("⛔️ No NEAR_URL_WS on .env file");

}

pub async fn near_connect_http(account_address: &str) {

    let url = &env::var("NEAR_URL_HTTP").expect("⛔️ No NEAR_URL_HTTP on .env file");

}

async fn get_accounts() {

    println!("✅ retrieving balances...");

}
