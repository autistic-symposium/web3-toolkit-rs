// src/utils/commands.rs
// author: steinkirch

mod utils;
mod market;
mod ethereum;
mod arbitrum;
mod avalanche;
mod optimism;
mod near;
mod polygon;
mod solana;

use clap::Parser;

use utils::{CliEnum, 
            CliStruct,
            handle_ws,
            handle_http,
            handle_account,
            handle_coin};

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();
    let args = CliStruct::parse();
    
    match args.command {
        CliEnum::Ws(args) => handle_ws(args).await,
        CliEnum::Http(args) => handle_http(args).await,
        CliEnum::Account(args) => handle_account(args).await,
        CliEnum::Coin(args) => handle_coin(args).await,
    }
}
