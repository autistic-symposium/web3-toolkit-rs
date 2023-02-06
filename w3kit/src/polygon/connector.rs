// polygon/connectors.rs
// author: steinkirch

use std::env;


pub async fn polygon_connect_ws(account_address: &str) {

    let url = &env::var("POLYGON_URL_WS").expect("⛔️ No POLYGON_URL_WS on .env file");

}

pub async fn polygon_connect_http(account_address: &str) {

    let url = &env::var("POLYGON_URL_HTTP").expect("⛔️ No POLYGON_URL_HTTP on .env file");

}

async fn get_accounts() {

    println!("✅ retrieving balances...");

}
