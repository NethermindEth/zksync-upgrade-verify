use clap::{Parser, Subcommand};
mod l2_contracts_names;
mod parse_upgrade_tx;
mod slots_names;
mod upgrade_abi;

use crate::parse_upgrade_tx::parse_upgrade_tx;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get upgrade info
    Info {
        /// Ethereum JSON-RPC endpoint
        #[arg(long = "rpc-url", short = 'r', required = true)]
        rpc_url: String,
        /// Transaction hash
        #[arg(long = "tx-hash", short = 't', required = true)]
        tx_hash: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Info { rpc_url, tx_hash } => {
            if let Err(err) = parse_upgrade_tx(tx_hash, rpc_url).await {
                eprintln!("Parse upgrade transaction error: {}", err);
            }
        }
    };
}
