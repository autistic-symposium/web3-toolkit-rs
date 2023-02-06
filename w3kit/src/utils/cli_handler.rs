// src/utils/cli_handler.rs
// author: steinkirch

use std::env;


use crate::{
    utils::commands::{ConnectionArgs},
    ethereum::connector::ethereum_connect,
    arbitrum::connector::{arbitrum_connect_ws, arbitrum_connect_http},
    avalanche::connector::{avalanche_connect_ws, avalanche_connect_http},
    optimism::connector::{optimism_connect_ws, optimism_connect_http},
    near::connector::{near_connect_ws, near_connect_http},
    polygon::connector::{polygon_connect_ws, polygon_connect_http},
    solana::connector::{solana_connect_ws, solana_connect_http},
};


pub async fn handle_ws(args: ConnectionArgs) {

    let blockchain = &args.blockchain.to_string();
    let account = &args.account.to_string();

    println!("✅ connecting to {:?}", blockchain);

    match String::from(blockchain).as_str() {

        "arbitrum" => arbitrum_connect_ws(account).await,
        "avalanche" => avalanche_connect_ws(account).await,
        "optimism" => optimism_connect_ws(account).await,
        "near" => near_connect_ws(account).await,
        "polygon" => polygon_connect_ws(account).await,
        "solana" => solana_connect_ws(account).await,
        "ethereum" =>  {
            let url = &env::var("ETHEREUM_URL_WS").expect("⛔️ No ETHEREUM_URL_WS on .env file");
            match ethereum_connect(url, account).await {
                Err(e) => println!("⛔️ {:?}", e),
                _ => ()
            }
        }
        &_ => todo!()
    } 
}


pub async fn handle_http(args: ConnectionArgs) {

    let blockchain =  &args.blockchain.to_string();
    let account = &args.account.to_string();

    println!("✅ connecting to {:?}", blockchain);

    match String::from(blockchain).as_str() {

        "arbitrum" => arbitrum_connect_http(account).await,
        "avalanche" => avalanche_connect_http(account).await,
        "optimism" => optimism_connect_http(account).await,
        "near" => near_connect_http(account).await,
        "polygon" => polygon_connect_http(account).await,
        "solana" => solana_connect_http(account).await,
        "ethereum" =>  {
            let url = &env::var("ETHEREUM_URL_HTTP").expect("⛔️ No ETHEREUM_URL_HTTP on .env file");
            match ethereum_connect(url, account).await {
                Err(e) => println!("⛔️ {:?}", e),
                _ => ()
            }
        }
        &_ => todo!()
    }
}