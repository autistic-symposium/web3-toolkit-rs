// lib.rs - author: steinkirch
// libraries don't have entry points, but used for functionality sharing

use std::env;
use std::str::FromStr;

use web3::types::{H160};

mod utils;



pub async fn run() -> web3::Result<()> {

    let websocket = web3::transports::WebSocket::new(&env::var("PROVIDER_URL_WS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("✅ accounts: {:?}", accounts);

    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("💰 for {:?}: {}", account, utils::wei_to_eth(balance));
    }
    
    
    Ok(())
}



