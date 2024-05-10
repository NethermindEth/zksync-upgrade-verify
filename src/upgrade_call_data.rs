use std::fmt;

use crate::function_selector::FunctionSelector;
use crate::{
    constants::{TX_TYPE_UPGGADE, ZERO},
    l2_contracts_names::get_system_contract_name,
    utils::bytes_to_hex_string,
    upgrade_abi::{ForceDeployOnAddressesCall, UpgradeCall},
    upgrade_abi_new::UpgradeCall as UpgradeCallNew,
};
use ethers::core::abi::AbiDecode;
use ethers::types::{Address, Bytes, U256};

struct VerifierParams {
    recursion_node_level_vk_hash: [u8; 32],
    recursion_leaf_level_vk_hash: [u8; 32],
    recursion_circuits_set_vks_hash: [u8; 32],
}

pub struct UpgradeCallData {
    bootloader_hash: Option<[u8; 32]>,
    default_account_hash: Option<[u8; 32]>,
    verifier: Option<Address>,
    verifier_params: Option<VerifierParams>,
    l_1_contracts_upgrade_calldata: Option<Bytes>,
    post_upgrade_calldata: Option<Bytes>,
    upgrade_timestamp: U256,
    new_protocol_version: U256,
    l_2_protocol_upgrade_tx: Option<Vec<(Address, [u8; 32])>>,
    new_allow_list: Option<Address>,
}

impl UpgradeCallData {
    pub fn from_calldata(calldata: &Bytes) -> Result<Self, String> {
        match calldata.selector() {
            [0x08, 0x28, 0x4e, 0x57] => UpgradeCallData::from_upgrade_call(&calldata),
            [0x1e, 0xd8, 0x24, 0xa0] => {
                UpgradeCallData::from_upgrade_call_with_allowlist(&calldata)
            }
            _ => return Err("Unknown init method signature".to_string()),
        }
    }

    pub fn from_upgrade_call_with_allowlist(upgrade_call: &Bytes) -> Result<Self, String> {
        let decoded = UpgradeCall::decode(upgrade_call).map_err(|err| err.to_string())?;
        // check bootloader_hash
        let bootloader_hash = if decoded.proposed_upgrade.bootloader_hash != ZERO {
            Some(decoded.proposed_upgrade.bootloader_hash)
        } else {
            None
        };
        // check default_account_hash
        let default_account_hash = if decoded.proposed_upgrade.default_account_hash != ZERO {
            Some(decoded.proposed_upgrade.default_account_hash)
        } else {
            None
        };
        // check verifier
        let verifier = if decoded.proposed_upgrade.verifier != Address::zero() {
            Some(decoded.proposed_upgrade.verifier)
        } else {
            None
        };
        // verifier_params if all zerros then doesn't change
        let verifier_params = if decoded
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
            Some(VerifierParams {
                recursion_node_level_vk_hash: decoded
                    .proposed_upgrade
                    .verifier_params
                    .recursion_node_level_vk_hash,
                recursion_leaf_level_vk_hash: decoded
                    .proposed_upgrade
                    .verifier_params
                    .recursion_leaf_level_vk_hash,
                recursion_circuits_set_vks_hash: decoded
                    .proposed_upgrade
                    .verifier_params
                    .recursion_circuits_set_vks_hash,
            })
        } else {
            None
        };
        // l_1_contracts_upgrade_calldata: expected to be empty
        // l1ContractsUpgradeCalldata Custom calldata for L1 contracts upgrade,
        // it may be interpreted differently in each upgrade. Usually empty.
        let l_1_contracts_upgrade_calldata = if decoded
            .proposed_upgrade
            .l_1_contracts_upgrade_calldata
            .len()
            != 0
        {
            Some(decoded.proposed_upgrade.l_1_contracts_upgrade_calldata)
        } else {
            None
        };
        // post_upgrade_calldata expected to be empty
        // postUpgradeCalldata Custom calldata for post upgrade hook,
        // it may be interpreted differently in each upgrade. Usually empty.
        let post_upgrade_calldata = if decoded.proposed_upgrade.post_upgrade_calldata.len() != 0 {
            Some(decoded.proposed_upgrade.post_upgrade_calldata)
        } else {
            None
        };
        // upgrade_timestamp
        //  upgradeTimestamp The timestamp after which the upgrade can be executed.
        let upgrade_timestamp = decoded.proposed_upgrade.upgrade_timestamp;
        // New protocol version
        let new_protocol_version = decoded.proposed_upgrade.new_protocol_version;
        // newAllowList The address of the new allowlist contract. If zero, it will not be updated.
        let new_allow_list = if decoded.proposed_upgrade.new_allow_list != Address::zero() {
            Some(decoded.proposed_upgrade.new_allow_list)
        } else {
            None
        };
        // Check l2 system contracts
        // https://github.com/matter-labs/era-contracts/blob/4aa7006153ad571643342dff22c16eaf4a70fdc1/l2-contracts/contracts/L2ContractHelper.sol#L47
        let l_2_protocol_upgrade_tx =
            if decoded.proposed_upgrade.l_2_protocol_upgrade_tx.tx_type == TX_TYPE_UPGGADE {
                let tx_input = decoded.proposed_upgrade.l_2_protocol_upgrade_tx.data;
                let decoded =
                    ForceDeployOnAddressesCall::decode(tx_input).map_err(|err| err.to_string())?;
                let mut value = Vec::with_capacity(decoded.deploy_params.len());
                for el in decoded.deploy_params {
                    value.push((el.new_address, el.bytecode_hash));
                }
                Some(value)
            } else {
                None
            };

