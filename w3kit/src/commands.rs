use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fmt;

#[derive(Debug, Parser)]
#[clap(name = "web3toolkit")]
#[clap(about = "A toolkit for the blockchains.")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(arg_required_else_help = true)]
    Ticker(TickerArg),
}

#[derive(Debug, Args)]
pub struct TickerArg {
    pub currencies: String,
    #[clap(short, long)]
    pub exchange: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Status {
    Ongoing,
    Upcoming,
    Ended,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
