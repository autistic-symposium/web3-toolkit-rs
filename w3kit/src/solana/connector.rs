// solana/connectors.rs
// author: steinkirch

use std::env;


pub async fn solana_connect_ws(account_address: &str) {

    let url = &env::var("SOLANA_URL_WS").expect("⛔️ No SOLANA_URL_WS on .env file");

}

pub async fn solana_connect_http(account_address: &str) {

    let url = &env::var("SOLANA_URL_HTTP").expect("⛔️ No SOLANA_URL_HTTP on .env file");




    

}

async fn get_accounts() {

    println!("✅ retrieving balances...");

}


