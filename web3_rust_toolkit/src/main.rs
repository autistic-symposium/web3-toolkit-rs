use std::env;
use std::str::FromStr;

use web3::contract::{Contract, Options};
use web3::types::{H160, Address, U256};

mod utils;


#[tokio::main]
async fn main() -> web3::Result<()> {

    //
    // SETUP
    //
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("PROVIDER_URL_WS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    //
    // ACCOUNTS
    //
    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("✅ accounts: {:?}", accounts);

    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("💰 for {:?}: {}", account, utils::wei_to_eth(balance));
    }
    
    //
    // CONTRACTS
    //
    let contract_addr = Address::from_str(&env::var("ACCOUNTCONTRACT_ADDRESS_ADDRESS").unwrap());

    let token_contract =
        Contract::from_json(web3s.eth(), contract_addr, include_bytes!(&env::var("CONTRACT_ABI").unwrap())).unwrap();

    let token_name: String = token_contract
        .query("name", (), None, Options::default(), None)
        .await
        .unwrap();

    let total_supply: U256 = token_contract
        .query("totalSupply", (), None, Options::default(), None)
        .await
        .unwrap();

    println!("✅ token {} total supply: {}", token_name, total_supply);

    //
    // FINALIZE
    //
    Ok(())
}