use crate::function_signature::FunctionSignature;
use crate::{
    parse_upgrade_call::parse_upgrade_call, parse_upgrade_call_new::parse_upgrade_call_new,
    slots_names::get_facet_name, upgrade_abi::DiamondCutData,
};

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
    match diamond_cut.init_calldata.sig() {
        [0x08, 0x28, 0x4e, 0x57] => parse_upgrade_call_new(&diamond_cut.init_calldata)?,
        [0x1e, 0xd8, 0x24, 0xa0] => parse_upgrade_call(&diamond_cut.init_calldata)?,
        _ => return Err("Unknown init method signature".to_string()),
    };

    Ok(())
}
