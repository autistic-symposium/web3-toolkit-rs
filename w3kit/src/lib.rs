// src/lib.rs - author: steinkirch
// note that in rust, libraries don't have entry points, 
// but they are used for functionality sharing.

use std::env;

pub mod utils;

pub async fn run() {
    
    // TODO: get an array of addresses instead of a str
    let addresses = &env::var("ACCOUNTS_LIST").unwrap();
    let provider_ws = &env::var("PROVIDER_URL_WS").unwrap();
    let provider_http = &env::var("PROVIDER_URL_HTTP").unwrap();   


}
