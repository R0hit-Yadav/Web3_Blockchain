use clap::Parser;

/// CLI arguments for the blockchain analyzer
#[derive(Parser, Debug)]
#[command(name = "Blockchain Analyzer", version = "1.0", about = "Analyze Ethereum transactions")]
pub struct Cli {
    /// Ethereum address to analyze
    #[arg(short, long)]
    pub address: String,

    /// Maximum number of transactions to fetch
    #[arg(short, long, default_value_t = 10)]
    pub limit: usize,
}
