use crate::parse_storage_diff::parse_storage_diff;
use ethers::types::{
    BlockId, GethDebugBuiltInTracerConfig, GethDebugBuiltInTracerType, GethDebugTracerConfig,
    GethDebugTracerType, GethDebugTracingOptions, GethTrace,
    PreStateConfig,
};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::H256,
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

    // Select PreStateTracer
    let mut opts = GethDebugTracingOptions::default();
    opts.tracer_config = Some(GethDebugTracerConfig::BuiltInTracer(
        GethDebugBuiltInTracerConfig::PreStateTracer(PreStateConfig {
            diff_mode: Some(true),
        }),
    ));
    opts.tracer = Some(GethDebugTracerType::BuiltInTracer(
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
        .debug_trace_transaction(tx_hash, opts)
        .await
        .map_err(|err| err.to_string())?;
    Ok((diff, block_id))
}

// Get upgrate info
pub async fn parse_history_trace(tx_hash: &str, rpc_url: &str) -> Result<(), String> {
    //let tx_hash: H256 = H256::from_str(tx_hash).map_err(|err| err.to_string())?;
    // Create provider
    let provider = Provider::<Http>::try_from(rpc_url).map_err(|err| err.to_string())?;
    // Get debug call trace
    let (diff, block_id) = get_pre_tracer_diff(&provider, tx_hash).await?;

    parse_storage_diff(&provider, &diff, block_id).await
}
