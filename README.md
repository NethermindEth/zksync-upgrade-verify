# Commands

## upgrade-info

This command decodes the calldata of an upgrade transaction and parses the transaction trace from `trace_replay_transaction` using the specified Ethereum JSON-RPC endpoint and transaction hash.

- `--rpc-url`: Specifies the Ethereum JSON-RPC endpoint.
- `--tx-hash`: Specifies the upgrade transaction hash.

#### How to run

```bash
cargo run upgrade-info --rpc-url https://eth.llamarpc.com --tx-hash 0xa5fd3584a815267a84a5686b386d911ed7e53d6c1863ff64a57ef0f7085bd4d7
```

## proposal-calldata

This command decodes the calldata of a proposal upgrade transaction using the specified Ethereum JSON-RPC endpoint and transaction hash.

- `--rpc-url`: Specifies the Ethereum JSON-RPC endpoint.
- `--tx-hash`: Specifies the upgrade transaction hash.

#### How to run

```bash
cargo run proposal-calldata --rpc-url https://eth.llamarpc.com --tx-hash 0x2ae11bb0f4fa6b712c3444909a75b10552111a11d4253c7a64eae1919bcae153
```

## proposal-trace

This command parses the upgrade proposal transaction trace from `debug_trace_call` using the specified Ethereum JSON-RPC endpoint and transaction hash.

- `--rpc-url`: Specifies the Ethereum JSON-RPC endpoint.
- `--tx-hash`: Specifies the upgrade transaction hash.

#### How to run

```bash
cargo run proposal-trace --rpc-url https://eth.llamarpc.com --tx-hash 0x2ae11bb0f4fa6b712c3444909a75b10552111a11d4253c7a64eae1919bcae153
```

## proposal-info

This command first calls `proposal-calldata` and then `proposal-trace`.

- `--rpc-url`: Specifies the Ethereum JSON-RPC endpoint.
- `--tx-hash`: Specifies the upgrade transaction hash.

#### How to run

```bash
cargo run proposal-info --rpc-url https://eth.llamarpc.com --tx-hash 0x2ae11bb0f4fa6b712c3444909a75b10552111a11d4253c7a64eae1919bcae153
```