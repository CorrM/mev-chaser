pub use simulator_abi::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
non_camel_case_types,
)]
pub mod simulator_abi {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("simulateGetAmountsOutUniswapV2"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "simulateGetAmountsOutUniswapV2",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("router"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("simulateGetAmountsOutUniswapV3"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "simulateGetAmountsOutUniswapV3",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("router"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("outputToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("path"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("simulateMultiSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateMultiSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swaps"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct OneSwapInfo[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainSwaps"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "AddressInsufficientBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MultiSwapError"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("MultiSwapError"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapIndex"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("errorReason"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotSupportedAmmProtocolError"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotSupportedAmmProtocolError",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("protocol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum AmmProtocol"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeERC20FailedOperation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SIMULATORABI_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\x17\x19\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\x005W`\x005`\xE0\x1C\x80c&W\x01Z\x14a\0>W\x80cJ\x0BD\xA2\x14a\0pW\x80c\x951\xD1\x89\x14a\0\x90W\0[6a\0<W\0[\0[4\x80\x15a\0JW`\0\x80\xFD[Pa\0^a\0Y6`\x04a\x10#V[a\0\xB0V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0|W`\0\x80\xFD[Pa\0^a\0\x8B6`\x04a\x10\xD9V[a\x02WV[4\x80\x15a\0\x9CW`\0\x80\xFD[Pa\0^a\0\xAB6`\x04a\x122V[a\x02\xEAV[`\0`\x01`\x01`\xA0\x1B\x03\x85\x16a\x01\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuInvalid router address`P\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83Q\x11a\x01NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsPath cannot be empty``\x1B`D\x82\x01R`d\x01a\0\xFDV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x84\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xBB\x91\x90a\x12\xF4V[\x90Pa\x01\xD5\x87\x86\x86`\0a\x01\xD0B`<a\x13#V[a\x05\"V[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x91P`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02B\x91\x90a\x12\xF4V[a\x02L\x91\x90a\x136V[\x97\x96PPPPPPPV[`\0\x80\x83Q\x11a\x02\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FSwaps array cannot be empty\0\0\0\0\0`D\x82\x01R`d\x01a\0\xFDV[`\0a\x02\xB7\x84\x84`\0a\x06\x91V[\x90P\x80`\x01\x82Qa\x02\xC8\x91\x90a\x136V[\x81Q\x81\x10a\x02\xD8Wa\x02\xD8a\x13IV[` \x02` \x01\x01Q\x91PP[\x92\x91PPV[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16a\x03;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuInvalid router address`P\x1B`D\x82\x01R`d\x01a\0\xFDV[`\x02\x83Q\x10\x15a\x03\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\t-\xCE\xCC-\x8D,\x84\x0E\x0C.\x8D\x04\r\x8C\xAD\xCC\xEE\x8D`k\x1B`D\x82\x01R`d\x01a\0\xFDV[`\0\x83`\0\x81Q\x81\x10a\x03\x98Wa\x03\x98a\x13IV[` \x02` \x01\x01Q\x90Pa\x03\xC0\x85\x84\x83`\x01`\x01`\xA0\x1B\x03\x16a\nO\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0\x84`\x01\x86Qa\x03\xD1\x91\x90a\x136V[\x81Q\x81\x10a\x03\xE1Wa\x03\xE1a\x13IV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x046W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04Z\x91\x90a\x12\xF4V[\x90P`\0\x80a\x04v\x89\x89\x89\x84a\x04qB`<a\x13#V[a\n\xDFV[\x92P\x92PP\x81\x15a\x04\x9FW`\0\x81`@Qc\x8B3le`\xE0\x1B\x81R`\x04\x01a\0\xFD\x92\x91\x90a\x13\xAFV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x83\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\t\x91\x90a\x12\xF4V[a\x05\x13\x91\x90a\x136V[\x95PPPPPP[\x93\x92PPPV[`@\x80Q`\xA0\x81\x01\x82R\x85\x81R0` \x82\x01R\x80\x82\x01\x83\x90R``\x81\x81\x01\x86\x90R`\x80\x82\x01\x85\x90R\x91Qc\xC0K\x8DY`\xE0\x1B\x81R`\0\x92\x83\x92\x90\x91\x89\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xC0K\x8DY\x90a\x05\x7F\x90\x84\x90`\x04\x01a\x13\xD0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x05\xBAWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x05\xB7\x91\x81\x01\x90a\x12\xF4V[`\x01[a\x06iWa\x05\xC6a\x14(V[\x80c\x08\xC3y\xA0\x03a\x05\xF7WPa\x05\xDAa\x14DV[\x80a\x05\xE5WPa\x05\xF9V[`\0\x95P`\x01\x94P\x92Pa\x06\x86\x91PPV[P[=\x80\x80\x15a\x06#W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06(V[``\x91P[P`\0`\x01`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x13[\xDC\xDD\x1B\x1EH\x1C\x18Z\\\x88\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`Z\x1B\x81RP\x95P\x95P\x95PPPPa\x06\x86V[\x80`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x95P\x95P\x95PPPP[\x95P\x95P\x95\x92PPPV[``\x82\x80\x15a\x06\xA1WP`\x02\x84Q\x10[\x15a\x06\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FChainSwap requires at least 2 sw`D\x82\x01Rbaps`\xE8\x1B`d\x82\x01R`\x84\x01a\0\xFDV[`\0``\x83a\x07\nW`\x01a\x07\rV[\x85Q[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07%Wa\x07%a\x0FCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07NW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P`\0\x80`\0[\x88Q\x81\x10\x15a\n\x1DW`\0\x89\x82\x81Q\x81\x10a\x07tWa\x07ta\x13IV[` \x02` \x01\x01Q\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x07\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x14\x9B\xDD]\x19\\\x88\x1A\\\xC8\x1B\x9D[\x1B`\x92\x1B`D\x82\x01R`d\x01a\0\xFDV[`\0\x81``\x01QQ\x11a\x08\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x14\x18]\x1A\x08\x1A\\\xC8\x1B\x9D[\x1B`\xA2\x1B`D\x82\x01R`d\x01a\0\xFDV[`@\x81\x01Q\x89\x15a\x08\xD0W\x82`\0\x03a\x08<W\x81`\x80\x01Q\x94P`\0\x93Pa\x08\xDFV[`\x01\x8BQa\x08J\x91\x90a\x136V[\x83\x03a\x08]W`\xA0\x82\x01Q\x93\x94Pa\x08\xDFV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xC5\x91\x90a\x12\xF4V[\x94P`\0\x93Pa\x08\xDFV[\x81`\x80\x01Q\x94P\x81`\xA0\x01Q\x93P[` \x82\x01Qa\x08\xF9\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x87a\nOV[\x81`\xC0\x01Q`\0\x03a\t\x16Wa\t\x10B`<a\x13#V[`\xC0\x83\x01R[`\0\x82Q`\x01\x81\x11\x15a\t+Wa\t+a\x14\xCEV[\x03a\toW`\0\x82``\x01Q\x80` \x01\x90Q\x81\x01\x90a\tJ\x91\x90a\x14\xE4V[\x90Pa\ta\x83` \x01Q\x82\x88\x88\x87`\xC0\x01Qa\n\xDFV[\x90\x99P\x97P\x94Pa\t\xCA\x90PV[`\x01\x82Q`\x01\x81\x11\x15a\t\x84Wa\t\x84a\x14\xCEV[\x03a\t\xAEWa\t\xA2\x82` \x01Q\x83``\x01Q\x87\x87\x86`\xC0\x01Qa\x05\"V[\x90\x98P\x96P\x93Pa\t\xCAV[\x81Q`@Qc\x95\x9E\xD9\xB9`\xE0\x1B\x81Ra\0\xFD\x91\x90`\x04\x01a\x15~V[\x86\x15a\t\xEDW\x82\x86`@Qc\x8B3le`\xE0\x1B\x81R`\x04\x01a\0\xFD\x92\x91\x90a\x13\xAFV[\x88\x15a\n\x13W\x83\x88\x84\x81Q\x81\x10a\n\x06Wa\n\x06a\x13IV[` \x02` \x01\x01\x81\x81RPP[PP`\x01\x01a\x07WV[P\x85a\nDW\x80\x85`\0\x81Q\x81\x10a\n7Wa\n7a\x13IV[` \x02` \x01\x01\x81\x81RPP[PPPP\x93\x92PPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`\0\x91\x90\x85\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x9FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xC3\x91\x90a\x12\xF4V[\x90Pa\n\xD9\x84\x84a\n\xD4\x85\x85a\x13#V[a\x0CPV[PPPPV[`@Qc8\xED\x179`\xE0\x1B\x81R`\0\x90\x81\x90``\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c8\xED\x179\x90a\x0B\x1D\x90\x8A\x90\x8A\x90\x8D\x900\x90\x8C\x90`\x04\x01a\x15\xA6V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x0B]WP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0BZ\x91\x90\x81\x01\x90a\x16\x19V[`\x01[a\x0C\nWa\x0Bia\x14(V[\x80c\x08\xC3y\xA0\x03a\x0B\x99WPa\x0B}a\x14DV[\x80a\x0B\x88WPa\x0B\x9BV[`\0\x94P`\x01\x93P\x91Pa\x06\x86\x90PV[P[=\x80\x80\x15a\x0B\xC5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xCAV[``\x91P[P`\0`\x01`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x13[\xDC\xDD\x1B\x1EH\x1C\x18Z\\\x88\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`Z\x1B\x81RP\x94P\x94P\x94PPPa\x06\x86V[\x80`\x01\x82Qa\x0C\x19\x91\x90a\x136V[\x81Q\x81\x10a\x0C)Wa\x0C)a\x13IV[` \x02` \x01\x01Q`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x94P\x94P\x94PPPa\x06\x86V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x0C\xA1\x84\x82a\r\x04V[a\n\xD9W`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16`$\x82\x01R`\0`D\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x0C\xFA\x90\x85\x90a\r\xACV[a\n\xD9\x84\x82a\r\xACV[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\r!\x91\x90a\x16\xAAV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\r^W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\rcV[``\x91P[P\x91P\x91P\x81\x80\x15a\r\x8DWP\x80Q\x15\x80a\r\x8DWP\x80\x80` \x01\x90Q\x81\x01\x90a\r\x8D\x91\x90a\x16\xC6V[\x80\x15a\r\xA3WP`\0\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x95\x94PPPPPV[`\0a\r\xC1`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x0E\x14V[\x90P\x80Q`\0\x14\x15\x80\x15a\r\xE6WP\x80\x80` \x01\x90Q\x81\x01\x90a\r\xE4\x91\x90a\x16\xC6V[\x15[\x15a\x0E\x0FW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\0\xFDV[PPPV[``a\x05\x1B\x83\x83`\0\x84`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x0E:\x91\x90a\x16\xAAV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0EwW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0E|V[``\x91P[P\x91P\x91Pa\x0E\x8C\x86\x83\x83a\x0E\x96V[\x96\x95PPPPPPV[``\x82a\x0E\xABWa\x0E\xA6\x82a\x0E\xF2V[a\x05\x1BV[\x81Q\x15\x80\x15a\x0E\xC2WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x0E\xEBW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\0\xFDV[P\x80a\x05\x1BV[\x80Q\x15a\x0F\x02W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\x1BW`\0\x80\xFD[\x805a\x0F>\x81a\x0F\x1EV[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\x7FWa\x0F\x7Fa\x0FCV[`@RPPV[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\xA9Wa\x0F\xA9a\x0FCV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x0F\xC0W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xDAWa\x0F\xDAa\x0FCV[`@Qa\x0F\xF1`\x1F\x83\x01`\x1F\x19\x16` \x01\x82a\x0FYV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x10\x06W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x109W`\0\x80\xFD[\x845a\x10D\x81a\x0F\x1EV[\x93P` \x85\x015a\x10T\x81a\x0F\x1EV[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10pW`\0\x80\xFD[a\x10|\x87\x82\x88\x01a\x0F\xAFV[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10\xA7Wa\x10\xA7a\x0FCV[P`\x05\x1B` \x01\x90V[\x805`\x02\x81\x10a\x0F>W`\0\x80\xFD[\x80\x15\x15\x81\x14a\x0F\x1BW`\0\x80\xFD[\x805a\x0F>\x81a\x10\xC0V[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xECW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11\x04W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x11\x18W`\0\x80\xFD[\x815` a\x11%\x82a\x10\x8DV[`@Qa\x112\x82\x82a\x0FYV[\x83\x81R`\x05\x93\x90\x93\x1B\x85\x01\x82\x01\x92\x82\x81\x01\x91P\x89\x84\x11\x15a\x11RW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x12\x15W\x805\x86\x81\x11\x15a\x11mW`\0\x80\xFD[\x87\x01`\xE0\x81\x8D\x03`\x1F\x19\x01\x12\x15a\x11\x83W`\0\x80\xFD[a\x11\x8Ba\x0F\x86V[a\x11\x96\x86\x83\x01a\x10\xB1V[\x81Ra\x11\xA4`@\x83\x01a\x0F3V[\x86\x82\x01Ra\x11\xB4``\x83\x01a\x0F3V[`@\x82\x01R`\x80\x80\x83\x015\x89\x81\x11\x15a\x11\xCDW`\0\x80\x81\xFD[a\x11\xDB\x8F\x89\x83\x87\x01\x01a\x0F\xAFV[``\x84\x01RP`\xA0\x83\x81\x015\x91\x83\x01\x91\x90\x91R`\xC0\x80\x84\x015\x91\x83\x01\x91\x90\x91R`\xE0\x90\x92\x015\x91\x81\x01\x91\x90\x91R\x83R\x91\x83\x01\x91\x83\x01a\x11VV[P\x96Pa\x12%\x90P\x87\x82\x01a\x10\xCEV[\x94PPPPP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12GW`\0\x80\xFD[\x835a\x12R\x81a\x0F\x1EV[\x92P` \x84\x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12oW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x12\x80W`\0\x80\xFD[\x805a\x12\x8B\x81a\x10\x8DV[`@Qa\x12\x98\x82\x82a\x0FYV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x89\x83\x11\x15a\x12\xB8W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x12\xDFW\x835a\x12\xD0\x81a\x0F\x1EV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x12\xBDV[\x96\x99\x96\x98PPPP`@\x94\x90\x94\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x13\x06W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x02\xE4Wa\x02\xE4a\x13\rV[\x81\x81\x03\x81\x81\x11\x15a\x02\xE4Wa\x02\xE4a\x13\rV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x13zW\x81\x81\x01Q\x83\x82\x01R` \x01a\x13bV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x13\x9B\x81` \x86\x01` \x86\x01a\x13_V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x82\x81R`@` \x82\x01R`\0a\x13\xC8`@\x83\x01\x84a\x13\x83V[\x94\x93PPPPV[` \x81R`\0\x82Q`\xA0` \x84\x01Ra\x13\xEC`\xC0\x84\x01\x82a\x13\x83V[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`@\x84\x01Q``\x84\x01R``\x84\x01Q`\x80\x84\x01R`\x80\x84\x01Q`\xA0\x84\x01R\x80\x91PP\x92\x91PPV[`\0`\x03=\x11\x15a\x14AW`\x04`\0\x80>P`\0Q`\xE0\x1C[\x90V[`\0`D=\x10\x15a\x14RW\x90V[`@Q`\x03\x19=\x81\x01`\x04\x83>\x81Q=g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`$\x84\x01\x11\x81\x84\x11\x17\x15a\x14\x82WPPPPP\x90V[\x82\x85\x01\x91P\x81Q\x81\x81\x11\x15a\x14\x9AWPPPPPP\x90V[\x84=\x87\x01\x01` \x82\x85\x01\x01\x11\x15a\x14\xB4WPPPPPP\x90V[a\x14\xC3` \x82\x86\x01\x01\x87a\x0FYV[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x14\xF7W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\x0EW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x15\x1FW`\0\x80\xFD[\x80Qa\x15*\x81a\x10\x8DV[`@Qa\x157\x82\x82a\x0FYV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\x15WW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x02LW\x83Qa\x15o\x81a\x0F\x1EV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x15\\V[` \x81\x01`\x02\x83\x10a\x15\xA0WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0`\xA0\x82\x01\x87\x83R` \x87` \x85\x01R`\xA0`@\x85\x01R\x81\x87Q\x80\x84R`\xC0\x86\x01\x91P` \x89\x01\x93P`\0[\x81\x81\x10\x15a\x15\xF8W\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x15\xD3V[PP`\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16``\x85\x01RPPP`\x80\x01R\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a\x16,W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16CW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x16TW`\0\x80\xFD[\x80Qa\x16_\x81a\x10\x8DV[`@Qa\x16l\x82\x82a\x0FYV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\x16\x8CW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x02LW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x16\x91V[`\0\x82Qa\x16\xBC\x81\x84` \x87\x01a\x13_V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x16\xD8W`\0\x80\xFD[\x81Qa\x05\x1B\x81a\x10\xC0V\xFE\xA2dipfsX\"\x12 \x81\x10\x8F\xFC]\xAF\xF3\xD9%\xB2s\x81\x0C.\x1C\xE2w\xE5\xEB\x963:e\x0C\xA82\xF84\r\xEB\x93]dsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static SIMULATORABI_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x005W`\x005`\xE0\x1C\x80c&W\x01Z\x14a\0>W\x80cJ\x0BD\xA2\x14a\0pW\x80c\x951\xD1\x89\x14a\0\x90W\0[6a\0<W\0[\0[4\x80\x15a\0JW`\0\x80\xFD[Pa\0^a\0Y6`\x04a\x10#V[a\0\xB0V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0|W`\0\x80\xFD[Pa\0^a\0\x8B6`\x04a\x10\xD9V[a\x02WV[4\x80\x15a\0\x9CW`\0\x80\xFD[Pa\0^a\0\xAB6`\x04a\x122V[a\x02\xEAV[`\0`\x01`\x01`\xA0\x1B\x03\x85\x16a\x01\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuInvalid router address`P\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x83Q\x11a\x01NW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsPath cannot be empty``\x1B`D\x82\x01R`d\x01a\0\xFDV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x84\x90`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xBB\x91\x90a\x12\xF4V[\x90Pa\x01\xD5\x87\x86\x86`\0a\x01\xD0B`<a\x13#V[a\x05\"V[PP`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x82\x91P`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x1EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02B\x91\x90a\x12\xF4V[a\x02L\x91\x90a\x136V[\x97\x96PPPPPPPV[`\0\x80\x83Q\x11a\x02\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FSwaps array cannot be empty\0\0\0\0\0`D\x82\x01R`d\x01a\0\xFDV[`\0a\x02\xB7\x84\x84`\0a\x06\x91V[\x90P\x80`\x01\x82Qa\x02\xC8\x91\x90a\x136V[\x81Q\x81\x10a\x02\xD8Wa\x02\xD8a\x13IV[` \x02` \x01\x01Q\x91PP[\x92\x91PPV[`\0`\x01`\x01`\xA0\x1B\x03\x84\x16a\x03;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuInvalid router address`P\x1B`D\x82\x01R`d\x01a\0\xFDV[`\x02\x83Q\x10\x15a\x03\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\t-\xCE\xCC-\x8D,\x84\x0E\x0C.\x8D\x04\r\x8C\xAD\xCC\xEE\x8D`k\x1B`D\x82\x01R`d\x01a\0\xFDV[`\0\x83`\0\x81Q\x81\x10a\x03\x98Wa\x03\x98a\x13IV[` \x02` \x01\x01Q\x90Pa\x03\xC0\x85\x84\x83`\x01`\x01`\xA0\x1B\x03\x16a\nO\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\0\x84`\x01\x86Qa\x03\xD1\x91\x90a\x136V[\x81Q\x81\x10a\x03\xE1Wa\x03\xE1a\x13IV[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x046W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04Z\x91\x90a\x12\xF4V[\x90P`\0\x80a\x04v\x89\x89\x89\x84a\x04qB`<a\x13#V[a\n\xDFV[\x92P\x92PP\x81\x15a\x04\x9FW`\0\x81`@Qc\x8B3le`\xE0\x1B\x81R`\x04\x01a\0\xFD\x92\x91\x90a\x13\xAFV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x83\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\t\x91\x90a\x12\xF4V[a\x05\x13\x91\x90a\x136V[\x95PPPPPP[\x93\x92PPPV[`@\x80Q`\xA0\x81\x01\x82R\x85\x81R0` \x82\x01R\x80\x82\x01\x83\x90R``\x81\x81\x01\x86\x90R`\x80\x82\x01\x85\x90R\x91Qc\xC0K\x8DY`\xE0\x1B\x81R`\0\x92\x83\x92\x90\x91\x89\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xC0K\x8DY\x90a\x05\x7F\x90\x84\x90`\x04\x01a\x13\xD0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x05\xBAWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x05\xB7\x91\x81\x01\x90a\x12\xF4V[`\x01[a\x06iWa\x05\xC6a\x14(V[\x80c\x08\xC3y\xA0\x03a\x05\xF7WPa\x05\xDAa\x14DV[\x80a\x05\xE5WPa\x05\xF9V[`\0\x95P`\x01\x94P\x92Pa\x06\x86\x91PPV[P[=\x80\x80\x15a\x06#W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06(V[``\x91P[P`\0`\x01`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x13[\xDC\xDD\x1B\x1EH\x1C\x18Z\\\x88\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`Z\x1B\x81RP\x95P\x95P\x95PPPPa\x06\x86V[\x80`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x95P\x95P\x95PPPP[\x95P\x95P\x95\x92PPPV[``\x82\x80\x15a\x06\xA1WP`\x02\x84Q\x10[\x15a\x06\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FChainSwap requires at least 2 sw`D\x82\x01Rbaps`\xE8\x1B`d\x82\x01R`\x84\x01a\0\xFDV[`\0``\x83a\x07\nW`\x01a\x07\rV[\x85Q[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07%Wa\x07%a\x0FCV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07NW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P`\0\x80`\0[\x88Q\x81\x10\x15a\n\x1DW`\0\x89\x82\x81Q\x81\x10a\x07tWa\x07ta\x13IV[` \x02` \x01\x01Q\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x07\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x14\x9B\xDD]\x19\\\x88\x1A\\\xC8\x1B\x9D[\x1B`\x92\x1B`D\x82\x01R`d\x01a\0\xFDV[`\0\x81``\x01QQ\x11a\x08\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x14\x18]\x1A\x08\x1A\\\xC8\x1B\x9D[\x1B`\xA2\x1B`D\x82\x01R`d\x01a\0\xFDV[`@\x81\x01Q\x89\x15a\x08\xD0W\x82`\0\x03a\x08<W\x81`\x80\x01Q\x94P`\0\x93Pa\x08\xDFV[`\x01\x8BQa\x08J\x91\x90a\x136V[\x83\x03a\x08]W`\xA0\x82\x01Q\x93\x94Pa\x08\xDFV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xC5\x91\x90a\x12\xF4V[\x94P`\0\x93Pa\x08\xDFV[\x81`\x80\x01Q\x94P\x81`\xA0\x01Q\x93P[` \x82\x01Qa\x08\xF9\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x87a\nOV[\x81`\xC0\x01Q`\0\x03a\t\x16Wa\t\x10B`<a\x13#V[`\xC0\x83\x01R[`\0\x82Q`\x01\x81\x11\x15a\t+Wa\t+a\x14\xCEV[\x03a\toW`\0\x82``\x01Q\x80` \x01\x90Q\x81\x01\x90a\tJ\x91\x90a\x14\xE4V[\x90Pa\ta\x83` \x01Q\x82\x88\x88\x87`\xC0\x01Qa\n\xDFV[\x90\x99P\x97P\x94Pa\t\xCA\x90PV[`\x01\x82Q`\x01\x81\x11\x15a\t\x84Wa\t\x84a\x14\xCEV[\x03a\t\xAEWa\t\xA2\x82` \x01Q\x83``\x01Q\x87\x87\x86`\xC0\x01Qa\x05\"V[\x90\x98P\x96P\x93Pa\t\xCAV[\x81Q`@Qc\x95\x9E\xD9\xB9`\xE0\x1B\x81Ra\0\xFD\x91\x90`\x04\x01a\x15~V[\x86\x15a\t\xEDW\x82\x86`@Qc\x8B3le`\xE0\x1B\x81R`\x04\x01a\0\xFD\x92\x91\x90a\x13\xAFV[\x88\x15a\n\x13W\x83\x88\x84\x81Q\x81\x10a\n\x06Wa\n\x06a\x13IV[` \x02` \x01\x01\x81\x81RPP[PP`\x01\x01a\x07WV[P\x85a\nDW\x80\x85`\0\x81Q\x81\x10a\n7Wa\n7a\x13IV[` \x02` \x01\x01\x81\x81RPP[PPPP\x93\x92PPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`\0\x91\x90\x85\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x9FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xC3\x91\x90a\x12\xF4V[\x90Pa\n\xD9\x84\x84a\n\xD4\x85\x85a\x13#V[a\x0CPV[PPPPV[`@Qc8\xED\x179`\xE0\x1B\x81R`\0\x90\x81\x90``\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c8\xED\x179\x90a\x0B\x1D\x90\x8A\x90\x8A\x90\x8D\x900\x90\x8C\x90`\x04\x01a\x15\xA6V[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x0B]WP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0BZ\x91\x90\x81\x01\x90a\x16\x19V[`\x01[a\x0C\nWa\x0Bia\x14(V[\x80c\x08\xC3y\xA0\x03a\x0B\x99WPa\x0B}a\x14DV[\x80a\x0B\x88WPa\x0B\x9BV[`\0\x94P`\x01\x93P\x91Pa\x06\x86\x90PV[P[=\x80\x80\x15a\x0B\xC5W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0B\xCAV[``\x91P[P`\0`\x01`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x13[\xDC\xDD\x1B\x1EH\x1C\x18Z\\\x88\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`Z\x1B\x81RP\x94P\x94P\x94PPPa\x06\x86V[\x80`\x01\x82Qa\x0C\x19\x91\x90a\x136V[\x81Q\x81\x10a\x0C)Wa\x0C)a\x13IV[` \x02` \x01\x01Q`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x94P\x94P\x94PPPa\x06\x86V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x0C\xA1\x84\x82a\r\x04V[a\n\xD9W`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16`$\x82\x01R`\0`D\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x0C\xFA\x90\x85\x90a\r\xACV[a\n\xD9\x84\x82a\r\xACV[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\r!\x91\x90a\x16\xAAV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\r^W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\rcV[``\x91P[P\x91P\x91P\x81\x80\x15a\r\x8DWP\x80Q\x15\x80a\r\x8DWP\x80\x80` \x01\x90Q\x81\x01\x90a\r\x8D\x91\x90a\x16\xC6V[\x80\x15a\r\xA3WP`\0\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x95\x94PPPPPV[`\0a\r\xC1`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x0E\x14V[\x90P\x80Q`\0\x14\x15\x80\x15a\r\xE6WP\x80\x80` \x01\x90Q\x81\x01\x90a\r\xE4\x91\x90a\x16\xC6V[\x15[\x15a\x0E\x0FW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\0\xFDV[PPPV[``a\x05\x1B\x83\x83`\0\x84`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x0E:\x91\x90a\x16\xAAV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x0EwW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0E|V[``\x91P[P\x91P\x91Pa\x0E\x8C\x86\x83\x83a\x0E\x96V[\x96\x95PPPPPPV[``\x82a\x0E\xABWa\x0E\xA6\x82a\x0E\xF2V[a\x05\x1BV[\x81Q\x15\x80\x15a\x0E\xC2WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x0E\xEBW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\0\xFDV[P\x80a\x05\x1BV[\x80Q\x15a\x0F\x02W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0F\x1BW`\0\x80\xFD[\x805a\x0F>\x81a\x0F\x1EV[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\x7FWa\x0F\x7Fa\x0FCV[`@RPPV[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0F\xA9Wa\x0F\xA9a\x0FCV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x0F\xC0W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\xDAWa\x0F\xDAa\x0FCV[`@Qa\x0F\xF1`\x1F\x83\x01`\x1F\x19\x16` \x01\x82a\x0FYV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x10\x06W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x109W`\0\x80\xFD[\x845a\x10D\x81a\x0F\x1EV[\x93P` \x85\x015a\x10T\x81a\x0F\x1EV[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10pW`\0\x80\xFD[a\x10|\x87\x82\x88\x01a\x0F\xAFV[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x10\xA7Wa\x10\xA7a\x0FCV[P`\x05\x1B` \x01\x90V[\x805`\x02\x81\x10a\x0F>W`\0\x80\xFD[\x80\x15\x15\x81\x14a\x0F\x1BW`\0\x80\xFD[\x805a\x0F>\x81a\x10\xC0V[`\0\x80`@\x83\x85\x03\x12\x15a\x10\xECW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x11\x04W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x11\x18W`\0\x80\xFD[\x815` a\x11%\x82a\x10\x8DV[`@Qa\x112\x82\x82a\x0FYV[\x83\x81R`\x05\x93\x90\x93\x1B\x85\x01\x82\x01\x92\x82\x81\x01\x91P\x89\x84\x11\x15a\x11RW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x12\x15W\x805\x86\x81\x11\x15a\x11mW`\0\x80\xFD[\x87\x01`\xE0\x81\x8D\x03`\x1F\x19\x01\x12\x15a\x11\x83W`\0\x80\xFD[a\x11\x8Ba\x0F\x86V[a\x11\x96\x86\x83\x01a\x10\xB1V[\x81Ra\x11\xA4`@\x83\x01a\x0F3V[\x86\x82\x01Ra\x11\xB4``\x83\x01a\x0F3V[`@\x82\x01R`\x80\x80\x83\x015\x89\x81\x11\x15a\x11\xCDW`\0\x80\x81\xFD[a\x11\xDB\x8F\x89\x83\x87\x01\x01a\x0F\xAFV[``\x84\x01RP`\xA0\x83\x81\x015\x91\x83\x01\x91\x90\x91R`\xC0\x80\x84\x015\x91\x83\x01\x91\x90\x91R`\xE0\x90\x92\x015\x91\x81\x01\x91\x90\x91R\x83R\x91\x83\x01\x91\x83\x01a\x11VV[P\x96Pa\x12%\x90P\x87\x82\x01a\x10\xCEV[\x94PPPPP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x12GW`\0\x80\xFD[\x835a\x12R\x81a\x0F\x1EV[\x92P` \x84\x81\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12oW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x12\x80W`\0\x80\xFD[\x805a\x12\x8B\x81a\x10\x8DV[`@Qa\x12\x98\x82\x82a\x0FYV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x89\x83\x11\x15a\x12\xB8W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x12\xDFW\x835a\x12\xD0\x81a\x0F\x1EV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x12\xBDV[\x96\x99\x96\x98PPPP`@\x94\x90\x94\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a\x13\x06W`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x02\xE4Wa\x02\xE4a\x13\rV[\x81\x81\x03\x81\x81\x11\x15a\x02\xE4Wa\x02\xE4a\x13\rV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x13zW\x81\x81\x01Q\x83\x82\x01R` \x01a\x13bV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x13\x9B\x81` \x86\x01` \x86\x01a\x13_V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x82\x81R`@` \x82\x01R`\0a\x13\xC8`@\x83\x01\x84a\x13\x83V[\x94\x93PPPPV[` \x81R`\0\x82Q`\xA0` \x84\x01Ra\x13\xEC`\xC0\x84\x01\x82a\x13\x83V[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`@\x84\x01Q``\x84\x01R``\x84\x01Q`\x80\x84\x01R`\x80\x84\x01Q`\xA0\x84\x01R\x80\x91PP\x92\x91PPV[`\0`\x03=\x11\x15a\x14AW`\x04`\0\x80>P`\0Q`\xE0\x1C[\x90V[`\0`D=\x10\x15a\x14RW\x90V[`@Q`\x03\x19=\x81\x01`\x04\x83>\x81Q=g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`$\x84\x01\x11\x81\x84\x11\x17\x15a\x14\x82WPPPPP\x90V[\x82\x85\x01\x91P\x81Q\x81\x81\x11\x15a\x14\x9AWPPPPPP\x90V[\x84=\x87\x01\x01` \x82\x85\x01\x01\x11\x15a\x14\xB4WPPPPPP\x90V[a\x14\xC3` \x82\x86\x01\x01\x87a\x0FYV[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x14\xF7W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\x0EW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x15\x1FW`\0\x80\xFD[\x80Qa\x15*\x81a\x10\x8DV[`@Qa\x157\x82\x82a\x0FYV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\x15WW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x02LW\x83Qa\x15o\x81a\x0F\x1EV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x15\\V[` \x81\x01`\x02\x83\x10a\x15\xA0WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0`\xA0\x82\x01\x87\x83R` \x87` \x85\x01R`\xA0`@\x85\x01R\x81\x87Q\x80\x84R`\xC0\x86\x01\x91P` \x89\x01\x93P`\0[\x81\x81\x10\x15a\x15\xF8W\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x15\xD3V[PP`\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16``\x85\x01RPPP`\x80\x01R\x93\x92PPPV[`\0` \x80\x83\x85\x03\x12\x15a\x16,W`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16CW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x16TW`\0\x80\xFD[\x80Qa\x16_\x81a\x10\x8DV[`@Qa\x16l\x82\x82a\x0FYV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\x16\x8CW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x02LW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x16\x91V[`\0\x82Qa\x16\xBC\x81\x84` \x87\x01a\x13_V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x16\xD8W`\0\x80\xFD[\x81Qa\x05\x1B\x81a\x10\xC0V\xFE\xA2dipfsX\"\x12 \x81\x10\x8F\xFC]\xAF\xF3\xD9%\xB2s\x81\x0C.\x1C\xE2w\xE5\xEB\x963:e\x0C\xA82\xF84\r\xEB\x93]dsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static SIMULATORABI_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SimulatorAbi<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SimulatorAbi<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SimulatorAbi<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SimulatorAbi<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SimulatorAbi<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SimulatorAbi))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SimulatorAbi<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SIMULATORABI_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                SIMULATORABI_ABI.clone(),
                SIMULATORABI_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `simulateGetAmountsOutUniswapV2` (0x9531d189) function
        pub fn simulate_get_amounts_out_uniswap_v2(
            &self,
            router: ::ethers::core::types::Address,
            path: ::std::vec::Vec<::ethers::core::types::Address>,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([149, 49, 209, 137], (router, path, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateGetAmountsOutUniswapV3` (0x2657015a) function
        pub fn simulate_get_amounts_out_uniswap_v3(
            &self,
            router: ::ethers::core::types::Address,
            output_token: ::ethers::core::types::Address,
            path: ::ethers::core::types::Bytes,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([38, 87, 1, 90], (router, output_token, path, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateMultiSwap` (0x4a0b44a2) function
        pub fn simulate_multi_swap(
            &self,
            swaps: ::std::vec::Vec<OneSwapInfo>,
            chain_swaps: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([74, 11, 68, 162], (swaps, chain_swaps))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SimulatorAbi<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
    #[derive(
    Clone,
    ::ethers::contract::EthError,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `AddressInsufficientBalance` with signature `AddressInsufficientBalance(address)` and selector `0xcd786059`
    #[derive(
    Clone,
    ::ethers::contract::EthError,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[etherror(
    name = "AddressInsufficientBalance",
    abi = "AddressInsufficientBalance(address)"
    )]
    pub struct AddressInsufficientBalance {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
    #[derive(
    Clone,
    ::ethers::contract::EthError,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
    ///Custom Error type `MultiSwapError` with signature `MultiSwapError(uint256,string)` and selector `0x8b336c65`
    #[derive(
    Clone,
    ::ethers::contract::EthError,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[etherror(name = "MultiSwapError", abi = "MultiSwapError(uint256,string)")]
    pub struct MultiSwapError {
        pub swap_index: ::ethers::core::types::U256,
        pub error_reason: ::std::string::String,
    }
    ///Custom Error type `NotSupportedAmmProtocolError` with signature `NotSupportedAmmProtocolError(uint8)` and selector `0x959ed9b9`
    #[derive(
    Clone,
    ::ethers::contract::EthError,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[etherror(
    name = "NotSupportedAmmProtocolError",
    abi = "NotSupportedAmmProtocolError(uint8)"
    )]
    pub struct NotSupportedAmmProtocolError {
        pub protocol: u8,
    }
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
    #[derive(
    Clone,
    ::ethers::contract::EthError,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[etherror(
    name = "SafeERC20FailedOperation",
    abi = "SafeERC20FailedOperation(address)"
    )]
    pub struct SafeERC20FailedOperation {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SimulatorAbiErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        FailedInnerCall(FailedInnerCall),
        MultiSwapError(MultiSwapError),
        NotSupportedAmmProtocolError(NotSupportedAmmProtocolError),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SimulatorAbiErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) = <AddressInsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressInsufficientBalance(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <MultiSwapError as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MultiSwapError(decoded));
            }
            if let Ok(decoded) = <NotSupportedAmmProtocolError as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotSupportedAmmProtocolError(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SimulatorAbiErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddressInsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedInnerCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiSwapError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotSupportedAmmProtocolError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SimulatorAbiErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <AddressInsufficientBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedInnerCall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MultiSwapError as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotSupportedAmmProtocolError as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SimulatorAbiErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::MultiSwapError(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotSupportedAmmProtocolError(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SimulatorAbiErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for SimulatorAbiErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance> for SimulatorAbiErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for SimulatorAbiErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<MultiSwapError> for SimulatorAbiErrors {
        fn from(value: MultiSwapError) -> Self {
            Self::MultiSwapError(value)
        }
    }
    impl ::core::convert::From<NotSupportedAmmProtocolError> for SimulatorAbiErrors {
        fn from(value: NotSupportedAmmProtocolError) -> Self {
            Self::NotSupportedAmmProtocolError(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation> for SimulatorAbiErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    ///Container type for all input parameters for the `simulateGetAmountsOutUniswapV2` function with signature `simulateGetAmountsOutUniswapV2(address,address[],uint256)` and selector `0x9531d189`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(
    name = "simulateGetAmountsOutUniswapV2",
    abi = "simulateGetAmountsOutUniswapV2(address,address[],uint256)"
    )]
    pub struct SimulateGetAmountsOutUniswapV2Call {
        pub router: ::ethers::core::types::Address,
        pub path: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `simulateGetAmountsOutUniswapV3` function with signature `simulateGetAmountsOutUniswapV3(address,address,bytes,uint256)` and selector `0x2657015a`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(
    name = "simulateGetAmountsOutUniswapV3",
    abi = "simulateGetAmountsOutUniswapV3(address,address,bytes,uint256)"
    )]
    pub struct SimulateGetAmountsOutUniswapV3Call {
        pub router: ::ethers::core::types::Address,
        pub output_token: ::ethers::core::types::Address,
        pub path: ::ethers::core::types::Bytes,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `simulateMultiSwap` function with signature `simulateMultiSwap((uint8,address,address,bytes,uint256,uint256,uint256)[],bool)` and selector `0x4a0b44a2`
    #[derive(
    Clone,
    ::ethers::contract::EthCall,
    ::ethers::contract::EthDisplay,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    #[ethcall(
        name = "simulateMultiSwap",
        abi = "simulateMultiSwap((uint8,address,address,bytes,uint256,uint256,uint256)[],bool)"
    )]
    pub struct SimulateMultiSwapCall {
        pub swaps: ::std::vec::Vec<OneSwapInfo>,
        pub chain_swaps: bool,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SimulatorAbiCalls {
        SimulateGetAmountsOutUniswapV2(SimulateGetAmountsOutUniswapV2Call),
        SimulateGetAmountsOutUniswapV3(SimulateGetAmountsOutUniswapV3Call),
        SimulateMultiSwap(SimulateMultiSwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for SimulatorAbiCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <SimulateGetAmountsOutUniswapV2Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateGetAmountsOutUniswapV2(decoded));
            }
            if let Ok(decoded) = <SimulateGetAmountsOutUniswapV3Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateGetAmountsOutUniswapV3(decoded));
            }
            if let Ok(decoded) = <SimulateMultiSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateMultiSwap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SimulatorAbiCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SimulateGetAmountsOutUniswapV2(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateGetAmountsOutUniswapV3(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateMultiSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SimulatorAbiCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SimulateGetAmountsOutUniswapV2(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SimulateGetAmountsOutUniswapV3(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SimulateMultiSwap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SimulateGetAmountsOutUniswapV2Call>
    for SimulatorAbiCalls {
        fn from(value: SimulateGetAmountsOutUniswapV2Call) -> Self {
            Self::SimulateGetAmountsOutUniswapV2(value)
        }
    }
    impl ::core::convert::From<SimulateGetAmountsOutUniswapV3Call>
    for SimulatorAbiCalls {
        fn from(value: SimulateGetAmountsOutUniswapV3Call) -> Self {
            Self::SimulateGetAmountsOutUniswapV3(value)
        }
    }
    impl ::core::convert::From<SimulateMultiSwapCall> for SimulatorAbiCalls {
        fn from(value: SimulateMultiSwapCall) -> Self {
            Self::SimulateMultiSwap(value)
        }
    }
    ///Container type for all return fields from the `simulateGetAmountsOutUniswapV2` function with signature `simulateGetAmountsOutUniswapV2(address,address[],uint256)` and selector `0x9531d189`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct SimulateGetAmountsOutUniswapV2Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `simulateGetAmountsOutUniswapV3` function with signature `simulateGetAmountsOutUniswapV3(address,address,bytes,uint256)` and selector `0x2657015a`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct SimulateGetAmountsOutUniswapV3Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `simulateMultiSwap` function with signature `simulateMultiSwap((uint8,address,address,bytes,uint256,uint256,uint256)[],bool)` and selector `0x4a0b44a2`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct SimulateMultiSwapReturn(pub ::ethers::core::types::U256);
    ///`OneSwapInfo(uint8,address,address,bytes,uint256,uint256,uint256)`
    #[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
    )]
    pub struct OneSwapInfo {
        pub protocol: u8,
        pub router: ::ethers::core::types::Address,
        pub token_in: ::ethers::core::types::Address,
        pub path: ::ethers::core::types::Bytes,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out_min: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
    }
}
