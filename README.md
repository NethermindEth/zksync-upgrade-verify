# Table of Contents
- [Commands](#commands)
  - [proposal-calldata](#proposal-calldata)
  - [proposal-trace](#proposal-trace)
  - [proposal-info](#proposal-info)
  - [upgrade-info](#upgrade-info)
- [How to Compile](#how-to-compile)

# Commands

## proposal-calldata

The proposal-calldata command is designed to work with transactions that emit `TransparentOperationScheduled` and aim to upgrade the `zkSync Era: Diamond Proxy` contract. It attempts to parse the `executeUpgrade` function call to `AdminFacet`.

With this command, you can decode the calldata from the `TransparentOperationScheduled` event and view the scheduled update parameters. This command decodes the calldata of a proposal upgrade transaction using the specified Ethereum JSON-RPC endpoint and transaction hash.

 `--rpc-url`: Specifies the Ethereum JSON-RPC endpoint.
 `--tx-hash`: Specifies the upgrade transaction hash.

#### How to run

```bash
zksync-upgrade-verify proposal-calldata --rpc-url https://eth.llamarpc.com --tx-hash 0x2ae11bb0f4fa6b712c3444909a75b10552111a11d4253c7a64eae1919bcae153
```

## proposal-trace

Similar to `proposal-calldata`, the `proposal-trace` command also operates with transactions that emit `TransparentOperationScheduled` and target the upgrade of the `zkSync Era: Diamond Proxy`contract. However, it utilizes calldata to execute `debug_trace_call` and display contract storage changes after the scheduled update.

With this command, you can decode storage changes using calldata from the `TransparentOperationScheduled` event and observe the scheduled update parameters. This command parses the upgrade proposal transaction trace from `debug_trace_call` using the specified Ethereum JSON-RPC endpoint and transaction hash.

- `--rpc-url`: Specifies the Ethereum JSON-RPC endpoint.
- `--tx-hash`: Specifies the upgrade transaction hash.

#### How to run

```bash
zksync-upgrade-verify proposal-trace --rpc-url https://eth.llamarpc.com --tx-hash 0x2ae11bb0f4fa6b712c3444909a75b10552111a11d4253c7a64eae1919bcae153
```

## proposal-info

The proposal-info command combines the functionalities of the two previous commands. It first invokes `proposal-calldata` to decode the calldata from the `TransparentOperationScheduled` event, allowing you to inspect the scheduled update parameters. Then, it proceeds to execute `proposal-trace`, which traces the upgrade proposal transaction using `debug_trace_call`, enabling you to examine contract storage changes after the scheduled update.

By utilizing `proposal-info`, you can gain a comprehensive understanding of both the calldata and storage changes associated with the upgrade proposal transaction.

- `--rpc-url`: Specifies the Ethereum JSON-RPC endpoint.
- `--tx-hash`: Specifies the upgrade transaction hash.

#### How to run

```bash
zksync-upgrade-verify proposal-info --rpc-url https://eth.llamarpc.com --tx-hash 0x2ae11bb0f4fa6b712c3444909a75b10552111a11d4253c7a64eae1919bcae153
```

## upgrade-info

The `upgrade-info` command provides detailed insights into executed updates. With this command, you can thoroughly examine executed upgrades. It specifically targets already executed upgrade transactions, capable of decoding transactions that invoke `executeUpgrade` on `AdminFacet`, as well as the older style update, `executeUpgradeWithProposalSalt` on `DiamondCutFacet`. The ideal transaction to utilize this command on is one that emits the `NewProtocolVersion` event.

This command decodes the calldata of an upgrade transaction and analyzes the transaction trace from trace_replay_transaction using the specified Ethereum JSON-RPC endpoint and transaction hash.

- `--rpc-url`: Specifies the Ethereum JSON-RPC endpoint.
- `--tx-hash:` Specifies the upgrade transaction hash.

#### How to run

```bash
zksync-upgrade-verify upgrade-info --rpc-url https://eth.llamarpc.com --tx-hash 0xa5fd3584a815267a84a5686b386d911ed7e53d6c1863ff64a57ef0f7085bd4d7
```

# How to compile

```bash
git clone https://github.com/NethermindEth/zksync-upgrade-verify.git
cd zksync-upgrade-verify
cargo build --release
```
