use crate::init_upgrade::*;
use ethers::core::abi::AbiDecode;
use ethers::core::types::H256;
use ethers::prelude::*;
use ethers::providers::{Http, Middleware, Provider};
use std::str::FromStr;
use std::{convert::TryFrom, sync::Arc};

pub async fn info(rpc_url: &str, tx_hash: &str) {
    abigen!(
        Multisig,
        r#"[
            function execTransaction(address to, uint256 value, bytes data, uint8 operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address refundReceiver, bytes signature)

            struct Call {address target;uint256 value;bytes data;}
            struct Operation {Call[] calls;bytes32 predecessor;bytes32 salt;}
            function execute(Operation calldata _operation) external payable
            
            struct FacetCut {address facet;uint8 action;bool isFreezable;bytes4[] selectors;}
            struct DiamondCutData {FacetCut[] facetCuts;address initAddress;bytes initCalldata;}
            function executeUpgrade(Diamond.DiamondCutData calldata _diamondCut) external
            
            struct ForceDeployment {bytes32 bytecodeHash;address newAddress;bool callConstructor;uint256 value;bytes input;}
            function forceDeployOnAddresses(ForceDeployment[] calldata _deployParams) external
      ]"#,
    );

    let tx_hash: H256 = H256::from_str(tx_hash).unwrap();

    let client = Provider::<Http>::try_from(rpc_url).expect("could not instantiate HTTP Provider");

    let client = Arc::new(client);
    let tx = client.get_transaction(tx_hash).await.unwrap().unwrap();

    let tx_input = tx.input;
    let decoded = ExecTransactionCall::decode(&tx_input).unwrap();
    //println!("msig call {:?}", decoded);
    //println!("---");

    let tx_input = decoded.data;
    let decoded = ExecuteCall::decode(tx_input).unwrap();
    println!("Governance call {:?}", decoded.operation);
    println!("---");

    //Admin facet selector 0xa9f6d941
    let tx_input = decoded.operation.calls[0].data.clone();
    let decoded = ExecuteUpgradeCall::decode(tx_input).unwrap();
    println!("Admin facet call {:?}", decoded);
    println!("---");

    // call upgrade contract
    // e.g. https://github.com/matter-labs/era-contracts/blob/4aa7006153ad571643342dff22c16eaf4a70fdc1/l1-contracts/contracts/upgrades/Upgrade_v1_4_1.sol
    let tx_input = decoded.diamond_cut.init_calldata;
    let decoded = UpgradeCall::decode(tx_input).unwrap();
    println!("Upgrade init_call {:?}", decoded);
    println!("---");

    // https://github.com/matter-labs/era-contracts/blob/4aa7006153ad571643342dff22c16eaf4a70fdc1/l2-contracts/contracts/L2ContractHelper.sol#L47
    let tx_input = decoded.proposed_upgrade.l_2_protocol_upgrade_tx.data;
    let decoded = ForceDeployOnAddressesCall::decode(tx_input).unwrap();
    println!("l2 protocol upgrade call {:?}", decoded);
    println!("---");
}
