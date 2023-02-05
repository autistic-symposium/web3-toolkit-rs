// src/main.rs - author: steinkirch

use clap::Parser;

use commands::Commands;
use near::{ping};

mod near;
mod commands;


fn main() {

    dotenv::dotenv().ok();

    let args = commands::Cli::parse();
    match args.command {
        Commands::Ethereum(arg) => ping("aaaa"),
        Commands::Arbitrum(arg) => ping("aaaa"),
        Commands::Avalanche(arg) => ping("aaaa"),
        Commands::Near(arg) => ping("aaaa"),
        Commands::Optimism(arg) => ping("aaaa"),
        Commands::Polygon(arg) => ping("aaaa"),
        Commands::Solana(arg) => ping("aaaa"),
    }
}
