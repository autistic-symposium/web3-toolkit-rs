// src/utils/cli_handler.rs
// author: steinkirch


use crate::{
    utils::commands::ConnectionArgs,
};



pub fn handle_ws(args: ConnectionArgs) {

    println!("arg.blockchain: {:?}", args.blockchain);
    println!("arg.url: {:?}", args.url);
    
}

pub fn handle_http(args: ConnectionArgs) {

    println!("arg.blockchain: {:?}", args.blockchain);
    println!("arg.url: {:?}", args.url);
    
}
