# Table of Contents
- [Commands](#commands)
  - [proposal](#proposal)
  - [history](#history)
- [How to Compile](#how-to-compile)

# Commands

## proposal

The proposal command is designed to work with transactions that emit the `TransparentOperationScheduled` event, targeting the upgrade of the `zkSync Era: Diamond Proxy` contract. This command utilizes calldata to execute `debug_trace_call`, displaying contract storage changes after the scheduled upgrade.

When upgrading the zkSync smart contracts following the steps outlined in [this repository](https://github.com/matter-labs/zksync-era/tree/main/infrastructure/protocol-upgrade), the propose-upgrade step generates a transaction hash that emits `TransparentOperationScheduled`. You can retrieve details of that transaction using the `proposal` command.

This command first decodes the calldata from the `TransparentOperationScheduled` event, enabling inspection of the scheduled upgrade parameters. Then, it traces the upgrade proposal transaction, allowing examination of contract storage changes post-upgrade.

Here are some examples of `Governance.scheduleTransparent` transactions that can be inspected using this command:
```
0x325a83b111cd1ebb3bda97426f373342f3f79507ce366ed50c4db69ceaf373a2
0x2ae11bb0f4fa6b712c3444909a75b10552111a11d4253c7a64eae1919bcae153
0x2038c816932cad4a08a5d8a5c7300cf730ff3a3573fadacb5bbc4c3716083316
0x7c85c5e95620970845519878c429d18dc075053b9dfeb5636e7ae342aa443048
```

By `proposal` command, you can gain a comprehensive understanding of both the calldata and storage changes associated with the upgrade proposal transaction.

- `--rpc-url`: Specifies the Ethereum JSON-RPC endpoint.
- `--tx-hash`: Specifies the upgrade proposal transaction hash.
- `--skip_trace`: Set this flag to skip decoding of transaction trace.
- `--skip_calldata`: Set this flag to skip decoding of transaction calldata.

#### How to run

```bash
zksync-upgrade-verify proposal-info --rpc-url https://eth.llamarpc.com --tx-hash 0x2ae11bb0f4fa6b712c3444909a75b10552111a11d4253c7a64eae1919bcae153
```

## history

The `history` command provides detailed insights into executed upgrades. With this command, you can thoroughly examine executed upgrades. It specifically targets already executed upgrade transactions, capable of decoding transactions that invoke `executeUpgrade` on `AdminFacet`, as well as the older style upgrade, `executeUpgradeWithProposalSalt` on `DiamondCutFacet`. The ideal transaction to utilize this command on is one that emits the `NewProtocolVersion` event. To obtain the transaction hashes of past upgrades, you can visit etherscan.io, find the ZkSync Era: Diamond Proxy contract (`0x32400084C286CF3E17e7B677ea9583e60a000324`), and filter its events by `0x4235104f56661fe2e9d2f2a460b42766581bc45ce366c6a30a9f86c8a2b371a7`. Then, call the history function with those transaction hashes.

- `--rpc-url`: Specifies the Ethereum JSON-RPC endpoint.
- `--tx-hash:` Specifies the upgrade transaction hash.

#### How to run

```bash
zksync-upgrade-verify history --rpc-url https://eth.llamarpc.com --tx-hash 0xa5fd3584a815267a84a5686b386d911ed7e53d6c1863ff64a57ef0f7085bd4d7
```

# How to compile

```bash
git clone https://github.com/NethermindEth/zksync-upgrade-verify.git
cd zksync-upgrade-verify
cargo build --release
```
