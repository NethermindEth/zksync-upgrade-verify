#[macro_use]
extern crate lazy_static;

use clap::{Parser, Subcommand};
mod info;
mod init_upgrade;
mod trace;

use crate::info::info;
use crate::trace::trace;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Trace {
        /// Ethereum JSON-RPC endpoint
        #[arg(long = "rpc-url", short = 'r', required = true)]
        rpc_url: String,
        /// Transaction hash
        #[arg(long = "tx-hash", short = 't', required = true)]
        tx_hash: String,
    },
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
        Commands::Trace { rpc_url, tx_hash } => {
            if let Err(err) = trace(rpc_url, tx_hash).await {
                eprintln!("Main Error: {}", err);
            }
        }
        Commands::Info { rpc_url, tx_hash } => info(rpc_url, tx_hash).await,
    };
}
