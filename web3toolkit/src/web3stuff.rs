// web3stuff.rs - author: steinkirch
// methods for wallets and accounts

use std::str::FromStr;
use web3::types::{H160};

use crate::helpers::wei_to_eth;

pub type Transport = web3::transports::Either<web3::transports::WebSocket, web3::transports::Http>;
pub type Http = web3::transports::Http;
pub type WebSocket = web3::transports::WebSocket;


pub async fn web3_connect_ws(provider_url: &str, addresses: &str) -> web3::Result {

    println!("✅ connecting ws {:?}", provider_url);
    let transport = WebSocket::new(provider_url).await?;
    
    get_accounts_ws(transport, addresses).await
    
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


pub async fn web3_connect_either(provider_url: &str, addresses: &str) -> web3::Result {

    println!("✅ connecting either ws/http{:?}", provider_url);
    let transport = Http::new(provider_url)?;
    
    get_accounts_either(web3::transports::Either::Right(transport), addresses).await
    
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


pub async fn web3_connect_batch(provider_url: &str, addresses: &str) -> web3::Result {

    println!("✅ connecting batches {:?}", provider_url);
    let transport = Http::new(provider_url)?;

    get_accounts_batches(transport, addresses).await

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

pub async fn web3_connect_http(provider_url: &str, addresses: &str) -> web3::Result {

    println!("✅ connecting http {:?}", provider_url);
    let transport = Http::new(provider_url)?;
    
    get_accounts_http(transport, addresses).await
    
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


