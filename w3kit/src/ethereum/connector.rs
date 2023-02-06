// ethereum/connectors.rs
// author: steinkirch

use std::str::FromStr;
use web3::types::{H160};

pub type Transport = web3::transports::Either<web3::transports::WebSocket, web3::transports::Http>;


use crate::{
    utils::maths::wei_to_eth,
};

pub async fn ethereum_connect(blockchain: &str, provider_url: &str, account_address: &str) -> web3::Result {

    println!("✅ connecting to {:?}", blockchain);
    let transport = web3::transports::Http::new(provider_url)?;
    get_accounts(web3::transports::Either::Right(transport), account_address).await

}

async fn get_accounts(transport: Transport, account_address: &str) -> web3::Result<()> {

    let web3s = web3::Web3::new(transport);

    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(account_address).unwrap());
    println!("✅ retrieving balances...");

    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("      💰 account {:?} 👉 {} ETH", account, wei_to_eth(balance));
    }
    
    Ok(())

}
