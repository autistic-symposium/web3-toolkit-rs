// avalanche/connectors.rs
// author: steinkirch

use std::env;


////////////////////////////
// Public functions
////////////////////////////

pub async fn avalanche_connect_ws() {

    let url = &env::var("AVALANCHE_URL_WS").expect("⛔️ No AVALANCHE_URL_WS on .env file");

}

pub async fn avalanche_connect_http() {

    let url = &env::var("AVALANCHE_URL_HTTP").expect("⛔️ No AVALANCHE_URL_HTTP on .env file");

}

pub async fn avalanche_get_account(account_address: &str) {

    println!("✅ retrieving balances...");

}
