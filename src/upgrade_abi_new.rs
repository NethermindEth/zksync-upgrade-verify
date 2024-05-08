pub use upgrade_abi_new::*;
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
pub mod upgrade_abi_new {
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
                        ],),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("struct ProposedUpgrade"),
                        ),
                    },],
                    outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("txHash"),
                        kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes32"),
                        ),
                    },],
                    constant: ::core::option::Option::None,
                    state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                },],
            )]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("NewL2BootloaderBytecodeHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewL2BootloaderBytecodeHash",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousBytecodeHash",),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newBytecodeHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewL2DefaultAccountBytecodeHash"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewL2DefaultAccountBytecodeHash",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousBytecodeHash",),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newBytecodeHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewProtocolVersion"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewProtocolVersion"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("previousProtocolVersion",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newProtocolVersion",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewVerifier"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewVerifier"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("oldVerifier"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newVerifier"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewVerifierParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("NewVerifierParams"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("oldVerifierParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ],),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newVerifierParams"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UpgradeComplete"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("UpgradeComplete"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newProtocolVersion",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("l2UpgradeTxHash"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("upgrade"),
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
                                                ::ethers::core::abi::ethabi::ParamType::Uint(
                                                    256usize
                                                ),
                                            ),
                                            4usize,
                                        ),
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ::ethers::core::abi::ethabi::ParamType::Array(
                                            ::std::boxed::Box::new(
                                                ::ethers::core::abi::ethabi::ParamType::Uint(
                                                    256usize
                                                ),
                                            ),
                                        ),
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ],),
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ),
                                    ),
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
                                ],),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static UPGRADEABINEW_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct UpgradeAbiNew<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UpgradeAbiNew<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UpgradeAbiNew<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UpgradeAbiNew<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UpgradeAbiNew<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(UpgradeAbiNew))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UpgradeAbiNew<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                UPGRADEABINEW_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `upgrade` (0x08284e57) function
        pub fn upgrade(
            &self,
            proposed_upgrade: ProposedUpgrade,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([8, 40, 78, 87], (proposed_upgrade,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `NewL2BootloaderBytecodeHash` event
        pub fn new_l2_bootloader_bytecode_hash_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewL2BootloaderBytecodeHashFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewL2DefaultAccountBytecodeHash` event
        pub fn new_l2_default_account_bytecode_hash_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NewL2DefaultAccountBytecodeHashFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NewProtocolVersion` event
        pub fn new_protocol_version_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewProtocolVersionFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `NewVerifier` event
        pub fn new_verifier_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewVerifierFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `NewVerifierParams` event
        pub fn new_verifier_params_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NewVerifierParamsFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `UpgradeComplete` event
        pub fn upgrade_complete_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpgradeCompleteFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpgradeAbiNewEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for UpgradeAbiNew<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "NewL2BootloaderBytecodeHash",
        abi = "NewL2BootloaderBytecodeHash(bytes32,bytes32)"
    )]
    pub struct NewL2BootloaderBytecodeHashFilter {
        #[ethevent(indexed)]
        pub previous_bytecode_hash: [u8; 32],
        #[ethevent(indexed)]
        pub new_bytecode_hash: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "NewL2DefaultAccountBytecodeHash",
        abi = "NewL2DefaultAccountBytecodeHash(bytes32,bytes32)"
    )]
    pub struct NewL2DefaultAccountBytecodeHashFilter {
        #[ethevent(indexed)]
        pub previous_bytecode_hash: [u8; 32],
        #[ethevent(indexed)]
        pub new_bytecode_hash: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "NewProtocolVersion",
        abi = "NewProtocolVersion(uint256,uint256)"
    )]
    pub struct NewProtocolVersionFilter {
        #[ethevent(indexed)]
        pub previous_protocol_version: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub new_protocol_version: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "NewVerifier", abi = "NewVerifier(address,address)")]
    pub struct NewVerifierFilter {
        #[ethevent(indexed)]
        pub old_verifier: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_verifier: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "NewVerifierParams",
        abi = "NewVerifierParams((bytes32,bytes32,bytes32),(bytes32,bytes32,bytes32))"
    )]
    pub struct NewVerifierParamsFilter {
        pub old_verifier_params: VerifierParams,
        pub new_verifier_params: VerifierParams,
    }
    #[derive(Clone, ::ethers::contract::EthEvent, ::ethers::contract::EthDisplay)]
    #[ethevent(
        name = "UpgradeComplete",
        abi = "UpgradeComplete(uint256,bytes32,((uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256[4],bytes,bytes,uint256[],bytes,bytes),bytes[],bytes32,bytes32,address,(bytes32,bytes32,bytes32),bytes,bytes,uint256,uint256))"
    )]
    pub struct UpgradeCompleteFilter {
        #[ethevent(indexed)]
        pub new_protocol_version: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub l_2_upgrade_tx_hash: [u8; 32],
        pub upgrade: ProposedUpgrade,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType)]
    pub enum UpgradeAbiNewEvents {
        NewL2BootloaderBytecodeHashFilter(NewL2BootloaderBytecodeHashFilter),
        NewL2DefaultAccountBytecodeHashFilter(NewL2DefaultAccountBytecodeHashFilter),
        NewProtocolVersionFilter(NewProtocolVersionFilter),
        NewVerifierFilter(NewVerifierFilter),
        NewVerifierParamsFilter(NewVerifierParamsFilter),
        UpgradeCompleteFilter(UpgradeCompleteFilter),
    }
    impl ::ethers::contract::EthLogDecode for UpgradeAbiNewEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = NewL2BootloaderBytecodeHashFilter::decode_log(log) {
                return Ok(UpgradeAbiNewEvents::NewL2BootloaderBytecodeHashFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewL2DefaultAccountBytecodeHashFilter::decode_log(log) {
                return Ok(UpgradeAbiNewEvents::NewL2DefaultAccountBytecodeHashFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = NewProtocolVersionFilter::decode_log(log) {
                return Ok(UpgradeAbiNewEvents::NewProtocolVersionFilter(decoded));
            }
            if let Ok(decoded) = NewVerifierFilter::decode_log(log) {
                return Ok(UpgradeAbiNewEvents::NewVerifierFilter(decoded));
            }
            if let Ok(decoded) = NewVerifierParamsFilter::decode_log(log) {
                return Ok(UpgradeAbiNewEvents::NewVerifierParamsFilter(decoded));
            }
            if let Ok(decoded) = UpgradeCompleteFilter::decode_log(log) {
                return Ok(UpgradeAbiNewEvents::UpgradeCompleteFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for UpgradeAbiNewEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::NewL2BootloaderBytecodeHashFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewL2DefaultAccountBytecodeHashFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NewProtocolVersionFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewVerifierFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewVerifierParamsFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeCompleteFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<NewL2BootloaderBytecodeHashFilter> for UpgradeAbiNewEvents {
        fn from(value: NewL2BootloaderBytecodeHashFilter) -> Self {
            Self::NewL2BootloaderBytecodeHashFilter(value)
        }
    }
    impl ::core::convert::From<NewL2DefaultAccountBytecodeHashFilter> for UpgradeAbiNewEvents {
        fn from(value: NewL2DefaultAccountBytecodeHashFilter) -> Self {
            Self::NewL2DefaultAccountBytecodeHashFilter(value)
        }
    }
    impl ::core::convert::From<NewProtocolVersionFilter> for UpgradeAbiNewEvents {
        fn from(value: NewProtocolVersionFilter) -> Self {
            Self::NewProtocolVersionFilter(value)
        }
    }
    impl ::core::convert::From<NewVerifierFilter> for UpgradeAbiNewEvents {
        fn from(value: NewVerifierFilter) -> Self {
            Self::NewVerifierFilter(value)
        }
    }
    impl ::core::convert::From<NewVerifierParamsFilter> for UpgradeAbiNewEvents {
        fn from(value: NewVerifierParamsFilter) -> Self {
            Self::NewVerifierParamsFilter(value)
        }
    }
    impl ::core::convert::From<UpgradeCompleteFilter> for UpgradeAbiNewEvents {
        fn from(value: UpgradeCompleteFilter) -> Self {
            Self::UpgradeCompleteFilter(value)
        }
    }
    ///Container type for all input parameters for the `upgrade` function with signature `upgrade(((uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256[4],bytes,bytes,uint256[],bytes,bytes),bytes[],bytes32,bytes32,address,(bytes32,bytes32,bytes32),bytes,bytes,uint256,uint256))` and selector `0x08284e57`
    #[derive(Debug, Clone, ::ethers::contract::EthCall, ::ethers::contract::EthDisplay)]
    #[ethcall(
        name = "upgrade",
        abi = "upgrade(((uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256[4],bytes,bytes,uint256[],bytes,bytes),bytes[],bytes32,bytes32,address,(bytes32,bytes32,bytes32),bytes,bytes,uint256,uint256))"
    )]
    pub struct UpgradeCall {
        pub proposed_upgrade: ProposedUpgrade,
    }
    ///Container type for all return fields from the `upgrade` function with signature `upgrade(((uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256[4],bytes,bytes,uint256[],bytes,bytes),bytes[],bytes32,bytes32,address,(bytes32,bytes32,bytes32),bytes,bytes,uint256,uint256))` and selector `0x08284e57`
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
    pub struct UpgradeReturn {
        pub tx_hash: [u8; 32],
    }
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
    ///`ProposedUpgrade((uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256,uint256[4],bytes,bytes,uint256[],bytes,bytes),bytes[],bytes32,bytes32,address,(bytes32,bytes32,bytes32),bytes,bytes,uint256,uint256)`
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