        Ok(UpgradeCallData {
            bootloader_hash,
            default_account_hash,
            verifier,
            verifier_params,
            l_1_contracts_upgrade_calldata,
            post_upgrade_calldata,
            upgrade_timestamp,
            new_protocol_version,
            l_2_protocol_upgrade_tx,
            new_allow_list,
        })
    }

    pub fn from_upgrade_call(upgrade_call: &Bytes) -> Result<Self, String> {
        let decoded = UpgradeCallNew::decode(upgrade_call).map_err(|err| err.to_string())?;
        // check bootloader_hash
        let bootloader_hash = if decoded.proposed_upgrade.bootloader_hash != ZERO {
            Some(decoded.proposed_upgrade.bootloader_hash)
        } else {
            None
        };
        // check default_account_hash
        let default_account_hash = if decoded.proposed_upgrade.default_account_hash != ZERO {
            Some(decoded.proposed_upgrade.default_account_hash)
        } else {
            None
        };
        // check verifier
        let verifier = if decoded.proposed_upgrade.verifier != Address::zero() {
            Some(decoded.proposed_upgrade.verifier)
        } else {
            None
        };
        // verifier_params if all zerros then doesn't change
        let verifier_params = if decoded
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
            Some(VerifierParams {
                recursion_node_level_vk_hash: decoded
                    .proposed_upgrade
                    .verifier_params
                    .recursion_node_level_vk_hash,
                recursion_leaf_level_vk_hash: decoded
                    .proposed_upgrade
                    .verifier_params
                    .recursion_leaf_level_vk_hash,
                recursion_circuits_set_vks_hash: decoded
                    .proposed_upgrade
                    .verifier_params
                    .recursion_circuits_set_vks_hash,
            })
        } else {
            None
        };
        // l_1_contracts_upgrade_calldata: expected to be empty
        // l1ContractsUpgradeCalldata Custom calldata for L1 contracts upgrade,
        // it may be interpreted differently in each upgrade. Usually empty.
        let l_1_contracts_upgrade_calldata = if decoded
            .proposed_upgrade
            .l_1_contracts_upgrade_calldata
            .len()
            != 0
        {
            Some(decoded.proposed_upgrade.l_1_contracts_upgrade_calldata)
        } else {
            None
        };
        // post_upgrade_calldata expected to be empty
        // postUpgradeCalldata Custom calldata for post upgrade hook,
        // it may be interpreted differently in each upgrade. Usually empty.
        let post_upgrade_calldata = if decoded.proposed_upgrade.post_upgrade_calldata.len() != 0 {
            Some(decoded.proposed_upgrade.post_upgrade_calldata)
        } else {
            None
        };
        // upgrade_timestamp
        //  upgradeTimestamp The timestamp after which the upgrade can be executed.
        let upgrade_timestamp = decoded.proposed_upgrade.upgrade_timestamp;
        // New protocol version
        let new_protocol_version = decoded.proposed_upgrade.new_protocol_version;
        // Check l2 system contracts
        // https://github.com/matter-labs/era-contracts/blob/4aa7006153ad571643342dff22c16eaf4a70fdc1/l2-contracts/contracts/L2ContractHelper.sol#L47
        let l_2_protocol_upgrade_tx =
            if decoded.proposed_upgrade.l_2_protocol_upgrade_tx.tx_type == TX_TYPE_UPGGADE {
                let tx_input = decoded.proposed_upgrade.l_2_protocol_upgrade_tx.data;
                let decoded =
                    ForceDeployOnAddressesCall::decode(tx_input).map_err(|err| err.to_string())?;
                let mut value = Vec::with_capacity(decoded.deploy_params.len());
                for el in decoded.deploy_params {
                    value.push((el.new_address, el.bytecode_hash));
                }
                Some(value)
            } else {
                None
            };

        Ok(UpgradeCallData {
            bootloader_hash,
            default_account_hash,
            verifier,
            verifier_params,
            l_1_contracts_upgrade_calldata,
            post_upgrade_calldata,
            upgrade_timestamp,
            new_protocol_version,
            l_2_protocol_upgrade_tx,
            new_allow_list: None,
        })
    }
}

