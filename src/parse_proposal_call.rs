use crate::constants::ZKSYNC_ERA;
use crate::parse_diamond_cut_data::parse_diamond_cut_data;
use crate::upgrade_abi::{ExecuteUpgradeCall, ExecuteUpgradeWithProposalSaltCall, Operation};
use ethers::core::abi::AbiDecode;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{H160, H256},
};
use std::str::FromStr;

use crate::function_signature::FunctionSignature;

pub async fn parse_proposal_call(tx_hash: &str, rpc_url: &str) -> Result<(), String> {
    let tx_hash: H256 = H256::from_str(tx_hash).map_err(|err| err.to_string())?;
    let provider = Provider::<Http>::try_from(rpc_url).map_err(|err| err.to_string())?;
    // Get upgrade transaction
    let tx = provider
        .get_transaction_receipt(tx_hash)
        .await
        .map_err(|err| err.to_string())?
        .ok_or("Transaction not found")?;

    //need to find topic
    let mut pos = None;
    for i in 0..tx.logs.len() {
        if tx.logs[i].address
            == H160::from_str("0x0b622a2061eaccae1c664ebc3e868b8438e03f61").unwrap()
            && tx.logs[i].topics[0]
                == H256::from_str(
                    "0x23bc9f5dc037eb49c162fd08c2a4d43dfe70063149e140d502273168da0a0625",
                )
                .unwrap()
        {
            pos = Some(i);
            break;
        };
    }

    if pos.is_none() {
        return Err("Wrong topic".to_string());
    };
    let pos = pos.unwrap();
    let decoded: Operation = Operation::decode(&tx.logs[pos].data[64..]).unwrap();
    // Check governance calls
    if decoded.calls[0].target != ZKSYNC_ERA
        || decoded.calls.len() != 1
        || decoded.calls[0].data.sig() != [0xa9, 0xf6, 0xd9, 0x41]
    {
        return Err(format!("Unexpected governance call: {:?}", decoded));
    }

    let tx_input = decoded.calls[0].data.clone();
    // Decode transaction to zkSync Era Diamond Proxy
    let diamond_cut = match tx_input.sig() {
        //AdminFacet - new
        [169, 246, 217, 65] => {
            ExecuteUpgradeCall::decode(tx_input.clone())
                .map_err(|err| err.to_string())?
                .diamond_cut
        }
        //DiamondCutFacet - old
        [54, 212, 235, 132] => {
            ExecuteUpgradeWithProposalSaltCall::decode(tx_input.clone())
                .map_err(|err| err.to_string())?
                .diamond_cut
        }
        _ => return Err("Unknown facet function selector".to_string()),
    };

    parse_diamond_cut_data(&diamond_cut)?;

    Ok(())
}
