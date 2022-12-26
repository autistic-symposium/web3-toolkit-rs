// acccounts.rs - author: steinkirch
// methods for wallets and accounts

use std::env;
use std::str::FromStr;

use web3::types::{H160};


pub async fn websocket() {

    let websocket = web3::transports::WebSocket::new(&env::var("PROVIDER_URL_WS").unwrap()).await;
    let web3s = web3::Web3::new(websocket);

}