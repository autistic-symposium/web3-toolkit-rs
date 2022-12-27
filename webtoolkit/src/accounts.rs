// acccounts.rs - author: steinkirch
// methods for wallets and accounts

use std::env;
use std::str::FromStr;
use web3::types::{H160};
use crate::helpers::wei_to_eth;


pub async fn account_ws(account_address: &str) -> web3::Result<()> {

    let transport = web3::transports::WebSocket::new(&env::var("PROVIDER_URL_WS").unwrap()).await?;
    let web3s = web3::Web3::new(transport);

    #[warn(unused_must_use)]
    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(account_address).unwrap());
    println!("✅ retrieving balances [ws]...");

    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("  - 💰 for {:?}: {} eth", account, wei_to_eth(balance));
    }
    
    Ok(())

}


pub async fn account_http(account_address: &str) -> web3::Result<()> {

    let transport = web3::transports::Http::new(&env::var("PROVIDER_URL_HTTP").unwrap())?;
    let web3s = web3::Web3::new(transport);

    #[warn(unused_must_use)]
    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(account_address).unwrap());
    println!("✅ retrieving balances [http]...");

    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("  - 💰 for {:?}: {} eth", account, wei_to_eth(balance));
    }
    
    Ok(())

}

