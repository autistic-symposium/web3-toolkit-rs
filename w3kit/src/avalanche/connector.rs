// avalanche/connectors.rs
// author: steinkirch

use std::env;


pub async fn avalanche_connect_ws(account_address: &str) {

    let url = &env::var("AVALANCHE_URL_WS").expect("⛔️ No AVALANCHE_URL_WS on .env file");

}

pub async fn avalanche_connect_http(account_address: &str) {

    let url = &env::var("AVALANCHE_URL_HTTP").expect("⛔️ No AVALANCHE_URL_HTTP on .env file");

}

async fn get_accounts() {

    println!("✅ retrieving balances...");

}
