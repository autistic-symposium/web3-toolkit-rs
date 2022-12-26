// lib.rs - author: steinkirch
// libraries don't have entry points, but used for functionality sharing

use std::env;

pub mod accounts;
pub mod utils;


pub async fn run() {

    let addreses = &env::var("ACCOUNT_ADDRESS").unwrap();

    let result = accounts::account(addreses).await;
    println!("✅ lib.run() finalized: {:?}", result);

}




