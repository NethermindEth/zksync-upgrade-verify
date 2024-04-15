## Commands

### info

This command decodes transaction call data using the specified Ethereum JSON-RPC endpoint and transaction hash.

- `--rpc-url`: Specifies the Ethereum JSON-RPC endpoint.
- `--tx-hash`: Specifies the transaction hash.

#### Example

```bash
cargo run info --rpc-url https://nd-422-757-666.p2pify.com/0a9d79d93fb2f4a4b1e04695da2b77a7/ --tx-hash 0xa5fd3584a815267a84a5686b386d911ed7e53d6c1863ff64a57ef0f7085bd4d7
```


### trace

This command runs `debug_traceTransaction` on the specified transaction using the provided Ethereum JSON-RPC endpoint.

- `--rpc-url`: Specifies the Ethereum JSON-RPC endpoint.
- `--tx-hash`: Specifies the transaction hash.

#### Example

```bash
cargo run trace --rpc-url https://nd-422-757-666.p2pify.com/0a9d79d93fb2f4a4b1e04695da2b77a7/ --tx-hash 0xa5fd3584a815267a84a5686b386d911ed7e53d6c1863ff64a57ef0f7085bd4d7
```