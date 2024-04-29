use crate::constants::ZKSYNC_ERA;
use crate::function_signature::FunctionSignature;
use crate::parse_diamond_cut_data::parse_diamond_cut_data;
use crate::slots_names::{
    get_storage_slot_name, insert_facet_to_selector_slots, insert_facets_and_isfrozen_slots,
    insert_selector_to_facet_slots,
};
use crate::upgrade_abi::{
    ExecTransactionCall, ExecuteCall, ExecuteUpgradeCall, ExecuteUpgradeWithProposalSaltCall,
};
use ethers::core::abi::AbiDecode;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Address, TraceType, H160, H256},
};
use std::collections::HashMap;
use std::str::FromStr;

// Get upgrate info
pub async fn parse_upgrade_tx(tx_hash: &str, rpc_url: &str) -> Result<(), String> {
    let tx_hash: H256 = H256::from_str(tx_hash).map_err(|err| err.to_string())?;
    let provider = Provider::<Http>::try_from(rpc_url).map_err(|err| err.to_string())?;
    // Get upgrade transaction
    let tx = provider
        .get_transaction(tx_hash)
        .await
        .map_err(|err| err.to_string())?
        .ok_or("Transaction not found")?;
    // Decode transaction to multisig 0x4e4943346848c4867F81dFb37c4cA9C5715A7828
    let tx_input = tx.input;
    let decoded = ExecTransactionCall::decode(&tx_input)
        .map_err(|err| format!("ExecTransactionCall error: {}", err.to_string()))?;
    // check call address
    let tx_input = if decoded.to == ZKSYNC_ERA {
        decoded.data
    } else if decoded.to == Address::from_str("0x0b622A2061EaccAE1c664eBC3E868b8438e03F61").unwrap()
    {
        // Decode transaction to governance
        let tx_input = decoded.data;
        let decoded = ExecuteCall::decode(tx_input)
            .map_err(|err| format!("ExecuteCall error: {}", err.to_string()))?;
        // Check governance calls
        if decoded.operation.calls[0].target != ZKSYNC_ERA
            || decoded.operation.calls.len() != 1
            || decoded.operation.calls[0].data.sig() != [0xa9, 0xf6, 0xd9, 0x41]
        {
            return Err(format!("Unexpected governance call: {}", decoded));
        }
        decoded.operation.calls[0].data.clone()
    } else {
        return Err(format!("Call to unknown contract: {:?}", decoded.to));
    };

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
    // Get and print Dimond Cut data
    parse_diamond_cut_data(&diamond_cut)?;
    // init diff state and storage slots
    // Get upgrade transaction state diff
    let trace = provider
        .trace_replay_transaction(tx_hash, vec![TraceType::StateDiff])
        .await
        .map_err(|err| err.to_string())?;
    let diff = trace.state_diff.ok_or("StateDiff not found")?;
    // Map storage slot to its name
    let zksync_era_storage_diff = &diff
        .0
        .get(&ZKSYNC_ERA)
        .ok_or("zkSync Era changes not found")?
        .storage;
    let mut zksync_era_slots_names: HashMap<H256, String> = HashMap::with_capacity(50);
    for (key, _) in zksync_era_storage_diff {
        if let Some(name) = get_storage_slot_name(key) {
            zksync_era_slots_names.insert(*key, name);
        }
    }

    let offset = diamond_cut.facet_cuts.len() / 2;
    // find slots for DiamondStorage.facets[]
    insert_facets_and_isfrozen_slots(&mut zksync_era_slots_names, offset);

    for i in 0..offset {
        // find facetToSelector slots for new facet address
        let slot_count: i32 = diamond_cut.facet_cuts[i + offset].selectors.len() as i32 / 8 + 1;
        insert_facet_to_selector_slots(
            &mut zksync_era_slots_names,
            &diamond_cut.facet_cuts[i + offset].facet,
            slot_count,
        );

        // For every selector we find changed slots
        // https://github.com/matter-labs/era-contracts/blob/a1a05513c110586f7a1d8b5fa46115cc7307587a/l1-contracts/contracts/zksync/libraries/Diamond.sol#L50C5-L55C6
        let old_facet_address: Option<H160> = insert_selector_to_facet_slots(
            &mut zksync_era_slots_names,
            zksync_era_storage_diff,
            &diamond_cut.facet_cuts[i + offset].selectors,
        );

        // find facet to selector for old facet address
        if let Some(old_addr) = old_facet_address {
            let slot_count: i32 = diamond_cut.facet_cuts[i].selectors.len() as i32 / 8 + 1;
            insert_facet_to_selector_slots(&mut zksync_era_slots_names, &old_addr, slot_count);
        }
    }

    // output zksync_era_storage_diff
    println!("\x1b[38;5;49mDiamond Proxy storage changed:\x1b[0m");
    for (key, value) in zksync_era_storage_diff {
        if let ethers::types::Diff::Changed(change) = value {
            if let Some(name) = zksync_era_slots_names.get(key) {
                println!("{}", name);
            } else {
                println!("\x1b[38;5;117m0x{:02x}\x1b[0m", key);
            }
            println!("  from: 0x{:02x}", change.from);
            println!("    to: 0x{:02x}", change.to);
        } else {
            println!("0x{:02x} - {:?}", key, value);
        }
    }

    Ok(())
}
