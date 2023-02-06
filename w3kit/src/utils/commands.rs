// src/utils/commands.rs
// author: steinkirch

use clap::{Args, Parser, Subcommand};


#[derive(Debug, Parser)]
#[clap(name = "w3kit")]
#[clap(about = "🕹✨ a rusty toolkit for several blockchains ✨🕹")]
pub struct CliStruct {
    #[clap(subcommand)]
    pub command: CliEnum,
}


#[derive(Debug, Subcommand)]
pub enum CliEnum {
    #[clap(arg_required_else_help = true)]
    /// Test a websocket connection to a given blockchain
    Ws(ConnectionArgs),
    #[clap(arg_required_else_help = true)]
    /// Test an http connection to a given blockchain
    Http(ConnectionArgs),
    #[clap(arg_required_else_help = true)]
    /// Get an account balance from a given blockchain
    Account(AccountArgs),
}


#[derive(Debug, Args)]
pub struct ConnectionArgs {
    #[clap(short, long)]
    /// The blockchain to connect to
    pub blockchain: String,
}


#[derive(Debug, Args)]
pub struct AccountArgs {
    #[clap(short, long)]
    /// The blockchain to connect to
    pub blockchain: String,
    #[clap(short, long)]
    /// The account to be fetched 
    pub account: String,
}

