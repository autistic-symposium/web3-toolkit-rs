// src/utils/cli_handler.rs
// author: steinkirch


use crate::{
    utils::commands::EthereumArg,
    utils::commands::ArbitrumArg,
    utils::commands::AvalancheArg,
    utils::commands::NearArg,
    utils::commands::OptimismArg,
    utils::commands::PolygonArg,
    utils::commands::SolanaArg,
};


pub fn eth_handler(arg: EthereumArg) {

    println!("arg.account: {:?}", arg.account);
    println!("arg.status: {:?}", arg.status);

}


pub fn arbitrum_handler(arg: ArbitrumArg) {

    println!("arg.account: {:?}", arg.account);
    println!("arg.status: {:?}", arg.status);

}


pub fn avax_handler(arg: AvalancheArg) {

    println!("arg.account: {:?}", arg.account);
    println!("arg.status: {:?}", arg.status);

}


pub fn near_handler(arg: NearArg) {

    println!("arg.account: {:?}", arg.account);
    println!("arg.status: {:?}", arg.status);

}


pub fn optimism_handler(arg: OptimismArg) {

    println!("arg.account: {:?}", arg.account);
    println!("arg.status: {:?}", arg.status);

}


pub fn polygon_handler(arg: PolygonArg) {

    println!("arg.account: {:?}", arg.account);
    println!("arg.status: {:?}", arg.status);

}


pub fn solana_handler(arg: SolanaArg) {

    println!("arg.account: {:?}", arg.account);
    println!("arg.status: {:?}", arg.status);

}

