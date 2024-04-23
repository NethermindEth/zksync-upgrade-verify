use std::collections::{BTreeMap, HashMap};

use ethers::{
    types::{Address, Diff, H160, H256},
    utils::keccak256,
};

// bytes4 padded 32 bytes . Diamond.DiamondStorageSlot
const ENCODED_SELECTOR: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    200, 252, 173, 141, 184, 77, 60, 193, 139, 76, 65, 213, 81, 234, 14, 230, 109, 213, 153, 205,
    224, 104, 217, 152, 229, 125, 94, 9, 51, 44, 19, 27,
];

// convert bytes to hex string
pub fn bytes_to_hex_string(bytes: &[u8]) -> String {
    let hex_string: String = bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();
    hex_string
}

// add one to big number
fn add_one_to_big_number(number: &mut [u8]) {
    let mut carry = 1;

    for byte in number.iter_mut().rev() {
        let sum = *byte as u16 + carry;
        *byte = sum as u8;
        carry = sum / 256;

        if carry == 0 {
            break;
        }
    }
}

// find slots for DiamondStorage.facets[]
// and DiamondStorage.isFrozen
//address[] facets;
pub fn insert_facets_and_isfrozen_slots(
    slot_names_map: &mut HashMap<H256, String>,
    facets_len: usize,
) {
    // get DiamondStorage slot
    let mut encoded_32: [u8; 32] = ENCODED_SELECTOR[32..]
        .try_into()
        .expect("Array must be of length 32");
    // move to DiamondStorage.facets[] slot. Offset is 2.
    // It is array length
    encoded_32[31] += 2;
    slot_names_map.insert(H256(encoded_32), format!("DimondStorage.facets.length"));
    // calculate storage slot of data in facets[]
    let mut faucet_arr_slot = keccak256(encoded_32);
    if facets_len != 0 {
        for i in 0..facets_len {
            slot_names_map.insert(
                H256(faucet_arr_slot),
                format!("DimondStorage.facets[{}]", i),
            );
            // increase current storage slot by one
            add_one_to_big_number(&mut faucet_arr_slot);
        }
    }
    // move to DiamondStorage.isFrozen slot. Offset is 3.
    encoded_32[31] += 1;
    slot_names_map.insert(H256(encoded_32), format!("DimondStorage.isFrozen"));
}

// find DiamondStorage.SelectorToFacet slots for selectors
// For every selector we find changed slots
// and for old(replaced) facet address
// mapping(bytes4 selector => SelectorToFacet selectorInfo) selectorToFacet;
// struct SelectorToFacet {
//    address facetAddress;
//    uint16 selectorPosition;
//    bool isFreezable;
//}
// https://github.com/matter-labs/era-contracts/blob/a1a05513c110586f7a1d8b5fa46115cc7307587a/l1-contracts/contracts/zksync/libraries/Diamond.sol#L50C5-L55C6
pub fn insert_selector_to_facet_slots(
    slot_names_map: &mut HashMap<H256, String>,
    storage_diff: &BTreeMap<H256, Diff<H256>>,
    selectors: &Vec<[u8; 4]>,
) -> Option<H160> {
    // get DiamondStorage slot
    let mut encoded = ENCODED_SELECTOR.clone();
    let mut old_facet_address: Option<H160> = None;
    for selector in selectors {
        // find selector to facet
        // encode data as byte4(key).. . SelectorToFacetStorageSlot
        encoded[..4].copy_from_slice(selector);
        // get SelectorToFacet[selector].facet slot
        let mut slot = keccak256(encoded);
        slot_names_map.insert(
            H256(slot),
            format!(
                "selectorToFacet[ 0x{} ].facetAddress",
                bytes_to_hex_string(selector)
            ),
        );
        // FacetToSelectors[selector]
        // get slot for selectorPosition and isFreezable
        let slot_facet_address = slot.clone();
        add_one_to_big_number(&mut slot);
        slot_names_map.insert(
            H256(slot),
            format!(
                "selectorToFacet[ 0x{} ].(selectorPosition,isFreezable)",
                bytes_to_hex_string(selector)
            ),
        );

        // get old(replaced) facet address
        if old_facet_address.is_none() {
            if let Some(ethers::types::Diff::Changed(value)) =
                storage_diff.get(&H256(slot_facet_address))
            {
                old_facet_address = Some(H160::from_slice(&value.from.as_ref()[12..32]));
            }
        }
    }
    old_facet_address
}

