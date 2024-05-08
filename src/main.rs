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
mod upgrade_abi_new;
mod upgrade_call_data;

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
    /// The command provides detailed insights into executed updates.
    History {
        /// Ethereum JSON-RPC endpoint
        #[arg(long = "rpc-url", short = 'r', required = true)]
        rpc_url: String,
        /// Transaction hash
        #[arg(long = "tx-hash", short = 't', required = true)]
        tx_hash: String,
    },
    /// Get upgrade proposal info. Run proposal-calldata first and then proposal-trace
    Proposal {
        /// Ethereum JSON-RPC endpoint
        #[arg(long = "rpc-url", short = 'u', required = true)]
        rpc_url: String,
        /// Transaction hash
        #[arg(long = "tx-hash", short = 't', required = true)]
        tx_hash: String,
        /// Set this flag to skip decoding of transaction trace.
        #[arg(long = "skip-trace")]
        skip_trace: bool,
        /// Set this flag to skip decoding of transaction calldata.
        #[arg(long = "skip-calldata")]
        skip_calldata: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::History { rpc_url, tx_hash } => {
            if let Err(err) = parse_upgrade_tx(tx_hash, rpc_url).await {
                eprintln!("Parse upgrade transaction error: {}", err);
            }
        }
        Commands::Proposal {
            rpc_url,
            tx_hash,
            skip_trace,
            skip_calldata,
        } => {
            if !skip_calldata {
                if let Err(err) = parse_proposal_call(tx_hash, rpc_url).await {
                    eprintln!("Parse proposal calldata error: {}", err);
                }
            }
            if !skip_trace {
                if let Err(err) = parse_proposal_trace(tx_hash, rpc_url).await {
                    eprintln!("Parse proposal trace error: {}", err);
                }
            }
        }
    };
}
