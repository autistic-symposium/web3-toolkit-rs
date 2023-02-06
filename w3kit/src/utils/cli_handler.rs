// src/utils/cli_handler.rs
// author: steinkirch


use crate::{
    utils::commands::ConnectionArgs,
    ethereum,
};

pub async fn handle_ws(args: ConnectionArgs) {

    let blockchain = &args.blockchain.to_string();

    if blockchain == "ethereum" {
        ethereum::connections::web3_connect(&args.url.to_string(), &args.account.to_string()).await;
    } else {
        println!("❌ blockchain not supported yet");
    }
}

pub async fn handle_http(args: ConnectionArgs) {

    let blockchain = &args.blockchain.to_string();

    if blockchain == "ethereum" {
        ethereum::connections::web3_connect(&args.url.to_string(), &args.account.to_string()).await;
    } else {
        println!("❌ blockchain not supported yet");
    }
    
}