// find DiamondStorage.facetToSelectors slots for facet address
// mapping(address facetAddress => FacetToSelectors facetInfo) facetToSelectors;
// facetToSelectors {
//    bytes4[] selectors;
//    uint16 facetPosition;
// }
pub fn insert_facet_to_selector_slots(
    slot_names_map: &mut HashMap<H256, String>,
    facet_address: &Address,
    slot_count: i32,
) {
    // get DiamondStorage slot
    let mut encoded = ENCODED_SELECTOR.clone();
    // get FacetToSelectors slot by offset
    encoded[63] += 1;
    // encode data as ..address(key) . FacetToSelectorsStorageSlot
    encoded[12..32].copy_from_slice(facet_address.as_ref());
    // get SelectorToFacet[facet_address].selectors length slot
    let array_hash = keccak256(encoded);
    slot_names_map.insert(
        H256(array_hash),
        format!(
            "facetToSelectors[ 0x{:02x} ].selectors[].length",
            facet_address
        ),
    );
    let mut facet_position_hash = array_hash.clone();
    // get SelectorToFacet[facet_address].facetPosition slot
    add_one_to_big_number(&mut facet_position_hash);
    slot_names_map.insert(
        H256(facet_position_hash),
        format!("facetToSelectors[ 0x{:02x} ].facetPosition", facet_address),
    );
    // get SelectorToFacet[facet_address].selectors data slot
    let mut value_hash = keccak256(array_hash);
    for j in 0..slot_count {
        slot_names_map.insert(
            H256(value_hash),
            format!(
                "facetToSelectors[ 0x{:02x} ].selectors[] slot({}/{})",
                facet_address, j, slot_count
            ),
        );
        add_one_to_big_number(&mut value_hash);
    }
}

// AppStorage slots to names
//https://github.com/matter-labs/era-contracts/blob/a1a05513c110586f7a1d8b5fa46115cc7307587a/l1-contracts/contracts/zksync/Storage.sol#L103C8-L103C18
pub fn get_storage_slot_name(slot: &H256) -> Option<String> {
    match slot {
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ) => Some("__DEPRECATED_diamondCutStorage".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7],
        ) => Some("governor".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8],
        ) => Some("pendingGovernor".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9],
        ) => Some("validators".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10],
        ) => Some("verifier".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11],
        ) => Some("totalBatchesExecuted".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12],
        ) => Some("totalBatchesVerified".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13],
        ) => Some("totalBatchesCommitted".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14],
        ) => Some("storedBatchHashes".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15],
        ) => Some("l2LogsRootHashes".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16],
        ) => Some("priorityQueue".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19],
        ) => Some("__DEPRECATED_allowList".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20],
        ) => Some("verifierParams.recursionNodeLevelVkHash".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21],
        ) => Some("verifierParams.recursionLeafLevelVkHash".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22],
        ) => Some("verifierParams.recursionCircuitsSetVksHash".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23],
        ) => Some("l2BootloaderBytecodeHash".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24],
        ) => Some("l2DefaultAccountBytecodeHash".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25],
        ) => Some("zkPorterIsAvailable".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26],
        ) => Some("priorityTxMaxGasLimit".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27],
        ) => Some("__DEPRECATED_upgrades".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29],
        ) => Some("isEthWithdrawalFinalized".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30],
        ) => Some("__DEPRECATED_lastWithdrawalLimitReset".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31],
        ) => Some("__DEPRECATED_withdrawnAmountInWindow".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32],
        ) => Some("__DEPRECATED_totalDepositedAmountPerUser".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33],
        ) => Some("protocolVersion".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34],
        ) => Some("l2SystemContractsUpgradeTxHash".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35],
        ) => Some("l2SystemContractsUpgradeBatchNumber".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36],
        ) => Some("admin".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37],
        ) => Some("pendingAdmin".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38],
        ) => Some("feeParams".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39],
        ) => Some("blobVersionedHashRetriever".to_string()),
        _ => None,
    }
}
