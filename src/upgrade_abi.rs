use ethers::contract::abigen;
pub use upgrade_abi::*;

abigen!(
    Contract,
    r#"[
        function execTransaction(address to, uint256 value, bytes data, uint8 operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address refundReceiver, bytes signature)       

        struct Call {address target;uint256 value;bytes data;}
        struct Operation {Call[] calls;bytes32 predecessor;bytes32 salt;}
        function execute(Operation calldata _operation) external payable

        struct FacetCut {address facet;uint8 action;bool isFreezable;bytes4[] selectors;}
        struct DiamondCutData {FacetCut[] facetCuts;address initAddress;bytes initCalldata;}
        function executeUpgrade(DiamondCutData calldata _diamondCut) external

        struct ForceDeployment {bytes32 bytecodeHash;address newAddress;bool callConstructor;uint256 value;bytes input;}
        function forceDeployOnAddresses(ForceDeployment[] calldata _deployParams) external
  ]"#,
);
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod upgrade_abi {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("upgrade"),
                ::std::vec![::ethers::core::abi::ethabi::Function {
                    name: ::std::borrow::ToOwned::to_owned("upgrade"),
                    inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_proposedUpgrade"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                            ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                    4usize,
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            ::ethers::core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ),),
                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ::ethers::core::abi::ethabi::ParamType::Address,
                            ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                            ],),
                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ::ethers::core::abi::ethabi::ParamType::Address,
                        ],),
                        internal_type: ::core::option::Option::None,
                    },],
                    outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::string::String::new(),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                        internal_type: ::core::option::Option::None,
                    },],
                    constant: ::core::option::Option::None,
                    state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                },],
            )]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed human-readable ABI of the contract.
    pub static INITUPGRADE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct InitUpgrade<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for InitUpgrade<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for InitUpgrade<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for InitUpgrade<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for InitUpgrade<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(InitUpgrade))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> InitUpgrade<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                INITUPGRADE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `upgrade` (0x1ed824a0) function
        pub fn upgrade(
            &self,
            proposed_upgrade: ProposedUpgrade,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([30, 216, 36, 160], (proposed_upgrade,))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for InitUpgrade<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `upgrade` function with signature `upgrade(((uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256[4],bytes,bytes,uint256[],bytes,bytes),bytes[],bytes32,bytes32,address,(bytes32,bytes32,bytes32),bytes,bytes,uint256,uint256,address))` and selector `0x1ed824a0`
    #[derive(Clone, Debug, ::ethers::contract::EthCall, ::ethers::contract::EthDisplay)]
    #[ethcall(
        name = "upgrade",
        abi = "upgrade(((uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256[4],bytes,bytes,uint256[],bytes,bytes),bytes[],bytes32,bytes32,address,(bytes32,bytes32,bytes32),bytes,bytes,uint256,uint256,address))"
    )]
    pub struct UpgradeCall {
        pub proposed_upgrade: ProposedUpgrade,
    }
    ///Container type for all return fields from the `upgrade` function with signature `upgrade(((uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256[4],bytes,bytes,uint256[],bytes,bytes),bytes[],bytes32,bytes32,address,(bytes32,bytes32,bytes32),bytes,bytes,uint256,uint256,address))` and selector `0x1ed824a0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UpgradeReturn(pub [u8; 32]);
    ///`L2CanonicalTransaction(uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256[4],bytes,bytes,uint256[],bytes,bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct L2CanonicalTransaction {
        pub tx_type: ::ethers::core::types::U256,
        pub from: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::U256,
        pub gas_limit: ::ethers::core::types::U256,
        pub gas_per_pubdata_byte_limit: ::ethers::core::types::U256,
        pub max_fee_per_gas: ::ethers::core::types::U256,
        pub max_priority_fee_per_gas: ::ethers::core::types::U256,
        pub paymaster: ::ethers::core::types::U256,
        pub nonce: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::U256,
        pub reserved: [::ethers::core::types::U256; 4],
        pub data: ::ethers::core::types::Bytes,
        pub signature: ::ethers::core::types::Bytes,
        pub factory_deps: ::std::vec::Vec<::ethers::core::types::U256>,
        pub paymaster_input: ::ethers::core::types::Bytes,
        pub reserved_dynamic: ::ethers::core::types::Bytes,
    }
    ///`ProposedUpgrade((uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256[4],bytes,bytes,uint256[],bytes,bytes),bytes[],bytes32,bytes32,address,(bytes32,bytes32,bytes32),bytes,bytes,uint256,uint256,address)`
    #[derive(Clone, Debug, ::ethers::contract::EthAbiType, ::ethers::contract::EthAbiCodec)]
    pub struct ProposedUpgrade {
        pub l_2_protocol_upgrade_tx: L2CanonicalTransaction,
        pub factory_deps: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub bootloader_hash: [u8; 32],
        pub default_account_hash: [u8; 32],
        pub verifier: ::ethers::core::types::Address,
        pub verifier_params: VerifierParams,
        pub l_1_contracts_upgrade_calldata: ::ethers::core::types::Bytes,
        pub post_upgrade_calldata: ::ethers::core::types::Bytes,
        pub upgrade_timestamp: ::ethers::core::types::U256,
        pub new_protocol_version: ::ethers::core::types::U256,
        pub new_allow_list: ::ethers::core::types::Address,
    }
    ///`VerifierParams(bytes32,bytes32,bytes32)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct VerifierParams {
        pub recursion_node_level_vk_hash: [u8; 32],
        pub recursion_leaf_level_vk_hash: [u8; 32],
        pub recursion_circuits_set_vks_hash: [u8; 32],
    }
}
