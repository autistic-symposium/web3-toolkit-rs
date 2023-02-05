// src/utils/commands.rs
// author: steinkirch

mod near;
mod utils;

use clap::Parser;

use utils::{CliEnum, 
            CliStruct,
            eth_handler, 
            arbitrum_handler, 
            avax_handler, 
            near_handler, 
            optimism_handler, 
            polygon_handler, 
            solana_handler};


fn main() {

    dotenv::dotenv().ok();

    let args = CliStruct::parse();
    match args.command {
        CliEnum::Ethereum(arg) => eth_handler(arg),
        CliEnum::Arbitrum(arg) => arbitrum_handler(arg),
        CliEnum::Avalanche(arg) => avax_handler(arg),
        CliEnum::Near(arg) =>  near_handler(arg),
        CliEnum::Optimism(arg) => optimism_handler(arg),
        CliEnum::Polygon(arg) => polygon_handler(arg),
        CliEnum::Solana(arg) => solana_handler(arg),
    }
}
