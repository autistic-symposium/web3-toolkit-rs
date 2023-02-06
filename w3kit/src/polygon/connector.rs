// polygon/connectors.rs
// author: steinkirch

use std::env;



////////////////////////////
// Public functions
////////////////////////////

pub async fn polygon_connect_ws() {

    let url = &env::var("POLYGON_URL_WS").expect("⛔️ No POLYGON_URL_WS on .env file");

}

pub async fn polygon_connect_http() {

    let url = &env::var("POLYGON_URL_HTTP").expect("⛔️ No POLYGON_URL_HTTP on .env file");

}

pub async fn polygon_get_account(account_address: &str) {

    println!("✅ retrieving balances...");

}