impl fmt::Display for UpgradeCallData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // print bootloader_hash
        if self.bootloader_hash.is_some() {
            writeln!(
                f,
                "\x1b[38;5;49mNew bootloader hash\x1b[0m 0x{}",
                bytes_to_hex_string(&self.bootloader_hash.unwrap())
            )?;
        }
        // print default_account_hash
        if self.default_account_hash.is_some() {
            writeln!(
                f,
                "\x1b[38;5;49mNew default account hash\x1b[0m 0x{}",
                bytes_to_hex_string(&self.default_account_hash.unwrap())
            )?;
        }
        // print verifier
        if self.verifier.is_some() {
            writeln!(
                f,
                "\x1b[38;5;49mNew verifier address:\x1b[0m 0x{:02x}",
                self.verifier.unwrap()
            )?;
        }
        // verifier_params if all zerros then doesn't change
        if self.verifier_params.is_some() {
            writeln!(f, "\x1b[38;5;49mNew verifier_params:\x1b[0m")?;
            writeln!(
                f,
                "   \x1b[38;5;117mNew recursionNodeLevelVkHash:\x1b[0m 0x{}",
                bytes_to_hex_string(
                    &self
                        .verifier_params
                        .as_ref()
                        .unwrap()
                        .recursion_node_level_vk_hash
                )
            )?;
            writeln!(
                f,
                "   \x1b[38;5;117mNew recursionLeafLevelVkHash:\x1b[0m 0x{}",
                bytes_to_hex_string(
                    &self
                        .verifier_params
                        .as_ref()
                        .unwrap()
                        .recursion_leaf_level_vk_hash
                )
            )?;
            writeln!(
                f,
                "   \x1b[38;5;117mNew recursionCircuitsSetVksHash:\x1b[0m 0x{}",
                bytes_to_hex_string(
                    &self
                        .verifier_params
                        .as_ref()
                        .unwrap()
                        .recursion_circuits_set_vks_hash
                )
            )?;
        }
        // l_1_contracts_upgrade_calldata: expected to be empty
        // l1ContractsUpgradeCalldata Custom calldata for L1 contracts upgrade,
        // it may be interpreted differently in each upgrade. Usually empty.
        if self.l_1_contracts_upgrade_calldata.is_some() {
            writeln!(
                f,
                "Warning: L1 contracts upgrade calldata not empty: {}",
                self.l_1_contracts_upgrade_calldata.as_ref().unwrap()
            )?;
        }
        // post_upgrade_calldata expected to be empty
        // postUpgradeCalldata Custom calldata for post upgrade hook,
        // it may be interpreted differently in each upgrade. Usually empty.
        if self.post_upgrade_calldata.is_some() {
            writeln!(
                f,
                "Warning: post upgrade calldata not empty: {}",
                self.post_upgrade_calldata.as_ref().unwrap()
            )?;
        }
        // upgrade_timestamp
        //  upgradeTimestamp The timestamp after which the upgrade can be executed.
        writeln!(
            f,
            "\x1b[38;5;49mCan be executed after:\x1b[0m {}",
            self.upgrade_timestamp
        )?;
        // new_protocol_version
        writeln!(
            f,
            "\x1b[38;5;49mNew protocol version:\x1b[0m {}",
            self.new_protocol_version
        )?;

        // newAllowList The address of the new allowlist contract. If zero, it will not be updated.
        if self.new_allow_list.is_some() {
            writeln!(
                f,
                "\x1b[38;5;49mNew allowlist address:\x1b[0m 0x{:02x}",
                self.new_allow_list.unwrap()
            )?;
        }
        // Print l2 system contracts
        // https://github.com/matter-labs/era-contracts/blob/4aa7006153ad571643342dff22c16eaf4a70fdc1/l2-contracts/contracts/L2ContractHelper.sol#L47
        if self.l_2_protocol_upgrade_tx.is_some() {
            writeln!(f, "\x1b[38;5;49mSystem contracts:\x1b[0m")?;
            for el in self.l_2_protocol_upgrade_tx.as_ref().unwrap() {
                writeln!(
                    f,
                    "  \x1b[38;5;117m{} bytecode hash:\x1b[0m 0x{}",
                    get_system_contract_name(&el.0),
                    bytes_to_hex_string(&el.1)
                )?;
            }
        }
        Ok(())
    }
}
