// src/utils/cli_handler.rs
// author: steinkirch


use crate::{
    utils::commands::ConnectionArgs,
    ethereum::connections::{web3_connect},
};

pub async fn handle_ws(args: ConnectionArgs) -> () {

    web3_connect(url, addresses).await;
    
}

pub async fn handle_http(args: ConnectionArgs) {

    web3_connect(url, addresses).await;
    
}
