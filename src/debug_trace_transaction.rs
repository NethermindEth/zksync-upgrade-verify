use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// Define a struct to represent the JSON-RPC request
#[derive(Serialize)]
struct RpcRequest {
    jsonrpc: &'static str,
    method: &'static str,
    params: Vec<Value>,
    id: i64,
}

// Define a struct to represent the JSON-RPC response
#[derive(Deserialize)]
struct RpcResponse {
    result: Option<Value>,
    error: Option<RpcError>,
}

// Define a struct to represent the RPC error
#[derive(Deserialize)]
struct RpcError {
    code: i32,
    message: String,
}

/// Make `debug_traceTransaction` JSON-RPC request
//e.g.
//rpc - "https://nd-422-757-666.p2pify.com/0a9d79d93fb2f4a4b1e04695da2b77a7/";
//tx - "0xa5fd3584a815267a84a5686b386d911ed7e53d6c1863ff64a57ef0f7085bd4d7"
pub fn debug_trace_transaction(rpc: &str, tx: &str) {
    // Initialize the reqwest client
    let client = Client::new();

    // Construct the JSON-RPC request
    let request_body = RpcRequest {
        jsonrpc: "2.0",
        method: "debug_traceTransaction",
        params: vec![
            json!(tx),
            json!({"tracer": "prestateTracer", "diffMode": "True"}),
            //json!({ "tracer": "trace" }),
        ],
        id: 1,
    };

    // Send the HTTP POST request to the Ethereum node
    let response = client
        .post(rpc)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request_body).unwrap())
        .send()
        .unwrap();

    // Parse the JSON response
    let rpc_response: RpcResponse = response.json().unwrap();
    // Check for errors
    if let Some(error) = rpc_response.error {
        println!("Error {}: {}", error.code, error.message);
    } else {
        println!("Result: {:?}", rpc_response.result.unwrap());
    }
}
