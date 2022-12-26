// acccounts.rs - author: steinkirch
// methods for wallets and accounts

use std::env;
use std::str::FromStr;
use web3::types::{H160};
use crate::utils::wei_to_eth;


pub async fn account(account_address: &str) -> web3::Result<()> {

    let websocket = web3::transports::WebSocket::new(&env::var("PROVIDER_URL_WS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(account_address).unwrap());
    println!("✅ retrieving balances for accounts: {:?}", accounts);

    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("  - 💰 for {:?}: {} eth", account, wei_to_eth(balance));
    }
    
    Ok(())

}