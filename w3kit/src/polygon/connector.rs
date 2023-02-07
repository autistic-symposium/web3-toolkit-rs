// polygon/connectors.rs
// author: steinkirch

use std::env;



////////////////////////////
// Public functions
////////////////////////////

pub async fn polygon_connect_ws() {

    let url = &env::var("POLYGON_URL_WS").expect("⛔️ No POLYGON_URL_WS on .env file");
    println!("⛔️ polygon_connect_ws() not implemented yet");

}

pub async fn polygon_connect_http() {

    let url = &env::var("POLYGON_URL_HTTP").expect("⛔️ No POLYGON_URL_HTTP on .env file");
    println!("⛔️ polygon_connect_http() not implemented yet");

}

pub async fn polygon_get_account(account_address: &str) {

    println!("⛔️ polygon_get_account() not implemented yet");

}
