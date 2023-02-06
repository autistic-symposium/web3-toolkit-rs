// optimism/connectors.rs
// author: steinkirch

use std::env;


pub async fn optimism_connect_ws(account_address: &str) {

    let url = &env::var("OPTIMISM_URL_WS").expect("⛔️ No OPTIMISM_URL_WS on .env file");

}

pub async fn optimism_connect_http(account_address: &str) {

    let url = &env::var("OPTIMISM_URL_HTTP").expect("⛔️ No OPTIMISM_URL_HTTP on .env file");

}

async fn get_accounts() {

    println!("✅ retrieving balances...");

}
