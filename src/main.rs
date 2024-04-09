use clap::{Parser, Subcommand};

mod debug_trace_transaction;

use crate::debug_trace_transaction::debug_trace_transaction;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Make `debug_traceTransaction` JSON-RPC request
    TraceTransaction {
        /// Ethereum JSON-RPC endpoint
        #[arg(long = "rpc", required = true)]
        rpc: String,
        /// Transaction hash
        #[arg(required = true)]
        tx: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::TraceTransaction { rpc, tx } => debug_trace_transaction(rpc, tx),
    };
}
