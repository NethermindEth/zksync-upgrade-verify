use crate::facet_names::get_facet_name;
use crate::upgrade_abi::Operation;
use crate::{
    constants::ZKSYNC_ERA,
    function_selector::FunctionSelector,
    upgrade_abi::{DiamondCutData, ExecuteUpgradeCall, ExecuteUpgradeWithProposalSaltCall},
    upgrade_call_data::UpgradeCallData,
};
use ethers::core::abi::AbiDecode;
use ethers::types::Bytes;

pub fn get_dimond_cut_call(operation: &Operation) -> Result<Bytes, String> {
    if operation.calls[0].target != ZKSYNC_ERA
        || operation.calls.len() != 1
        || operation.calls[0].data.selector() != [0xa9, 0xf6, 0xd9, 0x41]
    {
        return Err(format!("Unexpected governance call: {:?}", operation));
    }
    let tx_input = operation.calls[0].data.clone();
    Ok(tx_input)
}

pub fn parse_diamond_cut_call(tx_input: &Bytes) -> Result<(), String> {
    // Decode transaction to zkSync Era Diamond Proxy
    let diamond_cut = match tx_input.selector() {
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

    // Parse and print Dimond Cut data
    parse_diamond_cut_data(&diamond_cut)
}

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

    // delecatecall upgrade contract (init_address)
    // e.g. https://github.com/matter-labs/era-contracts/blob/4aa7006153ad571643342dff22c16eaf4a70fdc1/l1-contracts/contracts/upgrades/Upgrade_v1_4_1.sol
    let upgrade_call_data = UpgradeCallData::from_calldata(&diamond_cut.init_calldata)?;
    println!("{}", upgrade_call_data);

    Ok(())
}
