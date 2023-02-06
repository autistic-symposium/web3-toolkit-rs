// ethereum/acccounts.rs
// author: steinkirch


use std::str::FromStr;
use web3::types::{H160};

use crate::helpers::wei_to_eth;

pub type Transport = web3::transports::Either<web3::transports::WebSocket, web3::transports::Http>;


pub async fn web3_connect(provider_url: &str, account_address: &str) -> web3::Result {

    println!("✅ connecting to {:?}", provider_url);
    let transport = web3::transports::Http::new(provider_url)?;
    get_accounts(web3::transports::Either::Right(transport), account_address).await

}


async fn get_accounts(transport: Transport, account_address: &str) -> web3::Result<()> {

    let web3s = web3::Web3::new(transport);

    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(account_address).unwrap());
    println!("✅ retrieving balances:");

    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("  - 💰 for {:?}: {} eth", account, wei_to_eth(balance));
    }
    
    Ok(())

}


pub async fn get_accounts_ws(transport: WebSocket, addresses: &str) -> web3::Result {

    let w3 = web3::Web3::new(transport);
    println!("✅ Tranport info: {:?}", w3);

    let mut accounts = w3.eth().accounts().await?;
    accounts.push(H160::from_str(addresses).unwrap());
    println!("✅ retrieving balances:");

    for account in accounts {
        let balance = w3.eth().balance(account, None).await?;
        println!("  - 💰 for {:?}: {} eth", account, wei_to_eth(balance));
    }

    Ok(())
}

pub async fn get_accounts_either(transport: Transport, addresses: &str) -> web3::Result {

    let w3 = web3::Web3::new(transport);
    println!("✅ Tranport info: {:?}", w3);

    let mut accounts = w3.eth().accounts().await?;
    accounts.push(H160::from_str(addresses).unwrap());
    println!("✅ retrieving balances:");

    for account in accounts {
        let balance = w3.eth().balance(account, None).await?;
        println!("  - 💰 for {:?}: {} eth", account, wei_to_eth(balance));
    }

    Ok(())
}

pub async fn get_accounts_batches(transport: Http, addresses: &str) -> web3::Result {

    let w3 = web3::Web3::new(web3::transports::Batch::new(transport));
    println!("✅ Tranport info: {:?}", w3);

    // The next lines are different while sending batches:
    let accounts = w3.eth().accounts();
    let block = w3.eth().block_number();
    
    let result = w3.transport().submit_batch().await?;
    println!("Result: {:?}", result);

    let accounts = accounts.await?;
    println!("Accounts: {:?}", accounts);

    let block = block.await?;
    println!("Block: {:?}", block);

    Ok(())

}


pub async fn get_accounts_http(transport: Http, addresses: &str) -> web3::Result {

    let w3 = web3::Web3::new(transport);
    println!("✅ Tranport info: {:?}", w3);

    let mut accounts = w3.eth().accounts().await?;
    accounts.push(H160::from_str(addresses).unwrap());
    println!("✅ retrieving balances:");

    for account in accounts {
        let balance = w3.eth().balance(account, None).await?;
        println!("  - 💰 for {:?}: {} eth", account, wei_to_eth(balance));
    }

    Ok(())
}
