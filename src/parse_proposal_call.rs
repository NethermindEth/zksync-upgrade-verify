use crate::constants::{GOVERNANCE, TRANSPARENT_OPERATION_SCHEDULED};
use crate::parse_diamond_cut::{get_dimond_cut_call, parse_diamond_cut_call};
use crate::upgrade_abi::Operation;
use ethers::core::abi::AbiDecode;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::H256,
};
use std::str::FromStr;

pub async fn parse_proposal_call(tx_hash: &str, rpc_url: &str) -> Result<(), String> {
    let tx_hash: H256 = H256::from_str(tx_hash).map_err(|err| err.to_string())?;
    let provider = Provider::<Http>::try_from(rpc_url).map_err(|err| err.to_string())?;
    // Get upgrade transaction
    let tx = provider
        .get_transaction_receipt(tx_hash)
        .await
        .map_err(|err| err.to_string())?
        .ok_or("Transaction not found")?;

    //need to find TransparentOperationScheduled event
    let log = tx
        .logs
        .iter()
        .find(|log| log.address == GOVERNANCE && log.topics[0] == TRANSPARENT_OPERATION_SCHEDULED);

    if log.is_none() {
        return Err("Can't find TransparentOperationScheduled event".to_string());
    };
    let log = log.unwrap();
    let operation: Operation = Operation::decode(&log.data[64..]).unwrap();
    // Check governance calls
    let tx_input = get_dimond_cut_call(&operation)?;
    parse_diamond_cut_call(&tx_input)
}
