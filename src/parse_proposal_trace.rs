use crate::parse_storage_diff::parse_storage_diff;

use crate::constants::{GOVERNANCE, TRANSPARENT_OPERATION_SCHEDULED};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{
        BlockId, BlockOverrides, Bytes, GethDebugBuiltInTracerConfig, GethDebugBuiltInTracerType,
        GethDebugTracerConfig, GethDebugTracerType, GethDebugTracingCallOptions, GethTrace,
        PreStateConfig, H160, H256,
    },
};
use std::str::FromStr;

// Trace call upgrade proposal transaction and return Diff
async fn get_pre_tracer_diff(
    provider: &Provider<Http>,
    tx_hash: &str,
) -> Result<(GethTrace, BlockId), String> {
    // Get upgrade proposal transaction
    let tx_hash: H256 = H256::from_str(tx_hash).map_err(|err| err.to_string())?;
    let in_tx = provider
        .get_transaction_receipt(tx_hash)
        .await
        .map_err(|err| err.to_string())?
        .ok_or("Transaction not found")?;

    // Check if it is upgrade proposal transaction
    // Governance address should emit TransparentOperationScheduled (index_topic_1 bytes32 _id, uint256 delay, tuple _operation)
    if in_tx.logs.len() == 0
        || in_tx.logs[0].address != GOVERNANCE
        || in_tx.logs[0].topics[0] != TRANSPARENT_OPERATION_SCHEDULED
    {
        return Err("Wrong event topic".to_string());
    }

    // to governance
    let to = GOVERNANCE;
    // from multisig
    let from = H160::from_str("0x4e4943346848c4867F81dFb37c4cA9C5715A7828").unwrap();

    // Get calldata from event log
    let mut call_data = in_tx.logs[0].data[32..].to_vec();
    call_data[31] = 0x20;
    let call = format!("0x74da756b{}", ethers::utils::hex::encode(call_data));
    let data: Bytes = Bytes::from_str(&call).unwrap();
    let call_tx = ethers::types::TransactionRequest::default()
        .to(to)
        .from(from)
        .data(data);

    // Override block timestamp
    let mut block_overides = BlockOverrides::default();
    // Set maximum i32 timestamp to pass any time check
    block_overides.time = Some((i32::MAX).into());
    // Select PreStateTracer
    let mut opts = GethDebugTracingCallOptions::default();
    opts.block_overrides = Some(block_overides);
    opts.tracing_options.tracer_config = Some(GethDebugTracerConfig::BuiltInTracer(
        GethDebugBuiltInTracerConfig::PreStateTracer(PreStateConfig {
            diff_mode: Some(true),
        }),
    ));
    opts.tracing_options.tracer = Some(GethDebugTracerType::BuiltInTracer(
        GethDebugBuiltInTracerType::PreStateTracer,
    ));
    // Get block number
    let block_id = BlockId::Number(
        (in_tx
            .block_number
            .ok_or_else(|| "No block number".to_string()))?
        .into(),
    );

    // Get debug trace
    let diff = provider
        .debug_trace_call(call_tx, Some(block_id), opts)
        .await
        .map_err(|err| err.to_string())?;
    Ok((diff, block_id))
}

// Decode storage
pub async fn parse_proposal_trace(tx_hash: &str, rpc_url: &str) -> Result<(), String> {
    //let tx_hash: H256 = H256::from_str(tx_hash).map_err(|err| err.to_string())?;
    // Create provider
    let provider = Provider::<Http>::try_from(rpc_url).map_err(|err| err.to_string())?;
    // Get debug call trace
    let (diff, block_id) = get_pre_tracer_diff(&provider, tx_hash).await?;
    // Print information from debug trace
    parse_storage_diff(&provider, &diff, block_id).await
}
