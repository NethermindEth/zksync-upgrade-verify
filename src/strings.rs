pub fn get_dimond_storage_facets_string(i: u64) -> String {
    return format!("\x1b[38;5;117mDimondStorage.facets[{}]\n \x1b[38;5;115mThe array of all unique facet addresses that belong to the diamond proxy\x1b[0m", i);
}
pub fn get_facet_position_string(facet: &str) -> String {
    return format!("\x1b[38;5;117mfacetToSelectors[ 0x{} ].facetPosition\n \x1b[38;5;115mIndex in `DiamondStorage.facets` array, where is facet stored\x1b[0m", facet);
}
pub fn get_facet_to_selector_slots_string(facet: &str, i: i32, count: i32) -> String {
    return format!("\x1b[38;5;117mfacetToSelectors[ 0x{} ].selectors[] slot({}/{})\n \x1b[38;5;115mList of all selectors that belong to the facet\x1b[0m", facet,i,count);
}
pub fn get_selector_to_facet_slots0_string(selector: &str) -> String {
    return format!("\x1b[38;5;117mselectorToFacet[ 0x{} ].facetAddress\n \x1b[38;5;115mAddress of the facet which is connected with selector\x1b[0m", selector);
}
pub fn get_selector_to_facet_slots1_string(selector: &str) -> String {
    return format!("\x1b[38;5;117mselectorToFacet[ 0x{} ].(selectorPosition,isFreezable)\n \x1b[38;5;115mselectorPosition index in `FacetToSelectors.selectors` array, where is selector stored\nisFreezable denotes whether the selector can be frozen.\x1b[0m", selector);
}

pub const DIAMOND_STORAGE_ISFROZEN: &str = "\x1b[38;5;117mDimondStorage.isFrozen\n \x1b[38;5;115mDenotes whether the diamond proxy is frozen and all freezable facets are not accessible\x1b[0m";

pub const DIAMOND_STORAGE_FACETS_LENGTH: &str = "\x1b[38;5;117mDimondStorage.facets.length\n \x1b[38;5;115mThe length of the array of all unique facet addresses that belong to the diamond proxy\x1b[0m";

