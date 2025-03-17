mod blockchain;
mod ui;

use blockchain::fetch_transactions;
use clap::{Arg, Command};   
use tokio;
use ui::run_ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Blockchain Transaction Analyzer")
        .version("1.0")
        .author("Your Name")
        .about("Analyzes Ethereum transactions and visualizes them in a CLI")
        .arg(Arg::new("address").short('a').long("address").value_parser(clap::value_parser!(String)).required(true))
        .arg(Arg::new("limit").short('l').long("limit").value_parser(clap::value_parser!(usize)).default_value("10"))
        .get_matches();

    let address = matches.get_one::<String>("address").expect("Address is required");
    let limit = *matches.get_one::<usize>("limit").expect("Limit has a default value");

    let (transactions, graph) = fetch_transactions(address, limit).await?;

    run_ui(address,&transactions, &graph)?;

    Ok(())
}
