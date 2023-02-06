// src/utils/commands.rs
// author: steinkirch


mod utils;
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
            handle_http};

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();
    let args = CliStruct::parse();
    
    match args.command {
        CliEnum::WS(args) => handle_ws(args).await,
        CliEnum::HTTP(args) => handle_http(args).await,
    }
}
