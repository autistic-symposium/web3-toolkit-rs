// solana/connectors.rs
// author: steinkirch

use std::env;


////////////////////////////
// Public functions
////////////////////////////

pub async fn solana_connect_ws() {

    let url = &env::var("SOLANA_URL_WS").expect("⛔️ No SOLANA_URL_WS on .env file");
    println!("⛔️ solana_connect_ws() not implemented yet");

}

pub async fn solana_connect_http() {

    let url = &env::var("SOLANA_URL_HTTP").expect("⛔️ No SOLANA_URL_HTTP on .env file");
    println!("⛔️ solana_connect_http() not implemented yet");

}

pub async fn solana_get_account(account_address: &str) {

    println!("⛔️ solana_get_account() not implemented yet");

}


