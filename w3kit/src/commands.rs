use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fmt;

#[derive(Debug, Parser)]
#[clap(name = "w3kit")]
#[clap(about = "A toolkit for the blockchains.")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(arg_required_else_help = true)]
    Ethereum(EthereumArg),
    Arbitrum(ArbitrumArg),
    Avalanche(AvalancheArg),
    Near(NearArg),
    Optimism(OptimismArg),
    Polygon(PolygonArg),
    Solana(SolanaArg),
}

#[derive(Debug, Args)]
pub struct EthereumArg {
    pub currencies: String,
    #[clap(short, long)]
    pub exchange: Option<String>,
}


#[derive(Debug, Args)]
pub struct ArbitrumArg {
    pub currencies: String,
    #[clap(short, long)]
    pub exchange: Option<String>,
}

#[derive(Debug, Args)]
pub struct AvalancheArg {
    pub currencies: String,
    #[clap(short, long)]
    pub exchange: Option<String>,
}

#[derive(Debug, Args)]
pub struct NearArg {
    pub currencies: String,
    #[clap(short, long)]
    pub exchange: Option<String>,
}

#[derive(Debug, Args)]
pub struct OptimismArg {
    pub currencies: String,
    #[clap(short, long)]
    pub exchange: Option<String>,
}

#[derive(Debug, Args)]
pub struct PolygonArg {
    pub currencies: String,
    #[clap(short, long)]
    pub exchange: Option<String>,
}

#[derive(Debug, Args)]
pub struct SolanaArg {
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
