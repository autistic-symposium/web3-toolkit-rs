// optimism/connectors.rs
// author: steinkirch

use std::env;



////////////////////////////
// Public functions
////////////////////////////

pub async fn optimism_connect_ws() {

    let url = &env::var("OPTIMISM_URL_WS").expect("⛔️ No OPTIMISM_URL_WS on .env file");

}

pub async fn optimism_connect_http() {

    let url = &env::var("OPTIMISM_URL_HTTP").expect("⛔️ No OPTIMISM_URL_HTTP on .env file");

}

pub async fn optimism_get_account(account_address: &str) {

    println!("✅ retrieving balances...");

}
