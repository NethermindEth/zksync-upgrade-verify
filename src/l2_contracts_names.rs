use ethers::types::{Address, H160};

// Get System contract name by L2 address
// https://github.com/matter-labs/era-contracts/blob/4aa7006153ad571643342dff22c16eaf4a70fdc1/system-contracts/contracts/Constants.sol#L36
pub fn get_system_contract_name(address: &Address) -> String {
    match address {
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x01]) => {
            "ECRECOVER_SYSTEM_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x02]) => {
            "SHA256_SYSTEM_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x06]) => {
            "ECADD_SYSTEM_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x07]) => {
            "ECMUL_SYSTEM_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x01]) => {
            "BOOTLOADER_FORMAL_ADDRESS".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x02]) => {
            "ACCOUNT_CODE_STORAGE_SYSTEM_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x03]) => {
            "NONCE_HOLDER_SYSTEM_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x04]) => {
            "KNOWN_CODE_STORAGE_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x05]) => {
            "IMMUTABLE_SIMULATOR_SYSTEM_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x06]) => {
            "DEPLOYER_SYSTEM_CONTRACT".to_string()
        }
        // 0x8007 is a ForceDeployer address (which is not a system contract, but conventional address)
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x07]) => {
            "FORCE_DEPLOYER".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x08]) => {
            "L1_MESSENGER_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x09]) => {
            "MSG_VALUE_SYSTEM_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x0a]) => {
            "ETH_TOKEN_SYSTEM_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x0b]) => {
            "SYSTEM_CONTEXT_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x0c]) => {
            "BOOTLOADER_UTILITIES".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x0d]) => {
            "EVENT_WRITER_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x0e]) => {
            "COMPRESSOR_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x0f]) => {
            "COMPLEX_UPGRADER_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x10]) => {
            "KECCAK256_SYSTEM_CONTRACT".to_string()
        }
        H160([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80, 0x11]) => {
            "PUBDATA_CHUNK_PUBLISHER".to_string()
        }
        _ => format!("0x{:02x}", address).to_string(),
    }
}
