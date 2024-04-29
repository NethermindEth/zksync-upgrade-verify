use ethers::types::{H160, U256};

// zkSync Era Diamond Proxy Contract address in Ethereum Mainnet
pub const ZKSYNC_ERA: H160 = H160([
    0x32, 0x40, 0x00, 0x84, 0xc2, 0x86, 0xcf, 0x3e, 0x17, 0xe7, 0xb6, 0x77, 0xea, 0x95, 0x83, 0xe6,
    0x0a, 0x00, 0x03, 0x24,
]);

// DimondStorage,facets[] slot
pub const FACETS_LEN_SLOT: [u8; 32] = [
    200, 252, 173, 141, 184, 77, 60, 193, 139, 76, 65, 213, 81, 234, 14, 230, 109, 213, 153, 205,
    224, 104, 217, 152, 229, 125, 94, 9, 51, 44, 19, 29,
];
// DimondStorage,facets[] data slot
pub const FACETS_DATA_SLOT: [u8; 32] = [
    192, 215, 39, 97, 14, 161, 98, 65, 239, 244, 68, 125, 8, 187, 27, 69, 149, 247, 210, 236, 69,
    21, 40, 36, 55, 161, 59, 125, 13, 244, 185, 34,
];

// bytes4 padded 32 bytes . Diamond.DiamondStorageSlot
pub const ENCODED_SELECTOR: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    200, 252, 173, 141, 184, 77, 60, 193, 139, 76, 65, 213, 81, 234, 14, 230, 109, 213, 153, 205,
    224, 104, 217, 152, 229, 125, 94, 9, 51, 44, 19, 27,
];

// zero 32 bytes
pub const ZERO: [u8; 32] = [0; 32];

// Upgrade tx type = 254
// Little-endian large integer.
pub const TX_TYPE_UPGGADE: U256 = U256([0xfe, 0, 0, 0]);