pub const PRSISTENT_SLOT_NAMES: [&str; 40] = [
    "\x1b[38;5;117m__DEPRECATED_diamondCutStorage[1]\n \x1b[38;5;115mStorage of variables needed for deprecated diamond cut facet\x1b[0m",
    "\x1b[38;5;117m__DEPRECATED_diamondCutStorage[2]\n \x1b[38;5;115mStorage of variables needed for deprecated diamond cut facet\x1b[0m",
    "\x1b[38;5;117m__DEPRECATED_diamondCutStorage[3]\n \x1b[38;5;115mStorage of variables needed for deprecated diamond cut facet\x1b[0m",
    "\x1b[38;5;117m__DEPRECATED_diamondCutStorage[4]\n \x1b[38;5;115mStorage of variables needed for deprecated diamond cut facet\x1b[0m",
    "\x1b[38;5;117m__DEPRECATED_diamondCutStorage[5]\n \x1b[38;5;115mStorage of variables needed for deprecated diamond cut facet\x1b[0m",
    "\x1b[38;5;117m__DEPRECATED_diamondCutStorage[6]\n \x1b[38;5;115mStorage of variables needed for deprecated diamond cut facet\x1b[0m",
    "\x1b[38;5;117m__DEPRECATED_diamondCutStorage[7]\n \x1b[38;5;115mStorage of variables needed for deprecated diamond cut facet\x1b[0m",
    "\x1b[38;5;117mgovernor\n \x1b[38;5;115mAddress which will exercise critical changes to the Diamond Proxy (upgrades, freezing & unfreezing)\x1b[0m",
    "\x1b[38;5;117mpendingGovernor\n \x1b[38;5;115mAddress that the governor proposed as one that will replace it\x1b[0m",
    "\x1b[38;5;117mvalidators\n \x1b[38;5;115mList of permitted validators\x1b[0m",
    "\x1b[38;5;117mverifier\n \x1b[38;5;115mVerifier contract. Used to verify aggregated proof for batches\x1b[0m",
    "\x1b[38;5;117mtotalBatchesExecuted\n \x1b[38;5;115mTotal number of executed batches i.e. batches[totalBatchesExecuted] points at the latest executed batch (batch 0 is genesis)\x1b[0m",
    "\x1b[38;5;117mtotalBatchesVerified\n \x1b[38;5;115mTotal number of proved batches i.e. batches[totalBatchesProved] points at the latest proved batch\x1b[0m",
    "\x1b[38;5;117mtotalBatchesCommitted\n \x1b[38;5;115mTotal number of committed batches i.e. batches[totalBatchesCommitted] points at the latest committed batch\x1b[0m",
    "\x1b[38;5;117mstoredBatchHashes\n \x1b[38;5;115mStored hashed StoredBatch for batch number\x1b[0m",
    "\x1b[38;5;117ml2LogsRootHashes\n \x1b[38;5;115mStored root hashes of L2 -> L1 logs\x1b[0m",
    "\x1b[38;5;117mpriorityQueue.data\n \x1b[38;5;115mContainer that stores transactions requested from L1. data is the inner mapping that saves priority operation by its index\x1b[0m",
    "\x1b[38;5;117mpriorityQueue.tail\n \x1b[38;5;115mContainer that stores transactions requested from L1. tail is the pointer to the free slot\x1b[0m",
    "\x1b[38;5;117mpriorityQueue.head\n \x1b[38;5;115mContainer that stores transactions requested from L1. head is the pointer to the first unprocessed priority operation, equal to the tail if the queue is empty\x1b[0m",
    "\x1b[38;5;117m__DEPRECATED_allowList\n \x1b[38;5;115mThe smart contract that manages the list with permission to call contract functions\x1b[0m",
    "\x1b[38;5;117mverifierParams.recursionNodeLevelVkHash\n \x1b[38;5;115mPart of the configuration parameters of ZKP circuits. Used as an input for the verifier smart contract\x1b[0m",
    "\x1b[38;5;117mverifierParams.recursionLeafLevelVkHash\n \x1b[38;5;115mPart of the configuration parameters of ZKP circuits. Used as an input for the verifier smart contract\x1b[0m",
    "\x1b[38;5;117mverifierParams.recursionCircuitsSetVksHash\n \x1b[38;5;115mPart of the configuration parameters of ZKP circuits. Used as an input for the verifier smart contract\x1b[0m",
    "\x1b[38;5;117ml2BootloaderBytecodeHash\n \x1b[38;5;115mBytecode hash of bootloader program. Used as an input to zkp-circuit.\x1b[0m",
    "\x1b[38;5;117ml2DefaultAccountBytecodeHash\n \x1b[38;5;115mBytecode hash of default account (bytecode for EOA). Used as an input to zkp-circuit.\x1b[0m",
    "\x1b[38;5;117mzkPorterIsAvailable\n \x1b[38;5;115mIndicates that the porter may be touched on L2 transactions. Used as an input to zkp-circuit.\x1b[0m",
    "\x1b[38;5;117mpriorityTxMaxGasLimit\n \x1b[38;5;115mThe maximum number of the L2 gas that a user can request for L1 -> L2 transactions. This is the maximum number of L2 gas that is available for the \"body\" of the transaction, i.e. without overhead for proving the batch.\x1b[0m",
    "\x1b[38;5;117m__DEPRECATED_upgrades.proposedUpgradeHash\n \x1b[38;5;115mStorage of variables needed for upgrade facet\x1b[0m",
    "\x1b[38;5;117m__DEPRECATED_upgrades.slot(1)\n \x1b[38;5;115mStorage of variables needed for upgrade facet\x1b[0m",
    "\x1b[38;5;117misEthWithdrawalFinalized\n \x1b[38;5;115mA mapping L2 batch number => message number => flag. The L2 -> L1 log is sent for every withdrawal, so this mapping is serving as a flag to indicate that the message was already processed. Used to indicate that eth withdrawal was already processed\x1b[0m",
    "\x1b[38;5;117m__DEPRECATED_lastWithdrawalLimitReset\n \x1b[38;5;115mThe most recent withdrawal time and amount reset\x1b[0m",
    "\x1b[38;5;117m__DEPRECATED_withdrawnAmountInWindow\n \x1b[38;5;115mThe accumulated withdrawn amount during the withdrawal limit window\x1b[0m",
    "\x1b[38;5;117m__DEPRECATED_totalDepositedAmountPerUser\n \x1b[38;5;115mA mapping user address => the total deposited amount by the user\x1b[0m",
    "\x1b[38;5;117mprotocolVersion\n \x1b[38;5;115mStores the protocol version. Note, that the protocol version may not only encompass changes to the smart contracts, but also to the node behavior.\x1b[0m",
    "\x1b[38;5;117ml2SystemContractsUpgradeTxHash\n \x1b[38;5;115mHash of the system contract upgrade transaction. If 0, then no upgrade transaction needs to be done.\x1b[0m",
    "\x1b[38;5;117ml2SystemContractsUpgradeBatchNumber\n \x1b[38;5;115mBatch number where the upgrade transaction has happened. If 0, then no upgrade transaction has happened yet.\x1b[0m",
    "\x1b[38;5;117madmin\n \x1b[38;5;115m Address which will exercise non-critical changes to the Diamond Proxy (changing validator set & unfreezing)\x1b[0m",
    "\x1b[38;5;117mpendingAdmin\n \x1b[38;5;115mAddress that the governor or admin proposed as one that will replace admin role\x1b[0m",
    "\x1b[38;5;117mfeeParams\n \x1b[38;5;115mFee params used to derive gasPrice for the L1->L2 transactions. For L2 transactions, the bootloader gives enough freedom to the operator.\x1b[0m",
    "\x1b[38;5;117mblobVersionedHashRetriever\n \x1b[38;5;115mAddress of the blob versioned hash getter smart contract used for EIP-4844 versioned hashes.\x1b[0m",
];
