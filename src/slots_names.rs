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
// no overflow checks
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
    slot_names_map.insert(H256(encoded_32), format!("\x1b[38;5;117mDimondStorage.facets.length\n \x1b[38;5;115mThe length of the array of all unique facet addresses that belong to the diamond proxy\x1b[0m"));
    // calculate storage slot of data in facets[]
    let mut faucet_arr_slot = keccak256(encoded_32);
    if facets_len != 0 {
        for i in 0..facets_len {
            slot_names_map.insert(
                H256(faucet_arr_slot),
                format!("\x1b[38;5;117mDimondStorage.facets[{}]\n \x1b[38;5;115mThe array of all unique facet addresses that belong to the diamond proxy\x1b[0m", i),
                
            );
            // increase current storage slot by one
            add_one_to_big_number(&mut faucet_arr_slot);
        }
    }
    // move to DiamondStorage.isFrozen slot. Offset is 3.
    encoded_32[31] += 1;
    slot_names_map.insert(H256(encoded_32), format!("\x1b[38;5;117mDimondStorage.isFrozen\n \x1b[38;5;115mDenotes whether the diamond proxy is frozen and all freezable facets are not accessible\x1b[0m"));
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
                "\x1b[38;5;117mselectorToFacet[ 0x{} ].facetAddress\n \x1b[38;5;115mAddress of the facet which is connected with selector\x1b[0m",
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
                "\x1b[38;5;117mselectorToFacet[ 0x{} ].(selectorPosition,isFreezable)\n \x1b[38;5;115mselectorPosition index in `FacetToSelectors.selectors` array, where is selector stored\nisFreezable denotes whether the selector can be frozen.\x1b[0m",
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
            "\x1b[38;5;117mfacetToSelectors[ 0x{:02x} ].selectors[].length\n \x1b[38;5;115mThe length of the selectors list for all selectors belonging to the facet.\x1b[0m",
            facet_address
        ),
    );
    let mut facet_position_hash = array_hash.clone();
    // get SelectorToFacet[facet_address].facetPosition slot
    add_one_to_big_number(&mut facet_position_hash);
    slot_names_map.insert(
        H256(facet_position_hash),
        format!("\x1b[38;5;117mfacetToSelectors[ 0x{:02x} ].facetPosition\n \x1b[38;5;115mIndex in `DiamondStorage.facets` array, where is facet stored\x1b[0m", facet_address),
    );
    // get SelectorToFacet[facet_address].selectors data slot
    let mut value_hash = keccak256(array_hash);
    for j in 0..slot_count {
        slot_names_map.insert(
            H256(value_hash),
            format!(
                "\x1b[38;5;117mfacetToSelectors[ 0x{:02x} ].selectors[] slot({}/{})\n \x1b[38;5;115mList of all selectors that belong to the facet\x1b[0m",
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
        ) => Some("\x1b[38;5;117m__DEPRECATED_diamondCutStorage\n \x1b[38;5;115mStorage of variables needed for deprecated diamond cut facet\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7],
        ) => Some("\x1b[38;5;117mgovernor\n \x1b[38;5;115mAddress which will exercise critical changes to the Diamond Proxy (upgrades, freezing & unfreezing)\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8],
        ) => Some("\x1b[38;5;117mpendingGovernor\n \x1b[38;5;115mAddress that the governor proposed as one that will replace it\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9],
        ) => Some("\x1b[38;5;117mvalidators\n \x1b[38;5;115mList of permitted validators\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10],
        ) => Some("\x1b[38;5;117mverifier\n \x1b[38;5;115mVerifier contract. Used to verify aggregated proof for batches\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11],
        ) => Some("\x1b[38;5;117mtotalBatchesExecuted\n \x1b[38;5;115mTotal number of executed batches i.e. batches[totalBatchesExecuted] points at the latest executed batch (batch 0 is genesis)\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12],
        ) => Some("\x1b[38;5;117mtotalBatchesVerified\n \x1b[38;5;115mTotal number of proved batches i.e. batches[totalBatchesProved] points at the latest proved batch\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13],
        ) => Some("\x1b[38;5;117mtotalBatchesCommitted\n \x1b[38;5;115mTotal number of committed batches i.e. batches[totalBatchesCommitted] points at the latest committed batch\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14],
        ) => Some("\x1b[38;5;117mstoredBatchHashes\n \x1b[38;5;115mStored hashed StoredBatch for batch number\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15],
        ) => Some("\x1b[38;5;117ml2LogsRootHashes\n \x1b[38;5;115mStored root hashes of L2 -> L1 logs\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16],
        ) => Some("\x1b[38;5;117mpriorityQueue\n \x1b[38;5;115mContainer that stores transactions requested from L1\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19],
        ) => Some("\x1b[38;5;117m__DEPRECATED_allowList\n \x1b[38;5;115mThe smart contract that manages the list with permission to call contract functions\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20],
        ) => Some("\x1b[38;5;117mverifierParams.recursionNodeLevelVkHash\n \x1b[38;5;115mPart of the configuration parameters of ZKP circuits. Used as an input for the verifier smart contract\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21],
        ) => Some("\x1b[38;5;117mverifierParams.recursionLeafLevelVkHash\n \x1b[38;5;115mPart of the configuration parameters of ZKP circuits. Used as an input for the verifier smart contract\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22],
        ) => Some("\x1b[38;5;117mverifierParams.recursionCircuitsSetVksHash\n \x1b[38;5;115mPart of the configuration parameters of ZKP circuits. Used as an input for the verifier smart contract\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23],
        ) => Some("\x1b[38;5;117ml2BootloaderBytecodeHash\n \x1b[38;5;115mBytecode hash of bootloader program. Used as an input to zkp-circuit.\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24],
        ) => Some("\x1b[38;5;117ml2DefaultAccountBytecodeHash\n \x1b[38;5;115mBytecode hash of default account (bytecode for EOA). Used as an input to zkp-circuit.\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25],
        ) => Some("\x1b[38;5;117mzkPorterIsAvailable\n \x1b[38;5;115mIndicates that the porter may be touched on L2 transactions. Used as an input to zkp-circuit.\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26],
        ) => Some("\x1b[38;5;117mpriorityTxMaxGasLimit\n \x1b[38;5;115mThe maximum number of the L2 gas that a user can request for L1 -> L2 transactions. This is the maximum number of L2 gas that is available for the \"body\" of the transaction, i.e. without overhead for proving the batch.\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27],
            //https://github.com/matter-labs/era-contracts/blob/a1a05513c110586f7a1d8b5fa46115cc7307587a/l1-contracts/contracts/zksync/Storage.sol#L27
        ) => Some("\x1b[38;5;117m__DEPRECATED_upgrades.proposedUpgradeHash\n \x1b[38;5;115mStorage of variables needed for upgrade facet\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28],
        ) => Some("\x1b[38;5;117m__DEPRECATED_upgrades.slot(1)\n \x1b[38;5;115mStorage of variables needed for upgrade facet\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29],
        ) => Some("\x1b[38;5;117misEthWithdrawalFinalized\n \x1b[38;5;115mA mapping L2 batch number => message number => flag. The L2 -> L1 log is sent for every withdrawal, so this mapping is serving as a flag to indicate that the message was already processed. Used to indicate that eth withdrawal was already processed\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30],
        ) => Some("\x1b[38;5;117m__DEPRECATED_lastWithdrawalLimitReset\n \x1b[38;5;115mThe most recent withdrawal time and amount reset\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31],
        ) => Some("\x1b[38;5;117m__DEPRECATED_withdrawnAmountInWindow\n \x1b[38;5;115mThe accumulated withdrawn amount during the withdrawal limit window\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32],
        ) => Some("\x1b[38;5;117m__DEPRECATED_totalDepositedAmountPerUser\n \x1b[38;5;115mA mapping user address => the total deposited amount by the user\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33],
        ) => Some("\x1b[38;5;117mprotocolVersion\n \x1b[38;5;115mStores the protocol version. Note, that the protocol version may not only encompass changes to the smart contracts, but also to the node behavior.\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34],
        ) => Some("\x1b[38;5;117ml2SystemContractsUpgradeTxHash\n \x1b[38;5;115mHash of the system contract upgrade transaction. If 0, then no upgrade transaction needs to be done.\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35],
        ) => Some("\x1b[38;5;117ml2SystemContractsUpgradeBatchNumber\n \x1b[38;5;115mBatch number where the upgrade transaction has happened. If 0, then no upgrade transaction has happened yet.\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36],
        ) => Some("\x1b[38;5;117madmin\n \x1b[38;5;115m Address which will exercise non-critical changes to the Diamond Proxy (changing validator set & unfreezing)\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37],
        ) => Some("\x1b[38;5;117mpendingAdmin\n \x1b[38;5;115mAddress that the governor or admin proposed as one that will replace admin role\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38],
        ) => Some("\x1b[38;5;117mfeeParams\n \x1b[38;5;115mFee params used to derive gasPrice for the L1->L2 transactions. For L2 transactions, the bootloader gives enough freedom to the operator.\x1b[0m".to_string()),
        H256(
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39],
        ) => Some("\x1b[38;5;117mblobVersionedHashRetriever\n \x1b[38;5;115mAddress of the blob versioned hash getter smart contract used for EIP-4844 versioned hashes.\x1b[0m".to_string()),
        _ => None,
    }
}

