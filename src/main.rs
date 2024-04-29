use clap::{Parser, Subcommand};
mod constants;
mod function_signature;
mod l2_contracts_names;
mod parse_diamond_cut_data;
mod parse_proposal_call;
mod parse_proposal_trace;
mod parse_upgrade_tx;
mod slots_names;
mod strings;
mod upgrade_abi;

use crate::parse_proposal_call::parse_proposal_call;
use crate::parse_proposal_trace::parse_proposal_trace;
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
    /// Get upgrade transaction info
    UpgradeInfo {
        /// Ethereum JSON-RPC endpoint
        #[arg(long = "rpc-url", short = 'r', required = true)]
        rpc_url: String,
        /// Transaction hash
        #[arg(long = "tx-hash", short = 't', required = true)]
        tx_hash: String,
    },
    /// Get upgrade proposal info. Run proposal-calldata first and then proposal-trace
    ProposalInfo {
        /// Ethereum JSON-RPC endpoint
        #[arg(long = "rpc-url", short = 'r', required = true)]
        rpc_url: String,
        /// Transaction hash
        #[arg(long = "tx-hash", short = 't', required = true)]
        tx_hash: String,
    },
    /// Get upgrade proposal calldata
    ProposalCalldata {
        /// Ethereum JSON-RPC endpoint
        #[arg(long = "rpc-url", short = 'r', required = true)]
        rpc_url: String,
        /// Transaction hash
        #[arg(long = "tx-hash", short = 't', required = true)]
        tx_hash: String,
    },
    /// Emulate upgrade proposal transaction and get storage slots changes
    ProposalTrace {
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
        Commands::UpgradeInfo { rpc_url, tx_hash } => {
            if let Err(err) = parse_upgrade_tx(tx_hash, rpc_url).await {
                eprintln!("Parse upgrade transaction error: {}", err);
            }
        }
        Commands::ProposalInfo { rpc_url, tx_hash } => {
            if let Err(err) = parse_proposal_call(tx_hash, rpc_url).await {
                eprintln!("Parse proposal calldata error: {}", err);
            }
            if let Err(err) = parse_proposal_trace(tx_hash, rpc_url).await {
                eprintln!("Parse proposal trace error: {}", err);
            }
        }
        Commands::ProposalCalldata { rpc_url, tx_hash } => {
            if let Err(err) = parse_proposal_call(tx_hash, rpc_url).await {
                eprintln!("Parse proposal calldata error: {}", err);
            }
        }
        Commands::ProposalTrace { rpc_url, tx_hash } => {
            if let Err(err) = parse_proposal_trace(tx_hash, rpc_url).await {
                eprintln!("Parse proposal trace error: {}", err);
            }
        }
    };
}
