// ethereum/connections.rs
// author: steinkirch

use std::str::FromStr;
use web3::types::{H160};

use crate::helpers::wei_to_eth;

pub type Transport = web3::transports::Either<web3::transports::WebSocket, web3::transports::Http>;
pub type Http = web3::transports::Http;
pub type WebSocket = web3::transports::WebSocket;



pub async fn connect_ws(provider_url: &str, addresses: &str) -> web3::Result {

    println!("✅ connecting ws {:?}", provider_url);
    let transport = WebSocket::new(provider_url).await?;
    
    get_accounts_ws(transport, addresses).await
    
}


pub async fn connect_http(provider_url: &str, addresses: &str) -> web3::Result {

    println!("✅ connecting http {:?}", provider_url);
    let transport = Http::new(provider_url)?;
    
    get_accounts_http(transport, addresses).await
    
}


pub async fn connect_either(provider_url: &str, addresses: &str) -> web3::Result {

    println!("✅ connecting either ws/http{:?}", provider_url);
    let transport = Http::new(provider_url)?;
    
    get_accounts_either(web3::transports::Either::Right(transport), addresses).await
    
}


pub async fn connect_batch(provider_url: &str, addresses: &str) -> web3::Result {

    println!("✅ connecting batches {:?}", provider_url);
    let transport = Http::new(provider_url)?;

    get_accounts_batches(transport, addresses).await

}