// get facet name from function selector
pub fn get_facet_name(selector: &[u8; 4]) -> Option<String> {
    match selector {
        // Admin Facet
        [14, 24, 182, 129] => Some("Admin".to_string()),
        // Governance Facet [Deprecated]
        [229, 139, 182, 57] => Some("Governance".to_string()),
        // Executor Facet
        [112, 31, 88, 197] => Some("Executor".to_string()),
        [12, 77, 216, 16] => Some("Executor".to_string()),
        // Mailbox Facet
        [108, 9, 96, 249] => Some("Mailbox".to_string()),
        // Getters Facet
        [205, 255, 172, 198] => Some("Getters".to_string()),
        // DiamondCut Facet [Deprecated]
        [115, 251, 146, 151] => Some("DiamondCut".to_string()),
        _ => None,
    }
}

#[test]
fn test_bytes_to_hex_string() {
    let bytes = [0xAA, 0xBB, 0xCC, 0xDD];
    let expected_hex_string = "aabbccdd";
    assert_eq!(bytes_to_hex_string(&bytes), expected_hex_string);
}

#[test]
fn test_add_one_to_big_number() {
    let mut number = [0, 0, 254];
    add_one_to_big_number(&mut number);
    assert_eq!(number, [0, 0, 255]);
}

#[test]
fn test_add_one_to_big_number_carry() {
    let mut number = [1, 2, 255];
    add_one_to_big_number(&mut number);
    assert_eq!(number, [1, 3, 0]);
}

#[test]
fn test_insert_facets_and_isfrozen_slots() {
    let mut slot_names_map = HashMap::new();
    let facets_len = 2;

    insert_facets_and_isfrozen_slots(&mut slot_names_map, facets_len);

    assert_eq!(slot_names_map.len(), facets_len + 2);

    assert!(slot_names_map
        .get(&H256([
            200, 252, 173, 141, 184, 77, 60, 193, 139, 76, 65, 213, 81, 234, 14, 230, 109, 213,
            153, 205, 224, 104, 217, 152, 229, 125, 94, 9, 51, 44, 19, 29
        ]))
        .is_some());
    assert!(slot_names_map
        .get(&H256([
            192, 215, 39, 97, 14, 161, 98, 65, 239, 244, 68, 125, 8, 187, 27, 69, 149, 247, 210,
            236, 69, 21, 40, 36, 55, 161, 59, 125, 13, 244, 185, 34
        ]))
        .is_some());
}