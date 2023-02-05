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
        Commands::Ticker(ticker_arg) => ping("aaaa"),
    }
}
