// src/utils/commands.rs
// author: steinkirch


mod utils;
mod near;

use clap::Parser;

use utils::{CliEnum, 
            CliStruct,
            handle_ws, 
            handle_http,
            };


fn main() {

    dotenv::dotenv().ok();

    let args = CliStruct::parse();
    match args.command {
        CliEnum::WS(args) => handle_ws(args),
        CliEnum::HTTP(args) => handle_http(args),
    }
}
