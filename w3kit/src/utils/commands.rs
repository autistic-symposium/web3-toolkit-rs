// src/utils/commands.rs
// author: steinkirch

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fmt;


#[derive(Debug, Parser)]
#[clap(name = "w3kit")]
#[clap(about = "🕹✨ a rusty toolkit for the blockchains ✨🕹")]
pub struct CliStruct {
    #[clap(subcommand)]
    pub blockchain: CliEnum,
}


#[derive(Debug, Subcommand)]
pub enum CliEnum {
    #[clap(arg_required_else_help = true)]
    /// Run commands on the Arbitrum blockchain
    Arbitrum(ArbitrumArg),
    #[clap(arg_required_else_help = true)]
    /// Run commands on the Avalanche blockchain
    Avalanche(AvalancheArg),
    #[clap(arg_required_else_help = true)]
    /// Run commands on the Ethereum blockchain
    Ethereum(EthereumArg),
    #[clap(arg_required_else_help = true)]
    /// Run commands on the Near blockchain
    Near(NearArg),
    #[clap(arg_required_else_help = true)]
    /// Run commands on the Polygon blockchain
    Polygon(PolygonArg),
    #[clap(arg_required_else_help = true)]
    /// Run commands on the Optimism blockchain
    Optimism(OptimismArg),
    #[clap(arg_required_else_help = true)]
    /// Run commands on the Solana blockchain
    Solana(SolanaArg),
}


#[derive(Debug, Args)]
pub struct EthereumArg {
    #[clap(short, long)]
    pub status: Option<String>,
    #[clap(short, long)]
    pub account: Option<String>,
}


#[derive(Debug, Args)]
pub struct ArbitrumArg {
    #[clap(short, long)]
    pub status: Option<String>,
    #[clap(short, long)]
    pub account: Option<String>,
}


#[derive(Debug, Args)]
pub struct AvalancheArg {
    #[clap(short, long)]
    pub status: Option<String>,
    #[clap(short, long)]
    pub account: Option<String>,
}


#[derive(Debug, Args)]
pub struct NearArg {
    #[clap(short, long)]
    pub status: Option<String>,
    #[clap(short, long)]
    pub account: Option<String>,
}


#[derive(Debug, Args)]
pub struct OptimismArg {
    #[clap(short, long)]
    pub status: Option<String>,
    #[clap(short, long)]
    pub account: Option<String>,
}


#[derive(Debug, Args)]
pub struct PolygonArg {
    #[clap(short, long)]
    pub status: Option<String>,
    #[clap(short, long)]
    pub account: Option<String>,
}


#[derive(Debug, Args)]
pub struct SolanaArg {
    #[clap(short, long)]
    pub status: Option<String>,
    #[clap(short, long)]
    pub account: Option<String>,
}

