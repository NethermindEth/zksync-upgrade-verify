use crate::l2_contracts_names::get_system_contract_name;
use crate::slots_names::{
    bytes_to_hex_string, get_storage_slot_name, insert_facet_to_selector_slots,
    insert_facets_and_isfrozen_slots, insert_selector_to_facet_slots,
};
use crate::upgrade_abi::{
    ExecTransactionCall, ExecuteCall, ExecuteUpgradeCall, ForceDeployOnAddressesCall, UpgradeCall,
};
use ethers::core::abi::AbiDecode;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Address, Bytes, TraceType, H160, H256, U256},
};
use std::collections::HashMap;
use std::str::FromStr;

// Decode function signature from bytes
trait FunctionSignature {
    // get the first four bytes
    fn sig(&self) -> [u8; 4];
}
// Implement decoding of function signature from bytes
impl FunctionSignature for Bytes {
    fn sig(&self) -> [u8; 4] {
        if self.len() < 4 {
            return [0; 4];
        }
        let bytes: &[u8] = self.as_ref();
        [bytes[0], bytes[1], bytes[2], bytes[3]]
    }
}
// zkSync Era Diamond Proxy Contract address in Ethereum Mainnet
const ZK_ERA: H160 = H160([
    0x32, 0x40, 0x00, 0x84, 0xc2, 0x86, 0xcf, 0x3e, 0x17, 0xe7, 0xb6, 0x77, 0xea, 0x95, 0x83, 0xe6,
    0x0a, 0x00, 0x03, 0x24,
]);
// Upgrade tx type = 254
const TX_TYPE_UPGGADE: U256 = U256([0xfe, 0, 0, 0]);
// Zero hash
const ZERO_HASH: [u8; 32] = [0; 32];

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
    // Get upgrade transaction state diff
    let trace = provider
        .trace_replay_transaction(tx_hash, vec![TraceType::StateDiff])
        .await
        .map_err(|err| err.to_string())?;
    let diff = trace.state_diff.ok_or("StateDiff not found")?;
    // Map storage slot to its name
    let zksync_era_storage_diff = &diff
        .0
        .get(&ZK_ERA)
        .ok_or("zkSync Era changes not found")?
        .storage;
    let mut zk_era_slots_names: HashMap<H256, String> = HashMap::with_capacity(50);
    for (key, _) in zksync_era_storage_diff {
        if let Some(name) = get_storage_slot_name(key) {
            zk_era_slots_names.insert(*key, name);
        }
    }
    // Decode transaction to multisig 0x4e4943346848c4867F81dFb37c4cA9C5715A7828
    let tx_input = tx.input;
    let decoded = ExecTransactionCall::decode(&tx_input).map_err(|err| err.to_string())?;
    // todo check governance address decoded.to =  0x0b622A2061EaccAE1c664eBC3E868b8438e03F61

    // Decode transaction to governance
    let tx_input = decoded.data;
    let decoded = ExecuteCall::decode(tx_input).map_err(|err| err.to_string())?;
    // Check governance calls
    if decoded.operation.calls[0].target != ZK_ERA
        || decoded.operation.calls.len() != 1
        || decoded.operation.calls[0].data.sig() != [0xa9, 0xf6, 0xd9, 0x41]
    {
        return Err(format!("Unexpected governance call: {}", decoded));
    }

    // Decode transaction to zkSync Era Diamond Proxy
    // We call Admin facet selector 0xa9f6d941
    let tx_input = decoded.operation.calls[0].data.clone();
    let decoded = ExecuteUpgradeCall::decode(tx_input).map_err(|err| err.to_string())?;
    // check diamond_cut.facet_cuts and save them (check on slots?)
    // we want to see see Faucet delete/add
    if decoded.diamond_cut.facet_cuts.len() % 2 != 0 {
        return Err(format!(
            "Unexpected number of facets: {:?}",
            decoded.diamond_cut.facet_cuts
        ));
    }
    // delete selector i first and then add selector i + len/2.
    let offset = decoded.diamond_cut.facet_cuts.len() / 2;
    // find slots for DiamondStorage.facets[]
    insert_facets_and_isfrozen_slots(&mut zk_era_slots_names, offset);
    println!("\x1b[38;5;49mNew Facets:\x1b[0m");
    for i in 0..offset {
        if decoded.diamond_cut.facet_cuts[i].selectors
            != decoded.diamond_cut.facet_cuts[i + offset].selectors
            && decoded.diamond_cut.facet_cuts[i].action != 0
            && decoded.diamond_cut.facet_cuts[i + offset].action != 2
        {
            return Err(format!(
                "Unexpected facet cut {}: {:?}",
                i, decoded.diamond_cut.facet_cuts
            ));
        }

        // find facetToSelector slots for new facet address
        let slot_count: i32 =
            decoded.diamond_cut.facet_cuts[i + offset].selectors.len() as i32 / 8 + 1;
        insert_facet_to_selector_slots(
            &mut zk_era_slots_names,
            &decoded.diamond_cut.facet_cuts[i + offset].facet,
            slot_count,
        );

        // For every selector we find changed slots
        // https://github.com/matter-labs/era-contracts/blob/a1a05513c110586f7a1d8b5fa46115cc7307587a/l1-contracts/contracts/zksync/libraries/Diamond.sol#L50C5-L55C6
        let old_facet_address: Option<H160> = insert_selector_to_facet_slots(
            &mut zk_era_slots_names,
            zksync_era_storage_diff,
            &decoded.diamond_cut.facet_cuts[i + offset].selectors,
        );

        // find facet to selector for old facet address
        if let Some(old_addr) = old_facet_address {
            let slot_count: i32 = decoded.diamond_cut.facet_cuts[i].selectors.len() as i32 / 8 + 1;
            insert_facet_to_selector_slots(&mut zk_era_slots_names, &old_addr, slot_count);
        }

        // We get selector by first slot todo: improve approach
        let name = match decoded.diamond_cut.facet_cuts[i].selectors[0] {
            [14, 24, 182, 129] => "Admin Facet".to_string(),
            [205, 255, 172, 198] => "Getters Facet".to_string(),
            [108, 9, 96, 249] => "Mailbox Facet".to_string(),
            [112, 31, 88, 197] => "Executor Facet".to_string(),
            _ => format!(
                "Unknown facet: {:?}",
                decoded.diamond_cut.facet_cuts[i].selectors
            ),
        };
        println!(
            "  \x1b[38;5;117m{}\x1b[0m 0x{:02x}",
            name,
            decoded.diamond_cut.facet_cuts[i + offset].facet
        );
    }
    // print init_address for delecatecall
    println!(
        "\x1b[38;5;49mUpgrade contract address:\x1b[0m 0x{:02x}",
        decoded.diamond_cut.init_address
    );
    // check init_calldata: msg.sig==0x1ed824a0
    // It is not necessary, but we can't decode calls otherwise
    if decoded.diamond_cut.init_calldata.sig() != [0x1e, 0xd8, 0x24, 0xa0] {
        return Err(format!(
            "Unexpected init method signature : {}",
            decoded.diamond_cut.init_calldata
        ));
    }

    // delecatecall upgrade contract (init_address)
    // e.g. https://github.com/matter-labs/era-contracts/blob/4aa7006153ad571643342dff22c16eaf4a70fdc1/l1-contracts/contracts/upgrades/Upgrade_v1_4_1.sol
    let tx_input = decoded.diamond_cut.init_calldata;
    let decoded = UpgradeCall::decode(tx_input).map_err(|err| err.to_string())?;
    // check l_2_protocol_upgrade_tx.tx_type == 254
    if decoded.proposed_upgrade.l_2_protocol_upgrade_tx.tx_type != TX_TYPE_UPGGADE {
        println!("{:?} {}", TX_TYPE_UPGGADE, TX_TYPE_UPGGADE);
        return Err(format!(
            "Unexpected L2 protocol upgrade tx_type: {}",
            decoded.proposed_upgrade.l_2_protocol_upgrade_tx.tx_type
        ));
    }
    // todo factory_deps == l_2_protocol_upgrade_tx.factory_deps
    // check bootloader_hash
    if decoded.proposed_upgrade.bootloader_hash != ZERO_HASH {
        println!(
            "\x1b[38;5;49mNew bootloader_hash\x1b[0m 0x{}",
            bytes_to_hex_string(&decoded.proposed_upgrade.bootloader_hash)
        );
    }
    // check default_account_hash
    if decoded.proposed_upgrade.default_account_hash != ZERO_HASH {
        println!(
            "\x1b[38;5;49mNew default_account_hash\x1b[0m 0x{}",
            bytes_to_hex_string(&decoded.proposed_upgrade.default_account_hash)
        );
    }
    // check verifier
    if decoded.proposed_upgrade.verifier != Address::zero() {
        println!(
            "\x1b[38;5;49mNew verifier address:\x1b[0m 0x{:02x}",
            decoded.proposed_upgrade.verifier
        );
    }
    // verifier_params if all zerros then doesn't change
    if decoded
        .proposed_upgrade
        .verifier_params
        .recursion_node_level_vk_hash
        != ZERO_HASH
        || decoded
            .proposed_upgrade
            .verifier_params
            .recursion_leaf_level_vk_hash
            != ZERO_HASH
        || decoded
            .proposed_upgrade
            .verifier_params
            .recursion_circuits_set_vks_hash
            != ZERO_HASH
    {
        println!("\x1b[38;5;49mNew verifier_params:\x1b[0m");
        println!(
            "   \x1b[38;5;117m0xNew recursionNodeLevelVkHash:\x1b[0m {:?}",
            bytes_to_hex_string(
                &decoded
                    .proposed_upgrade
                    .verifier_params
                    .recursion_node_level_vk_hash
            )
        );
        println!(
            "   \x1b[38;5;117m0xNew recursionLeafLevelVkHash:\x1b[0m {:?}",
            bytes_to_hex_string(
                &decoded
                    .proposed_upgrade
                    .verifier_params
                    .recursion_leaf_level_vk_hash
            )
        );
        println!(
            "   \x1b[38;5;117m0xNew recursionCircuitsSetVksHash:\x1b[0m {:?}",
            bytes_to_hex_string(
                &decoded
                    .proposed_upgrade
                    .verifier_params
                    .recursion_circuits_set_vks_hash
            )
        );
    }
    // l_1_contracts_upgrade_calldata: expected to be empty
    // l1ContractsUpgradeCalldata Custom calldata for L1 contracts upgrade,
    // it may be interpreted differently in each upgrade. Usually empty.
    if decoded
        .proposed_upgrade
        .l_1_contracts_upgrade_calldata
        .len()
        != 0
    {
        println!(
            "Warning: L1 contracts upgrade calldata not empty: {}",
            decoded.proposed_upgrade.l_1_contracts_upgrade_calldata
        );
    }
    // post_upgrade_calldata expected to be empty
    // postUpgradeCalldata Custom calldata for post upgrade hook,
    // it may be interpreted differently in each upgrade. Usually empty.
    if decoded.proposed_upgrade.post_upgrade_calldata.len() != 0 {
        println!(
            "Warning: post upgrade calldata not empty: {}",
            decoded.proposed_upgrade.l_1_contracts_upgrade_calldata
        );
    }
    // upgrade_timestamp
    //  upgradeTimestamp The timestamp after which the upgrade can be executed.
    println!(
        "\x1b[38;5;49mCan be executed after:\x1b[0m {}",
        decoded.proposed_upgrade.upgrade_timestamp
    );
    // new_protocol_version
    println!(
        "\x1b[38;5;49mNew_protocol version:\x1b[0m {}",
        decoded.proposed_upgrade.new_protocol_version
    );
    // newAllowList The address of the new allowlist contract. If zero, it will not be updated.
    if decoded.proposed_upgrade.new_allow_list != Address::zero() {
        println!(
            "\x1b[38;5;49mNew allowlist address:\x1b[0m 0x{:02x}",
            decoded.proposed_upgrade.new_allow_list
        );
    }

    // https://github.com/matter-labs/era-contracts/blob/4aa7006153ad571643342dff22c16eaf4a70fdc1/l2-contracts/contracts/L2ContractHelper.sol#L47
    let tx_input = decoded.proposed_upgrade.l_2_protocol_upgrade_tx.data;
    let decoded = ForceDeployOnAddressesCall::decode(tx_input).map_err(|err| err.to_string())?;
    // show new hashed for system contracts
    println!("\x1b[38;5;49mSystem contracts:\x1b[0m");
    for el in decoded.deploy_params {
        println!(
            "  \x1b[38;5;117m{} bytecode_hash:\x1b[0m 0x{}",
            get_system_contract_name(&el.new_address),
            bytes_to_hex_string(&el.bytecode_hash)
        );
        //todo check call_constructor
        //todo check value
        //todo check input
    }

    // output zksync_era_storage_diff
    println!("\x1b[38;5;49mDiamond Proxy storage changed:\x1b[0m");
    for (key, value) in zksync_era_storage_diff {
        if let ethers::types::Diff::Changed(change) = value {
            if let Some(name) = zk_era_slots_names.get(key) {
                println!("\x1b[38;5;117m{}\x1b[0m", name);
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
