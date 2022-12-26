// lib.rs - author: steinkirch
// libraries don't have entry points, but used for functionality sharing

use std::env;

pub mod accounts;
pub mod helpers;


pub async fn run() {
    
    let addreses = &env::var("ACCOUNT_ADDRESS").unwrap();

    let result_ws = accounts::account_ws(addreses).await;
    println!("✅ account_ws() finalized: {:?}", result_ws);

    let result_http = accounts::account_http(addreses).await;
    println!("✅ account_http() finalized: {:?}", result_http);

}




