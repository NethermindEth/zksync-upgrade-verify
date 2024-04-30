# Table of Contents
- [How to use with zkSync Protocol Upgrade Tool](#how-to-use-with-zksync-protocol-upgrade-tool)
- [Commands](#commands)
  - [proposal](#proposal)
  - [history](#history)
- [How to Compile](#how-to-compile)

# How to use with zkSync Protocol Upgrade Tool

You should work with [Protocol Upgrade Tool](https://github.com/matter-labs/zksync-era/tree/main/infrastructure/protocol-upgrade#protocol-upgrade-tool) as usual. After you generate a proposal  and run [`propose-upgrade`](https://github.com/matter-labs/zksync-era/tree/main/infrastructure/ ) you recive output:
```
Proposing upgrade for protocolVersion <VERSION>
Transaction hash: <TX_HASH>
Transaction is executed
```
After that you can use TX_HASH to inspect the proposed upgrade running the tool:
```bash
zksync-upgrade-verify proposal --rpc-url https://eth.llamarpc.com --tx-hash <TX_HASH>
```
If you are satisfied with the upgrade proposal, you can proceed with the upgrade process by executing `execute-upgrade`. Alternatively, you can cancel the upgrade using `cancel-upgrade`

# Commands

## proposal

The proposal command is designed to work with transactions that emit the `TransparentOperationScheduled` event, targeting the upgrade of the `zkSync Era: Diamond Proxy` contract. This command utilizes calldata to execute `debug_trace_call`, displaying contract storage changes after the scheduled upgrade.

When upgrading the zkSync Protocol following the steps outlined in [this repository](https://github.com/matter-labs/zksync-era/tree/main/infrastructure/protocol-upgrade), the `propose-upgrade` step executes the `proposeTransparentUpgrade` transaction on L1 and outputs its hash. Using that hash, you can retrieve details of  the proposed upgrade using the `proposal` command.\

This command first decodes the calldata from the `TransparentOperationScheduled` event, enabling inspection of the scheduled upgrade parameters. Then, it traces the upgrade proposal transaction, allowing examination of contract storage changes post-upgrade.

Here are some examples of `Governance.scheduleTransparent` transactions that can be inspected using this command:
```
0x325a83b111cd1ebb3bda97426f373342f3f79507ce366ed50c4db69ceaf373a2 // refers to the v1.4.2 enchancement upgrade. New_protocol version: 22
0x2ae11bb0f4fa6b712c3444909a75b10552111a11d4253c7a64eae1919bcae153 // refers to the protodanksharding upgrade. New_protocol version: 21
0x2038c816932cad4a08a5d8a5c7300cf730ff3a3573fadacb5bbc4c3716083316 // refers to the fee-model-and-1.4.1 20 upgrade. New_protocol version: 20
0x7c85c5e95620970845519878c429d18dc075053b9dfeb5636e7ae342aa443048 // refers to the allowlist-removal upgrade. New_protocol version: 19
```

- `--rpc-url`: Specifies the Ethereum JSON-RPC endpoint.
- `--tx-hash`: Specifies the upgrade proposal transaction hash.
- `--skip_trace`: Set this flag to skip decoding of transaction trace.
- `--skip_calldata`: Set this flag to skip decoding of transaction calldata.

#### How to run

```bash
zksync-upgrade-verify proposal --rpc-url https://eth.llamarpc.com --tx-hash 0x2ae11bb0f4fa6b712c3444909a75b10552111a11d4253c7a64eae1919bcae153
```

## history

The `history` command provides detailed insights into executed upgrades. With this command, you can thoroughly examine executed upgrades. It specifically targets already executed upgrade transactions, capable of decoding transactions that invoke `executeUpgrade` on `AdminFacet`, as well as the older style upgrade, `executeUpgradeWithProposalSalt` on `DiamondCutFacet`. The ideal transaction to utilize this command on is one that emits the `NewProtocolVersion` event. To obtain the transaction hashes of past upgrades, you can visit etherscan.io, find the ZkSync Era: Diamond Proxy contract (`0x32400084C286CF3E17e7B677ea9583e60a000324`), and filter its events by `0x4235104f56661fe2e9d2f2a460b42766581bc45ce366c6a30a9f86c8a2b371a7`. Then, call the history function with those transaction hashes.

Here are some examples of `executeUpgrade` transactions that can be inspected using this command:
```
0xc78a986be023f367f121c06fa9662ef950ad76f2cfe9397693f63de6c5959c61 // refers to the v1.4.2 enchancement upgrade. New_protocol version: 22
0xa5fd3584a815267a84a5686b386d911ed7e53d6c1863ff64a57ef0f7085bd4d7 // refers to the protodanksharding upgrade. New_protocol version: 21
0x937dd21a05142c02159170dafb1bbaaa145ae7bd2c29bf512534fbec9ff801ab // refers to the fee-model-and-1.4.1 20 upgrade. New_protocol version: 20
0x2200e7109d3abbb74cb03144fea37f7227188e1fcaba4538bd9dfa3fa17cca02 // refers to the allowlist-removal upgrade. New_protocol version: 19
```

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
