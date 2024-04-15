use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

// Define a struct to represent the JSON-RPC request
#[derive(Serialize)]
struct RpcRequest {
    jsonrpc: &'static str,
    method: &'static str,
    params: Vec<Value>,
    id: i64,
}

// Define a struct to represent the JSON-RPC response
#[derive(Deserialize, Debug)]
struct RpcResponse {
    result: Option<Value>,
    error: Option<RpcError>,
}

// Define a struct to represent the RPC error
#[derive(Deserialize, Debug)]
struct RpcError {
    code: i32,
    message: String,
}

// account diff data
#[derive(Debug, Serialize, Deserialize)]
struct DiffData {
    balance: Option<String>,
    nonce: Option<u64>,
    code: Option<String>,
    storage: Option<HashMap<String, String>>,
}

lazy_static! {
    static ref DIAMOND_PROXY_LAYOUT: HashMap<String, (&'static str, &'static str)> = {
        let mut map = HashMap::new();
        map.insert(
            format!("0x{:0>64x}", 0),
            ("__DEPRECATED_diamondCutStorage", "uint256[7]"),
        );
        map.insert(format!("0x{:0>64x}", 7), ("governor", "address"));
        map.insert(format!("0x{:0>64x}", 8), ("pendingGovernor", "address"));
        map.insert(
            format!("0x{:0>64x}", 9),
            ("validators", "mapping(address => bool)"),
        );
        map.insert(
            format!("0x{:0>64x}", 10),
            ("verifier", "contract IVerifier"),
        );
        map.insert(
            format!("0x{:0>64x}", 11),
            ("totalBatchesExecuted", "uint256"),
        );
        map.insert(
            format!("0x{:0>64x}", 12),
            ("totalBatchesVerified", "uint256"),
        );
        map.insert(
            format!("0x{:0>64x}", 13),
            ("totalBatchesCommitted", "uint256"),
        );
        map.insert(
            format!("0x{:0>64x}", 14),
            ("storedBatchHashes", "mapping(uint256 => bytes32)"),
        );
        map.insert(
            format!("0x{:0>64x}", 15),
            ("l2LogsRootHashes", "mapping(uint256 => bytes32)"),
        );
        map.insert(
            format!("0x{:0>64x}", 16),
            ("priorityQueue", "struct PriorityQueue.Queue"),
        );
        map.insert(
            format!("0x{:0>64x}", 19),
            ("__DEPRECATED_allowList", "address"),
        );
        map.insert(
            format!("0x{:0>64x}", 20),
            ("verifierParams", "struct VerifierParams"),
        );
        map.insert(
            format!("0x{:0>64x}", 23),
            ("l2BootloaderBytecodeHash", "bytes32"),
        );
        map.insert(
            format!("0x{:0>64x}", 24),
            ("l2DefaultAccountBytecodeHash", "bytes32"),
        );
        map.insert(format!("0x{:0>64x}", 25), ("zkPorterIsAvailable", "bool"));
        map.insert(
            format!("0x{:0>64x}", 26),
            ("priorityTxMaxGasLimit", "uint256"),
        );
        map.insert(
            format!("0x{:0>64x}", 27),
            ("__DEPRECATED_upgrades", "struct UpgradeStorage"),
        );
        map.insert(
            format!("0x{:0>64x}", 29),
            (
                "isEthWithdrawalFinalized",
                "mapping(uint256 => mapping(uint256 => bool))",
            ),
        );
        map.insert(
            format!("0x{:0>64x}", 30),
            ("__DEPRECATED_lastWithdrawalLimitReset", "uint256"),
        );
        map.insert(
            format!("0x{:0>64x}", 31),
            ("__DEPRECATED_withdrawnAmountInWindow", "uint256"),
        );
        map.insert(
            format!("0x{:0>64x}", 32),
            (
                "__DEPRECATED_totalDepositedAmountPerUser",
                "mapping(address => uint256)",
            ),
        );
        map.insert(format!("0x{:0>64x}", 33), ("protocolVersion", "uint256"));
        map.insert(
            format!("0x{:0>64x}", 34),
            ("l2SystemContractsUpgradeTxHash", "bytes32"),
        );
        map.insert(
            format!("0x{:0>64x}", 35),
            ("l2SystemContractsUpgradeBatchNumber", "uint256"),
        );
        map.insert(format!("0x{:0>64x}", 36), ("admin", "address"));
        map.insert(format!("0x{:0>64x}", 37), ("pendingAdmin", "address"));
        map.insert(format!("0x{:0>64x}", 38), ("feeParams", "struct FeeParams"));
        map.insert(
            format!("0x{:0>64x}", 39),
            ("blobVersionedHashRetriever", "address"),
        );
        map
    };
}

