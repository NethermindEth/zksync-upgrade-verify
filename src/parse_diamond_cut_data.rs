use ethers::types::Address;

use crate::{
    constants::{TX_TYPE_UPGGADE, ZERO},
    l2_contracts_names::get_system_contract_name,
    slots_names::{bytes_to_hex_string, get_facet_name},
    upgrade_abi::{DiamondCutData, ForceDeployOnAddressesCall, UpgradeCall},
};

use crate::function_signature::FunctionSignature;
use ethers::core::abi::AbiDecode;

// Parse and print Dimond Cut data
pub fn parse_diamond_cut_data(diamond_cut: &DiamondCutData) -> Result<(), String> {
    // check diamond_cut.facet_cuts and save them (check on slots?)
    // we want to see see Faucet delete/add
    if diamond_cut.facet_cuts.len() % 2 != 0 {
        return Err(format!(
            "Unexpected number of facets: {:?}",
            diamond_cut.facet_cuts
        ));
    }

    let offset = diamond_cut.facet_cuts.len() / 2;
    if offset > 0 {
        println!("\x1b[38;5;49mNew Facets:\x1b[0m");
    }

    for i in 0..offset {
        if diamond_cut.facet_cuts[i].action != 2 || diamond_cut.facet_cuts[i + offset].action != 0 {
            return Err(format!(
                "Unexpected facet cut {}: {:?}",
                i, diamond_cut.facet_cuts
            ));
        }

        // We get selector by first slot todo: Improvement needed
        let name = get_facet_name(&diamond_cut.facet_cuts[i].selectors[0])
            .unwrap_or_else(|| format!("Unknown facet: {:?}", diamond_cut.facet_cuts[i].selectors));

        println!(
            "  \x1b[38;5;117m{}\x1b[0m 0x{:02x}",
            name,
            diamond_cut.facet_cuts[i + offset].facet
        );
    }
    // print init_address for delecatecall
    println!(
        "\x1b[38;5;49mUpgrade contract address:\x1b[0m 0x{:02x}",
        diamond_cut.init_address
    );
    // check init_calldata: msg.sig==0x1ed824a0
    // It is not necessary, but we can't decode calls otherwise
    if diamond_cut.init_calldata.sig() != [0x1e, 0xd8, 0x24, 0xa0] {
        return Err(format!(
            "Unexpected init method signature : {}",
            diamond_cut.init_calldata
        ));
    }

    // delecatecall upgrade contract (init_address)
    // e.g. https://github.com/matter-labs/era-contracts/blob/4aa7006153ad571643342dff22c16eaf4a70fdc1/l1-contracts/contracts/upgrades/Upgrade_v1_4_1.sol
    let tx_input = diamond_cut.init_calldata.clone();
    let decoded = UpgradeCall::decode(tx_input).map_err(|err| err.to_string())?;
    // check bootloader_hash
    if decoded.proposed_upgrade.bootloader_hash != ZERO {
        println!(
            "\x1b[38;5;49mNew bootloader_hash\x1b[0m 0x{}",
            bytes_to_hex_string(&decoded.proposed_upgrade.bootloader_hash)
        );
    }
    // check default_account_hash
    if decoded.proposed_upgrade.default_account_hash != ZERO {
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
        != ZERO
        || decoded
            .proposed_upgrade
            .verifier_params
            .recursion_leaf_level_vk_hash
            != ZERO
        || decoded
            .proposed_upgrade
            .verifier_params
            .recursion_circuits_set_vks_hash
            != ZERO
    {
        println!("\x1b[38;5;49mNew verifier_params:\x1b[0m");
        println!(
            "   \x1b[38;5;117mNew recursionNodeLevelVkHash:\x1b[0m 0x{}",
            bytes_to_hex_string(
                &decoded
                    .proposed_upgrade
                    .verifier_params
                    .recursion_node_level_vk_hash
            )
        );
        println!(
            "   \x1b[38;5;117mNew recursionLeafLevelVkHash:\x1b[0m 0x{}",
            bytes_to_hex_string(
                &decoded
                    .proposed_upgrade
                    .verifier_params
                    .recursion_leaf_level_vk_hash
            )
        );
        println!(
            "   \x1b[38;5;117mNew recursionCircuitsSetVksHash:\x1b[0m 0x{}",
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
    // Print l2 system contracts
    // https://github.com/matter-labs/era-contracts/blob/4aa7006153ad571643342dff22c16eaf4a70fdc1/l2-contracts/contracts/L2ContractHelper.sol#L47
    if decoded.proposed_upgrade.l_2_protocol_upgrade_tx.tx_type == TX_TYPE_UPGGADE {
        let tx_input = decoded.proposed_upgrade.l_2_protocol_upgrade_tx.data;
        let decoded =
            ForceDeployOnAddressesCall::decode(tx_input).map_err(|err| err.to_string())?;
        // show new hashed for system contracts
        println!("\x1b[38;5;49mSystem contracts:\x1b[0m");
        for el in decoded.deploy_params {
            println!(
                "  \x1b[38;5;117m{} bytecode_hash:\x1b[0m 0x{}",
                get_system_contract_name(&el.new_address),
                bytes_to_hex_string(&el.bytecode_hash)
            );
        }
    }
    Ok(())
}
