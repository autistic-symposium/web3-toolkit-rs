// solana/connectors.rs
// author: steinkirch

use std::env;


////////////////////////////
// Public functions
////////////////////////////

pub async fn solana_connect_ws() {

    let url = &env::var("SOLANA_URL_WS").expect("⛔️ No SOLANA_URL_WS on .env file");

}

pub async fn solana_connect_http() {

    let url = &env::var("SOLANA_URL_HTTP").expect("⛔️ No SOLANA_URL_HTTP on .env file");

}

pub async fn solana_get_account(account_address: &str) {

    println!("✅ retrieving balances...");

}


