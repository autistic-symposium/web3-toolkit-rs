// src/utils/cli_handler.rs
// author: steinkirch


use crate::{
    utils::commands::ConnectionArgs,
    ethereum,
};

pub async fn handle_ws(args: ConnectionArgs) {

    let blockchain = &args.blockchain.to_string();
    let url = &args.url.to_string();
    let account = &args.account.to_string();

    if blockchain == "ethereum" {
        ethereum::connections::web3_connect(blockchain, url, account).await;
    } else {
        println!("❌ blockchain not supported yet");
    }
}

pub async fn handle_http(args: ConnectionArgs) {

    let blockchain = &args.blockchain.to_string();
    let url = &args.url.to_string();
    let account = &args.account.to_string();

    if blockchain == "ethereum" {
        ethereum::connections::web3_connect(blockchain, url, account).await;
    } else {
        println!("❌ blockchain not supported yet");
    }
    
}
