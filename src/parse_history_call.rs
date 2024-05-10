use crate::constants::{GOVERNANCE, ZKSYNC_ERA};

use crate::parse_diamond_cut::{get_dimond_cut_call, parse_diamond_cut_call};

use crate::upgrade_abi::{ExecTransactionCall, ExecuteCall};
use ethers::core::abi::AbiDecode;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::H256,
};
use std::str::FromStr;

// Get upgrate info
pub async fn parse_history_call(tx_hash: &str, rpc_url: &str) -> Result<(), String> {
    let tx_hash: H256 = H256::from_str(tx_hash).map_err(|err| err.to_string())?;
    let provider = Provider::<Http>::try_from(rpc_url).map_err(|err| err.to_string())?;
    // Get upgrade transaction
    let tx = provider
        .get_transaction(tx_hash)
        .await
        .map_err(|err| err.to_string())?
        .ok_or("Transaction not found")?;
    // Decode transaction to multisig 0x4e4943346848c4867F81dFb37c4cA9C5715A7828
    let tx_input = tx.input;
    let decoded = ExecTransactionCall::decode(&tx_input)
        .map_err(|err| format!("ExecTransactionCall error: {}", err.to_string()))?;
    // check call address
    let tx_input = if decoded.to == ZKSYNC_ERA {
        decoded.data
    } else if decoded.to == GOVERNANCE {
        // Decode transaction to governance
        let tx_input = decoded.data;
        let decoded = ExecuteCall::decode(tx_input)
            .map_err(|err| format!("ExecuteCall error: {}", err.to_string()))?;
        // Get dimond cut calldata
        get_dimond_cut_call(&decoded.operation)?
    } else {
        return Err(format!("Call to unknown contract: {:?}", decoded.to));
    };

    parse_diamond_cut_call(&tx_input)
}
