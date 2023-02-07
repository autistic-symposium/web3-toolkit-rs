// src/utils/cli_handler.rs
// author: steinkirch

use std::env;

use crate::{
    utils::commands::{ConnectionArgs,
                      AccountArgs,
                      CoinArgs},
    ethereum::connector::{ethereum_connect, 
                          ethereum_get_account},
    arbitrum::connector::{arbitrum_connect_ws, 
                          arbitrum_get_account,
                          arbitrum_connect_http},
    avalanche::connector::{avalanche_connect_ws, 
                           avalanche_get_account,
                           avalanche_connect_http},
    optimism::connector::{optimism_connect_ws, 
                          optimism_get_account,
                          optimism_connect_http},
    near::connector::{near_connect_ws, 
                      near_get_account,
                      near_connect_http},
    polygon::connector::{polygon_connect_ws,
                         polygon_get_account, 
                         polygon_connect_http},
    solana::connector::{solana_connect_ws, 
                        solana_get_account,
                        solana_connect_http},
    market::coingecko::{get_coin_info},
};

/////////////////////////////////////////
// Public functions
/////////////////////////////////////////

// Note: these methods will be refactored once 
// more specific connectors are implemented

pub async fn handle_ws(args: ConnectionArgs) {

    let blockchain = &args.blockchain.to_string();

    println!("✅ connecting to {:?}", blockchain);

    match String::from(blockchain).as_str() {

        "arbitrum" => arbitrum_connect_ws().await,
        "avalanche" => avalanche_connect_ws().await,
        "optimism" => optimism_connect_ws().await,
        "near" => near_connect_ws().await,
        "polygon" => polygon_connect_ws().await,
        "solana" => solana_connect_ws().await,
        "ethereum" =>  {
            let url = &env::var("ETHEREUM_URL_WS").expect("⛔️ No ETHEREUM_URL_WS on .env file");
            match ethereum_connect(url).await {
                Err(e) => println!("⛔️ {:?}", e),
                _ => ()
            }
        }
        &_ => todo!()
    } 
}


pub async fn handle_http(args: ConnectionArgs) {

    let blockchain =  &args.blockchain.to_string();

    println!("✅ connecting to {:?}", blockchain);

    match String::from(blockchain).as_str() {

        "arbitrum" => arbitrum_connect_http().await,
        "avalanche" => avalanche_connect_http().await,
        "optimism" => optimism_connect_http().await,
        "near" => near_connect_http().await,
        "polygon" => polygon_connect_http().await,
        "solana" => solana_connect_http().await,
        "ethereum" =>  {
            let url = &env::var("ETHEREUM_URL_HTTP").expect("⛔️ No ETHEREUM_URL_HTTP on .env file");
            match ethereum_connect(url).await {
                Err(e) => println!("⛔️ {:?}", e),
                _ => ()
            }
        }
        &_ => todo!()
    }
}


pub async fn handle_account(args: AccountArgs) {

    let blockchain =  &args.blockchain.to_string();
    let account = &args.account.to_string();

    println!("✅ connecting to {:?}", blockchain);
    println!("✅ fetching account info: {:?}", account);

    match String::from(blockchain).as_str() {

        "arbitrum" => arbitrum_get_account(account).await,
        "avalanche" => avalanche_get_account(account).await,
        "optimism" => optimism_get_account(account).await,
        "near" => near_get_account(account).await,
        "polygon" => polygon_get_account(account).await,
        "solana" => solana_get_account(account).await,
        "ethereum" =>  {
            let url = &env::var("ETHEREUM_URL_HTTP").expect("⛔️ No ETHEREUM_URL_HTTP on .env file");
            match ethereum_get_account(url, account).await {
                Err(e) => println!("⛔️ {:?}", e),
                _ => ()
            }
        }
        &_ => todo!()
    }
}


pub async fn handle_coin(args: CoinArgs) {  

    let coin = &args.coin.to_string();
    get_coin_info(coin).await;

}

