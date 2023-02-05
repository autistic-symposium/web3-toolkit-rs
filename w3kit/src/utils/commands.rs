// src/utils/commands.rs
// author: steinkirch

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fmt;


#[derive(Debug, Parser)]
#[clap(name = "w3kit")]
#[clap(about = "🕹✨ a rusty toolkit for the blockchains ✨🕹")]
pub struct CliStruct {
    #[clap(subcommand)]
    pub command: CliEnum,
}


#[derive(Debug, Subcommand)]
pub enum CliEnum {
    #[clap(arg_required_else_help = true)]
    Arbitrum(ArbitrumArg),
    #[clap(arg_required_else_help = true)]
    Avalanche(AvalancheArg),
    #[clap(arg_required_else_help = true)]
    Ethereum(EthereumArg),
    #[clap(arg_required_else_help = true)]
    Near(NearArg),
    #[clap(arg_required_else_help = true)]
    Polygon(PolygonArg),
    #[clap(arg_required_else_help = true)]
    Optimism(OptimismArg),
    #[clap(arg_required_else_help = true)]
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