fn get_slot_name(addr: &str, slot: &str) -> String {
    if addr == "0x32400084c286cf3e17e7b677ea9583e60a000324" {
        if let Some((name, _)) = DIAMOND_PROXY_LAYOUT.get(slot) {
            return name.to_string();
        }
    }
    return slot.to_string();
}

fn get_slot_value(storage: Option<&HashMap<String, String>>, key: &str) -> String {
    storage
        .and_then(|storage| storage.get(key))
        .map(|v| v.to_string())
        .unwrap_or_else(|| "-".to_string())
}

pub async fn trace(rpc_url: &str, tx_hash: &str) -> Result<(), String> {
    let client = Client::new();
    let request_body = RpcRequest {
        jsonrpc: "2.0",
        method: "debug_traceTransaction",
        params: vec![
            json!(tx_hash),
            json!({"tracer": "prestateTracer", "tracerConfig": {"diffMode": true}}),
        ],
        id: 1,
    };
    let response = client
        .post(rpc_url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|err| err.to_string())?;
    let rpc_response: RpcResponse = response.json().await.map_err(|err| err.to_string())?;
    if let Some(error) = rpc_response.error {
        return Err(format!(
            "JSON-RPC request error: code: {}, message: {}",
            error.code, error.message
        ));
    }
    let map = rpc_response
        .result
        .ok_or("No result in JSON-RPC response")?;
    let map_pre = map["pre"]
        .as_object()
        .ok_or("No pre field in JSON-RPC response")?;
    let map_post = map["post"]
        .as_object()
        .ok_or("No post field in JSON-RPC response")?;

    for (addr, val) in map_post {
        if let Some(old) = map_pre.get(addr) {
            println!("\x1b[38;5;190mAccount\x1b[0m {}", addr);
            let new_data: DiffData =
                serde_json::from_value(val.clone()).map_err(|err| err.to_string())?;
            let old_data: DiffData =
                serde_json::from_value(old.clone()).map_err(|err| err.to_string())?;
            if let Some(balance) = new_data.balance {
                println!(
                    "   \x1b[38;5;33mBalance\x1b[38;5;66m {}\x1b[38;5;33m -> \x1b[38;5;69m{}\x1b[0m",
                    old_data.balance.unwrap_or( "-".to_string() ),
                    balance
                );
            }
            if let Some(nonce) = new_data.nonce {
                println!("   Nonce   {} -> {}", old_data.nonce.unwrap_or(0), nonce);
            }
            if new_data.code.is_some() {
                unreachable!(); // code only for new account
            }
            if new_data.storage.is_some() {
                println!("  Storage");
                let old_storage = old_data.storage.as_ref();
                if let Some(new_storage) = new_data.storage.as_ref() {
                    for (slot, value) in new_storage {
                        let slot_name = get_slot_name(addr, slot);
                        let old_value = get_slot_value(old_storage, &slot);
                        println!(
                            "  \x1b[38;5;33m{}\x1b[0m:\n      {} ->\n      {}",
                            slot_name, old_value, value
                        );
                    }
                }
                let new_storage = new_data.storage.as_ref();
                if let Some(old_storage) = old_data.storage.as_ref() {
                    for (slot, value) in old_storage {
                        let new_value = get_slot_value(new_storage, &slot);
                        if new_value == "-" {
                            println!(
                                "  \x1b[38;5;33m{}\x1b[0m:\n      {} ->\n      -",
                                slot, value
                            );
                        }
                    }
                }
            }
        } else {
            println!("New Account {}", addr);
        }
    }
    Ok(())
}
