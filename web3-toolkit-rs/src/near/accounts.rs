// near/acccounts.rs - author: steinkirch
// methods for wallets and accounts

use std::str::FromStr;
use web3::types::{H160};


pub type Transport = web3::transports::Either<web3::transports::WebSocket, web3::transports::Http>;


pub async fn web3_connect(provider_url: &str, account_address: &str) {

    println!("to be implemented");

}


async fn get_accounts(transport: Transport, account_address: &str)  {

    println!("to be implemented");

}


pub async fn get_accounts_ws(transport: WebSocket, addresses: &str) {

    println!("to be implemented");

}


pub async fn get_accounts_either(transport: Transport, addresses: &str) {

    println!("to be implemented");

}


pub async fn get_accounts_batches(transport: Http, addresses: &str) {

    println!("to be implemented");

}