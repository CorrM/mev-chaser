pub use simulator_abi::*;

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
pub mod simulator_abi {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("simulateGetAmountsOut"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("simulateGetAmountsOut",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("protocol"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "enum AmmProtocol"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "address"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("path"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bytes"),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("simulateMultiSwap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("simulateMultiSwap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swaps"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                        ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                        ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ],),
                                ),),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "struct OneSwapInfo[]"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("chainSwaps"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("bool"),),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("uint256"),),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("target"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AddressInsufficientBalance",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("account"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FailedInnerCall"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MultiSwapError"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MultiSwapError"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                    "uint256"
                                ),),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("errorReason"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("string"),),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotSupportedAmmProtocolError"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotSupportedAmmProtocolError",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("protocol"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                                "enum AmmProtocol"
                            ),),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SafeERC20FailedOperation",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned("address"),),
                        },],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SIMULATORABI_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x12\xF5\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cJ\x0BD\xA2\x14a\0;W\x80c\xB4\xAC\x9AK\x14a\0`W[`\0\x80\xFD[a\0Na\0I6`\x04a\x0C\x1BV[a\0sV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0Na\0n6`\x04a\rtV[a\0\xB5V[`\0\x80a\0\x82\x84\x84`\0a\0\xCDV[\x90P\x80`\x01\x82Qa\0\x93\x91\x90a\r\xF2V[\x81Q\x81\x10a\0\xA3Wa\0\xA3a\x0E\x05V[` \x02` \x01\x01Q\x91PP[\x92\x91PPV[`2a\0\xC5V[`@Q\x80\x91\x03\x90\xFD[\x94\x93PPPPV[``\x82\x80\x15a\0\xDDWP`\x02\x84Q\x10[\x15a\x016W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FChainSwap requires at least 2 sw`D\x82\x01Rbaps`\xE8\x1B`d\x82\x01R`\x84\x01a\0\xBCV[`\0``\x83a\x01FW`\x01a\x01IV[\x85Q[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01aWa\x01aa\n\xCAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\x8AW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P`\0\x80`\0[\x88Q\x81\x10\x15a\x04YW`\0\x89\x82\x81Q\x81\x10a\x01\xB0Wa\x01\xB0a\x0E\x05V[` \x02` \x01\x01Q\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x02\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x14\x9B\xDD]\x19\\\x88\x1A\\\xC8\x1B\x9D[\x1B`\x92\x1B`D\x82\x01R`d\x01a\0\xBCV[`\0\x81``\x01QQ\x11a\x02UW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x14\x18]\x1A\x08\x1A\\\xC8\x1B\x9D[\x1B`\xA2\x1B`D\x82\x01R`d\x01a\0\xBCV[`@\x81\x01Q\x89\x15a\x03\x0CW\x82`\0\x03a\x02xW\x81`\x80\x01Q\x94P`\0\x93Pa\x03\x1BV[`\x01\x8BQa\x02\x86\x91\x90a\r\xF2V[\x83\x03a\x02\x99W`\xA0\x82\x01Q\x93\x94Pa\x03\x1BV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x01\x91\x90a\x0E\xFEV[\x94P`\0\x93Pa\x03\x1BV[\x81`\x80\x01Q\x94P\x81`\xA0\x01Q\x93P[` \x82\x01Qa\x035\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x87a\x04\x8CV[\x81`\xC0\x01Q`\0\x03a\x03RWa\x03LB`<a\x0F\x17V[`\xC0\x83\x01R[`\0\x82Q`\x01\x81\x11\x15a\x03gWa\x03ga\x0E\x1BV[\x03a\x03\xABW`\0\x82``\x01Q\x80` \x01\x90Q\x81\x01\x90a\x03\x86\x91\x90a\x0E1V[\x90Pa\x03\x9D\x83` \x01Q\x82\x88\x88\x87`\xC0\x01Qa\x05\x1CV[\x90\x99P\x97P\x94Pa\x04\x06\x90PV[`\x01\x82Q`\x01\x81\x11\x15a\x03\xC0Wa\x03\xC0a\x0E\x1BV[\x03a\x03\xEAWa\x03\xDE\x82` \x01Q\x83``\x01Q\x87\x87\x86`\xC0\x01Qa\x06\x94V[\x90\x98P\x96P\x93Pa\x04\x06V[\x81Q`@Qc\x95\x9E\xD9\xB9`\xE0\x1B\x81Ra\0\xBC\x91\x90`\x04\x01a\x0E\xD6V[\x86\x15a\x04)W\x82\x86`@Qc\x8B3le`\xE0\x1B\x81R`\x04\x01a\0\xBC\x92\x91\x90a\x0FzV[\x88\x15a\x04OW\x83\x88\x84\x81Q\x81\x10a\x04BWa\x04Ba\x0E\x05V[` \x02` \x01\x01\x81\x81RPP[PP`\x01\x01a\x01\x93V[P\x85a\x04\x80W\x80\x85`\0\x81Q\x81\x10a\x04sWa\x04sa\x0E\x05V[` \x02` \x01\x01\x81\x81RPP[PPPP[\x93\x92PPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`\0\x91\x90\x85\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\0\x91\x90a\x0E\xFEV[\x90Pa\x05\x16\x84\x84a\x05\x11\x85\x85a\x0F\x17V[a\x07\xFCV[PPPPV[`@Qc8\xED\x179`\xE0\x1B\x81R`\0\x90\x81\x90``\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c8\xED\x179\x90a\x05Z\x90\x8A\x90\x8A\x90\x8D\x900\x90\x8C\x90`\x04\x01a\x11LV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x05\x9AWP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05\x97\x91\x90\x81\x01\x90a\x0F\xD8V[`\x01[a\x06GWa\x05\xA6a\x11\x88V[\x80c\x08\xC3y\xA0\x03a\x05\xD6WPa\x05\xBAa\x11\xA4V[\x80a\x05\xC5WPa\x05\xD8V[`\0\x94P`\x01\x93P\x91Pa\x06\x89\x90PV[P[=\x80\x80\x15a\x06\x02W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\x07V[``\x91P[P`\0`\x01`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x13[\xDC\xDD\x1B\x1EH\x1C\x18Z\\\x88\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`Z\x1B\x81RP\x94P\x94P\x94PPPa\x06\x89V[\x80`\x01\x82Qa\x06V\x91\x90a\r\xF2V[\x81Q\x81\x10a\x06fWa\x06fa\x0E\x05V[` \x02` \x01\x01Q`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x94P\x94P\x94PPP[\x95P\x95P\x95\x92PPPV[`@\x80Q`\xA0\x81\x01\x82R\x85\x81R0` \x82\x01R\x80\x82\x01\x83\x90R``\x81\x81\x01\x86\x90R`\x80\x82\x01\x85\x90R\x91Qc\xC0K\x8DY`\xE0\x1B\x81R`\0\x92\x83\x92\x90\x91\x89\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xC0K\x8DY\x90a\x06\xF1\x90\x84\x90`\x04\x01a\x12.V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x07,WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x07)\x91\x81\x01\x90a\x0E\xFEV[`\x01[a\x07\xDBWa\x078a\x11\x88V[\x80c\x08\xC3y\xA0\x03a\x07iWPa\x07La\x11\xA4V[\x80a\x07WWPa\x07kV[`\0\x95P`\x01\x94P\x92Pa\x06\x89\x91PPV[P[=\x80\x80\x15a\x07\x95W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x07\x9AV[``\x91P[P`\0`\x01`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x13[\xDC\xDD\x1B\x1EH\x1C\x18Z\\\x88\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`Z\x1B\x81RP\x95P\x95P\x95PPPPa\x06\x89V[\x80`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x95P\x95P\x95PPPPa\x06\x89V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x08M\x84\x82a\x08\xB0V[a\x05\x16W`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16`$\x82\x01R`\0`D\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x08\xA6\x90\x85\x90a\tXV[a\x05\x16\x84\x82a\tXV[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x08\xCD\x91\x90a\x12\x86V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\t\nW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\x0FV[``\x91P[P\x91P\x91P\x81\x80\x15a\t9WP\x80Q\x15\x80a\t9WP\x80\x80` \x01\x90Q\x81\x01\x90a\t9\x91\x90a\x12\xA2V[\x80\x15a\tOWP`\0\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x95\x94PPPPPV[`\0a\tm`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\t\xC0V[\x90P\x80Q`\0\x14\x15\x80\x15a\t\x92WP\x80\x80` \x01\x90Q\x81\x01\x90a\t\x90\x91\x90a\x12\xA2V[\x15[\x15a\t\xBBW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\0\xBCV[PPPV[``a\x04\x85\x83\x83`\0\x84`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\t\xE6\x91\x90a\x12\x86V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\n#W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n(V[``\x91P[P\x91P\x91Pa\n8\x86\x83\x83a\nBV[\x96\x95PPPPPPV[``\x82a\nWWa\nR\x82a\n\x9EV[a\x04\x85V[\x81Q\x15\x80\x15a\nnWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\n\x97W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\0\xBCV[P\x80a\x04\x85V[\x80Q\x15a\n\xAEW\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x06Wa\x0B\x06a\n\xCAV[`@RPPV[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B0Wa\x0B0a\n\xCAV[`@R\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0BPWa\x0BPa\n\xCAV[P`\x05\x1B` \x01\x90V[\x805`\x02\x81\x10a\x0BiW`\0\x80\xFD[\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xC7W`\0\x80\xFD[\x805a\x0Bi\x81a\x0BnV[`\0\x82`\x1F\x83\x01\x12a\x0B\x9FW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xB9Wa\x0B\xB9a\n\xCAV[`@Qa\x0B\xD0`\x1F\x83\x01`\x1F\x19\x16` \x01\x82a\n\xE0V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0B\xE5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x80\x15\x15\x81\x14a\n\xC7W`\0\x80\xFD[\x805a\x0Bi\x81a\x0C\x02V[`\0\x80`@\x83\x85\x03\x12\x15a\x0C.W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0CFW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0CZW`\0\x80\xFD[\x815` a\x0Cg\x82a\x0B6V[`@Qa\x0Ct\x82\x82a\n\xE0V[\x83\x81R`\x05\x93\x90\x93\x1B\x85\x01\x82\x01\x92\x82\x81\x01\x91P\x89\x84\x11\x15a\x0C\x94W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\rWW\x805\x86\x81\x11\x15a\x0C\xAFW`\0\x80\xFD[\x87\x01`\xE0\x81\x8D\x03`\x1F\x19\x01\x12\x15a\x0C\xC5W`\0\x80\xFD[a\x0C\xCDa\x0B\rV[a\x0C\xD8\x86\x83\x01a\x0BZV[\x81Ra\x0C\xE6`@\x83\x01a\x0B\x83V[\x86\x82\x01Ra\x0C\xF6``\x83\x01a\x0B\x83V[`@\x82\x01R`\x80\x80\x83\x015\x89\x81\x11\x15a\r\x0FW`\0\x80\x81\xFD[a\r\x1D\x8F\x89\x83\x87\x01\x01a\x0B\x8EV[``\x84\x01RP`\xA0\x83\x81\x015\x91\x83\x01\x91\x90\x91R`\xC0\x80\x84\x015\x91\x83\x01\x91\x90\x91R`\xE0\x90\x92\x015\x91\x81\x01\x91\x90\x91R\x83R\x91\x83\x01\x91\x83\x01a\x0C\x98V[P\x96Pa\rg\x90P\x87\x82\x01a\x0C\x10V[\x94PPPPP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\r\x8AW`\0\x80\xFD[a\r\x93\x85a\x0BZV[\x93P` \x85\x015a\r\xA3\x81a\x0BnV[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xBFW`\0\x80\xFD[a\r\xCB\x87\x82\x88\x01a\x0B\x8EV[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\0\xAFWa\0\xAFa\r\xDCV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x0EDW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E[W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x0ElW`\0\x80\xFD[\x80Qa\x0Ew\x81a\x0B6V[`@Qa\x0E\x84\x82\x82a\n\xE0V[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\x0E\xA4W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x0E\xCBW\x83Qa\x0E\xBC\x81a\x0BnV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x0E\xA9V[\x97\x96PPPPPPPV[` \x81\x01`\x02\x83\x10a\x0E\xF8WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15a\x0F\x10W`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\0\xAFWa\0\xAFa\r\xDCV[`\0[\x83\x81\x10\x15a\x0FEW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0F-V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0Ff\x81` \x86\x01` \x86\x01a\x0F*V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x82\x81R`@` \x82\x01R`\0a\0\xC5`@\x83\x01\x84a\x0FNV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x0F\xCDW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x0F\xA8V[P\x94\x95\x94PPPPPV[`\0` \x80\x83\x85\x03\x12\x15a\x0F\xEBW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x02W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x10\x13W`\0\x80\xFD[\x80Qa\x10\x1E\x81a\x0B6V[`@Qa\x10+\x82\x82a\n\xE0V[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\x10KW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x0E\xCBW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x10PV[`\0\x82`\x1F\x83\x01\x12a\x10zW`\0\x80\xFD[\x81Q` a\x10\x87\x82a\x0B6V[`@Qa\x10\x94\x82\x82a\n\xE0V[\x80\x91P\x83\x81R` \x81\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x10\xB8W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x10\xE7W\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10\xDAW`\0\x80\x81\xFD[\x83R\x91\x83\x01\x91\x83\x01a\x10\xBDV[P\x96\x95PPPPPPV[\x82\x85\x10\x15a\x11\x14W\x84Qa\x11\x05\x81a\x0BnV[\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x10\xF2V[`@\x8B\x01Q\x90\x98P\x94PPP\x80\x83\x11\x15a\x11-W`\0\x80\xFD[PPa\x11;\x87\x82\x88\x01a\x10iV[``\x96\x90\x96\x01Q\x94\x97\x93\x96PPPPV[\x85\x81R\x84` \x82\x01R`\xA0`@\x82\x01R`\0a\x11k`\xA0\x83\x01\x86a\x0F\x93V[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x83\x01RP`\x80\x01R\x93\x92PPPV[`\0`\x03=\x11\x15a\x11\xA1W`\x04`\0\x80>P`\0Q`\xE0\x1C[\x90V[`\0`D=\x10\x15a\x11\xB2W\x90V[`@Q`\x03\x19=\x81\x01`\x04\x83>\x81Q=g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`$\x84\x01\x11\x81\x84\x11\x17\x15a\x11\xE2WPPPPP\x90V[\x82\x85\x01\x91P\x81Q\x81\x81\x11\x15a\x11\xFAWPPPPPP\x90V[\x84=\x87\x01\x01` \x82\x85\x01\x01\x11\x15a\x12\x14WPPPPPP\x90V[a\x12#` \x82\x86\x01\x01\x87a\n\xE0V[P\x90\x95\x94PPPPPV[` \x81R`\0\x82Q`\xA0` \x84\x01Ra\x12J`\xC0\x84\x01\x82a\x0FNV[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`@\x84\x01Q``\x84\x01R``\x84\x01Q`\x80\x84\x01R`\x80\x84\x01Q`\xA0\x84\x01R\x80\x91PP\x92\x91PPV[`\0\x82Qa\x12\x98\x81\x84` \x87\x01a\x0F*V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12\xB4W`\0\x80\xFD[\x81Qa\x04\x85\x81a\x0C\x02V\xFE\xA2dipfsX\"\x12 \x01\xB3\xAA%\xF7&\x9D\x11\x95\xD7\xE2o\x9A*v\x11\xA9\x0C\xA5\x04t\xC9\xB7\\\xE9'\x11\xDD\xBA\xE9\xBF\x95dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static SIMULATORABI_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cJ\x0BD\xA2\x14a\0;W\x80c\xB4\xAC\x9AK\x14a\0`W[`\0\x80\xFD[a\0Na\0I6`\x04a\x0C\x1BV[a\0sV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0Na\0n6`\x04a\rtV[a\0\xB5V[`\0\x80a\0\x82\x84\x84`\0a\0\xCDV[\x90P\x80`\x01\x82Qa\0\x93\x91\x90a\r\xF2V[\x81Q\x81\x10a\0\xA3Wa\0\xA3a\x0E\x05V[` \x02` \x01\x01Q\x91PP[\x92\x91PPV[`2a\0\xC5V[`@Q\x80\x91\x03\x90\xFD[\x94\x93PPPPV[``\x82\x80\x15a\0\xDDWP`\x02\x84Q\x10[\x15a\x016W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FChainSwap requires at least 2 sw`D\x82\x01Rbaps`\xE8\x1B`d\x82\x01R`\x84\x01a\0\xBCV[`\0``\x83a\x01FW`\x01a\x01IV[\x85Q[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01aWa\x01aa\n\xCAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\x8AW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P`\0\x80`\0[\x88Q\x81\x10\x15a\x04YW`\0\x89\x82\x81Q\x81\x10a\x01\xB0Wa\x01\xB0a\x0E\x05V[` \x02` \x01\x01Q\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x02\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x14\x9B\xDD]\x19\\\x88\x1A\\\xC8\x1B\x9D[\x1B`\x92\x1B`D\x82\x01R`d\x01a\0\xBCV[`\0\x81``\x01QQ\x11a\x02UW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x14\x18]\x1A\x08\x1A\\\xC8\x1B\x9D[\x1B`\xA2\x1B`D\x82\x01R`d\x01a\0\xBCV[`@\x81\x01Q\x89\x15a\x03\x0CW\x82`\0\x03a\x02xW\x81`\x80\x01Q\x94P`\0\x93Pa\x03\x1BV[`\x01\x8BQa\x02\x86\x91\x90a\r\xF2V[\x83\x03a\x02\x99W`\xA0\x82\x01Q\x93\x94Pa\x03\x1BV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x01\x91\x90a\x0E\xFEV[\x94P`\0\x93Pa\x03\x1BV[\x81`\x80\x01Q\x94P\x81`\xA0\x01Q\x93P[` \x82\x01Qa\x035\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x87a\x04\x8CV[\x81`\xC0\x01Q`\0\x03a\x03RWa\x03LB`<a\x0F\x17V[`\xC0\x83\x01R[`\0\x82Q`\x01\x81\x11\x15a\x03gWa\x03ga\x0E\x1BV[\x03a\x03\xABW`\0\x82``\x01Q\x80` \x01\x90Q\x81\x01\x90a\x03\x86\x91\x90a\x0E1V[\x90Pa\x03\x9D\x83` \x01Q\x82\x88\x88\x87`\xC0\x01Qa\x05\x1CV[\x90\x99P\x97P\x94Pa\x04\x06\x90PV[`\x01\x82Q`\x01\x81\x11\x15a\x03\xC0Wa\x03\xC0a\x0E\x1BV[\x03a\x03\xEAWa\x03\xDE\x82` \x01Q\x83``\x01Q\x87\x87\x86`\xC0\x01Qa\x06\x94V[\x90\x98P\x96P\x93Pa\x04\x06V[\x81Q`@Qc\x95\x9E\xD9\xB9`\xE0\x1B\x81Ra\0\xBC\x91\x90`\x04\x01a\x0E\xD6V[\x86\x15a\x04)W\x82\x86`@Qc\x8B3le`\xE0\x1B\x81R`\x04\x01a\0\xBC\x92\x91\x90a\x0FzV[\x88\x15a\x04OW\x83\x88\x84\x81Q\x81\x10a\x04BWa\x04Ba\x0E\x05V[` \x02` \x01\x01\x81\x81RPP[PP`\x01\x01a\x01\x93V[P\x85a\x04\x80W\x80\x85`\0\x81Q\x81\x10a\x04sWa\x04sa\x0E\x05V[` \x02` \x01\x01\x81\x81RPP[PPPP[\x93\x92PPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R`\0\x91\x90\x85\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xDCW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\0\x91\x90a\x0E\xFEV[\x90Pa\x05\x16\x84\x84a\x05\x11\x85\x85a\x0F\x17V[a\x07\xFCV[PPPPV[`@Qc8\xED\x179`\xE0\x1B\x81R`\0\x90\x81\x90``\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c8\xED\x179\x90a\x05Z\x90\x8A\x90\x8A\x90\x8D\x900\x90\x8C\x90`\x04\x01a\x11LV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x05\x9AWP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05\x97\x91\x90\x81\x01\x90a\x0F\xD8V[`\x01[a\x06GWa\x05\xA6a\x11\x88V[\x80c\x08\xC3y\xA0\x03a\x05\xD6WPa\x05\xBAa\x11\xA4V[\x80a\x05\xC5WPa\x05\xD8V[`\0\x94P`\x01\x93P\x91Pa\x06\x89\x90PV[P[=\x80\x80\x15a\x06\x02W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x06\x07V[``\x91P[P`\0`\x01`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x13[\xDC\xDD\x1B\x1EH\x1C\x18Z\\\x88\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`Z\x1B\x81RP\x94P\x94P\x94PPPa\x06\x89V[\x80`\x01\x82Qa\x06V\x91\x90a\r\xF2V[\x81Q\x81\x10a\x06fWa\x06fa\x0E\x05V[` \x02` \x01\x01Q`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x94P\x94P\x94PPP[\x95P\x95P\x95\x92PPPV[`@\x80Q`\xA0\x81\x01\x82R\x85\x81R0` \x82\x01R\x80\x82\x01\x83\x90R``\x81\x81\x01\x86\x90R`\x80\x82\x01\x85\x90R\x91Qc\xC0K\x8DY`\xE0\x1B\x81R`\0\x92\x83\x92\x90\x91\x89\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xC0K\x8DY\x90a\x06\xF1\x90\x84\x90`\x04\x01a\x12.V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x92PPP\x80\x15a\x07,WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x07)\x91\x81\x01\x90a\x0E\xFEV[`\x01[a\x07\xDBWa\x078a\x11\x88V[\x80c\x08\xC3y\xA0\x03a\x07iWPa\x07La\x11\xA4V[\x80a\x07WWPa\x07kV[`\0\x95P`\x01\x94P\x92Pa\x06\x89\x91PPV[P[=\x80\x80\x15a\x07\x95W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x07\x9AV[``\x91P[P`\0`\x01`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x13[\xDC\xDD\x1B\x1EH\x1C\x18Z\\\x88\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`Z\x1B\x81RP\x95P\x95P\x95PPPPa\x06\x89V[\x80`\0`@Q\x80` \x01`@R\x80`\0\x81RP\x95P\x95P\x95PPPPa\x06\x89V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x08M\x84\x82a\x08\xB0V[a\x05\x16W`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16`$\x82\x01R`\0`D\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x08\xA6\x90\x85\x90a\tXV[a\x05\x16\x84\x82a\tXV[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x08\xCD\x91\x90a\x12\x86V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\t\nW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\t\x0FV[``\x91P[P\x91P\x91P\x81\x80\x15a\t9WP\x80Q\x15\x80a\t9WP\x80\x80` \x01\x90Q\x81\x01\x90a\t9\x91\x90a\x12\xA2V[\x80\x15a\tOWP`\0\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x95\x94PPPPPV[`\0a\tm`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\t\xC0V[\x90P\x80Q`\0\x14\x15\x80\x15a\t\x92WP\x80\x80` \x01\x90Q\x81\x01\x90a\t\x90\x91\x90a\x12\xA2V[\x15[\x15a\t\xBBW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\0\xBCV[PPPV[``a\x04\x85\x83\x83`\0\x84`\0\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\t\xE6\x91\x90a\x12\x86V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\n#W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\n(V[``\x91P[P\x91P\x91Pa\n8\x86\x83\x83a\nBV[\x96\x95PPPPPPV[``\x82a\nWWa\nR\x82a\n\x9EV[a\x04\x85V[\x81Q\x15\x80\x15a\nnWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\n\x97W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\0\xBCV[P\x80a\x04\x85V[\x80Q\x15a\n\xAEW\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\x06Wa\x0B\x06a\n\xCAV[`@RPPV[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B0Wa\x0B0a\n\xCAV[`@R\x90V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0BPWa\x0BPa\n\xCAV[P`\x05\x1B` \x01\x90V[\x805`\x02\x81\x10a\x0BiW`\0\x80\xFD[\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xC7W`\0\x80\xFD[\x805a\x0Bi\x81a\x0BnV[`\0\x82`\x1F\x83\x01\x12a\x0B\x9FW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xB9Wa\x0B\xB9a\n\xCAV[`@Qa\x0B\xD0`\x1F\x83\x01`\x1F\x19\x16` \x01\x82a\n\xE0V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0B\xE5W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x80\x15\x15\x81\x14a\n\xC7W`\0\x80\xFD[\x805a\x0Bi\x81a\x0C\x02V[`\0\x80`@\x83\x85\x03\x12\x15a\x0C.W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0CFW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0CZW`\0\x80\xFD[\x815` a\x0Cg\x82a\x0B6V[`@Qa\x0Ct\x82\x82a\n\xE0V[\x83\x81R`\x05\x93\x90\x93\x1B\x85\x01\x82\x01\x92\x82\x81\x01\x91P\x89\x84\x11\x15a\x0C\x94W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\rWW\x805\x86\x81\x11\x15a\x0C\xAFW`\0\x80\xFD[\x87\x01`\xE0\x81\x8D\x03`\x1F\x19\x01\x12\x15a\x0C\xC5W`\0\x80\xFD[a\x0C\xCDa\x0B\rV[a\x0C\xD8\x86\x83\x01a\x0BZV[\x81Ra\x0C\xE6`@\x83\x01a\x0B\x83V[\x86\x82\x01Ra\x0C\xF6``\x83\x01a\x0B\x83V[`@\x82\x01R`\x80\x80\x83\x015\x89\x81\x11\x15a\r\x0FW`\0\x80\x81\xFD[a\r\x1D\x8F\x89\x83\x87\x01\x01a\x0B\x8EV[``\x84\x01RP`\xA0\x83\x81\x015\x91\x83\x01\x91\x90\x91R`\xC0\x80\x84\x015\x91\x83\x01\x91\x90\x91R`\xE0\x90\x92\x015\x91\x81\x01\x91\x90\x91R\x83R\x91\x83\x01\x91\x83\x01a\x0C\x98V[P\x96Pa\rg\x90P\x87\x82\x01a\x0C\x10V[\x94PPPPP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\r\x8AW`\0\x80\xFD[a\r\x93\x85a\x0BZV[\x93P` \x85\x015a\r\xA3\x81a\x0BnV[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\xBFW`\0\x80\xFD[a\r\xCB\x87\x82\x88\x01a\x0B\x8EV[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\0\xAFWa\0\xAFa\r\xDCV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0` \x80\x83\x85\x03\x12\x15a\x0EDW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E[W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x0ElW`\0\x80\xFD[\x80Qa\x0Ew\x81a\x0B6V[`@Qa\x0E\x84\x82\x82a\n\xE0V[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\x0E\xA4W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x0E\xCBW\x83Qa\x0E\xBC\x81a\x0BnV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x0E\xA9V[\x97\x96PPPPPPPV[` \x81\x01`\x02\x83\x10a\x0E\xF8WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15a\x0F\x10W`\0\x80\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\0\xAFWa\0\xAFa\r\xDCV[`\0[\x83\x81\x10\x15a\x0FEW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0F-V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0Ff\x81` \x86\x01` \x86\x01a\x0F*V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x82\x81R`@` \x82\x01R`\0a\0\xC5`@\x83\x01\x84a\x0FNV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x0F\xCDW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x0F\xA8V[P\x94\x95\x94PPPPPV[`\0` \x80\x83\x85\x03\x12\x15a\x0F\xEBW`\0\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x10\x02W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x10\x13W`\0\x80\xFD[\x80Qa\x10\x1E\x81a\x0B6V[`@Qa\x10+\x82\x82a\n\xE0V[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\x10KW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x0E\xCBW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x10PV[`\0\x82`\x1F\x83\x01\x12a\x10zW`\0\x80\xFD[\x81Q` a\x10\x87\x82a\x0B6V[`@Qa\x10\x94\x82\x82a\n\xE0V[\x80\x91P\x83\x81R` \x81\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x10\xB8W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x10\xE7W\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10\xDAW`\0\x80\x81\xFD[\x83R\x91\x83\x01\x91\x83\x01a\x10\xBDV[P\x96\x95PPPPPPV[\x82\x85\x10\x15a\x11\x14W\x84Qa\x11\x05\x81a\x0BnV[\x82R\x93\x85\x01\x93\x90\x85\x01\x90a\x10\xF2V[`@\x8B\x01Q\x90\x98P\x94PPP\x80\x83\x11\x15a\x11-W`\0\x80\xFD[PPa\x11;\x87\x82\x88\x01a\x10iV[``\x96\x90\x96\x01Q\x94\x97\x93\x96PPPPV[\x85\x81R\x84` \x82\x01R`\xA0`@\x82\x01R`\0a\x11k`\xA0\x83\x01\x86a\x0F\x93V[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x83\x01RP`\x80\x01R\x93\x92PPPV[`\0`\x03=\x11\x15a\x11\xA1W`\x04`\0\x80>P`\0Q`\xE0\x1C[\x90V[`\0`D=\x10\x15a\x11\xB2W\x90V[`@Q`\x03\x19=\x81\x01`\x04\x83>\x81Q=g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`$\x84\x01\x11\x81\x84\x11\x17\x15a\x11\xE2WPPPPP\x90V[\x82\x85\x01\x91P\x81Q\x81\x81\x11\x15a\x11\xFAWPPPPPP\x90V[\x84=\x87\x01\x01` \x82\x85\x01\x01\x11\x15a\x12\x14WPPPPPP\x90V[a\x12#` \x82\x86\x01\x01\x87a\n\xE0V[P\x90\x95\x94PPPPPV[` \x81R`\0\x82Q`\xA0` \x84\x01Ra\x12J`\xC0\x84\x01\x82a\x0FNV[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`@\x84\x01Q``\x84\x01R``\x84\x01Q`\x80\x84\x01R`\x80\x84\x01Q`\xA0\x84\x01R\x80\x91PP\x92\x91PPV[`\0\x82Qa\x12\x98\x81\x84` \x87\x01a\x0F*V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x12\xB4W`\0\x80\xFD[\x81Qa\x04\x85\x81a\x0C\x02V\xFE\xA2dipfsX\"\x12 \x01\xB3\xAA%\xF7&\x9D\x11\x95\xD7\xE2o\x9A*v\x11\xA9\x0C\xA5\x04t\xC9\xB7\\\xE9'\x11\xDD\xBA\xE9\xBF\x95dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static SIMULATORABI_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
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
        pub fn new<T: Into<::ethers::core::types::Address>>(address: T, client: ::std::sync::Arc<M>) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SIMULATORABI_ABI.clone(),
                client,
            ))
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
        ///Calls the contract's `simulateGetAmountsOut` (0xb4ac9a4b) function
        pub fn simulate_get_amounts_out(
            &self,
            protocol: u8,
            contract_address: ::ethers::core::types::Address,
            path: ::ethers::core::types::Bytes,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([180, 172, 154, 75], (protocol, contract_address, path, amount_in))
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
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for SimulatorAbi<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
    #[derive(
        Clone, ::ethers::contract::EthError, ::ethers::contract::EthDisplay, Default, Debug, PartialEq, Eq, Hash,
    )]
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `AddressInsufficientBalance` with signature `AddressInsufficientBalance(address)` and selector `0xcd786059`
    #[derive(
        Clone, ::ethers::contract::EthError, ::ethers::contract::EthDisplay, Default, Debug, PartialEq, Eq, Hash,
    )]
    #[etherror(name = "AddressInsufficientBalance", abi = "AddressInsufficientBalance(address)")]
    pub struct AddressInsufficientBalance {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `FailedInnerCall` with signature `FailedInnerCall()` and selector `0x1425ea42`
    #[derive(
        Clone, ::ethers::contract::EthError, ::ethers::contract::EthDisplay, Default, Debug, PartialEq, Eq, Hash,
    )]
    #[etherror(name = "FailedInnerCall", abi = "FailedInnerCall()")]
    pub struct FailedInnerCall;
    ///Custom Error type `MultiSwapError` with signature `MultiSwapError(uint256,string)` and selector `0x8b336c65`
    #[derive(
        Clone, ::ethers::contract::EthError, ::ethers::contract::EthDisplay, Default, Debug, PartialEq, Eq, Hash,
    )]
    #[etherror(name = "MultiSwapError", abi = "MultiSwapError(uint256,string)")]
    pub struct MultiSwapError {
        pub swap_index: ::ethers::core::types::U256,
        pub error_reason: ::std::string::String,
    }
    ///Custom Error type `NotSupportedAmmProtocolError` with signature `NotSupportedAmmProtocolError(uint8)` and selector `0x959ed9b9`
    #[derive(
        Clone, ::ethers::contract::EthError, ::ethers::contract::EthDisplay, Default, Debug, PartialEq, Eq, Hash,
    )]
    #[etherror(name = "NotSupportedAmmProtocolError", abi = "NotSupportedAmmProtocolError(uint8)")]
    pub struct NotSupportedAmmProtocolError {
        pub protocol: u8,
    }
    ///Custom Error type `SafeERC20FailedOperation` with signature `SafeERC20FailedOperation(address)` and selector `0x5274afe7`
    #[derive(
        Clone, ::ethers::contract::EthError, ::ethers::contract::EthDisplay, Default, Debug, PartialEq, Eq, Hash,
    )]
    #[etherror(name = "SafeERC20FailedOperation", abi = "SafeERC20FailedOperation(address)")]
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
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) = <AddressInsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddressInsufficientBalance(decoded));
            }
            if let Ok(decoded) = <FailedInnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FailedInnerCall(decoded));
            }
            if let Ok(decoded) = <MultiSwapError as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MultiSwapError(decoded));
            }
            if let Ok(decoded) = <NotSupportedAmmProtocolError as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotSupportedAmmProtocolError(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SimulatorAbiErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddressInsufficientBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FailedInnerCall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MultiSwapError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotSupportedAmmProtocolError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeERC20FailedOperation(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SimulatorAbiErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <AddressInsufficientBalance as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <FailedInnerCall as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <MultiSwapError as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NotSupportedAmmProtocolError as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <SafeERC20FailedOperation as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SimulatorAbiErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::MultiSwapError(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotSupportedAmmProtocolError(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeERC20FailedOperation(element) => ::core::fmt::Display::fmt(element, f),
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
    ///Container type for all input parameters for the `simulateGetAmountsOut` function with signature `simulateGetAmountsOut(uint8,address,bytes,uint256)` and selector `0xb4ac9a4b`
    #[derive(
        Clone, ::ethers::contract::EthCall, ::ethers::contract::EthDisplay, Default, Debug, PartialEq, Eq, Hash,
    )]
    #[ethcall(
        name = "simulateGetAmountsOut",
        abi = "simulateGetAmountsOut(uint8,address,bytes,uint256)"
    )]
    pub struct SimulateGetAmountsOutCall {
        pub protocol: u8,
        pub contract_address: ::ethers::core::types::Address,
        pub path: ::ethers::core::types::Bytes,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `simulateMultiSwap` function with signature `simulateMultiSwap((uint8,address,address,bytes,uint256,uint256,uint256)[],bool)` and selector `0x4a0b44a2`
    #[derive(
        Clone, ::ethers::contract::EthCall, ::ethers::contract::EthDisplay, Default, Debug, PartialEq, Eq, Hash,
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
        SimulateGetAmountsOut(SimulateGetAmountsOutCall),
        SimulateMultiSwap(SimulateMultiSwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for SimulatorAbiCalls {
        fn decode(data: impl AsRef<[u8]>) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <SimulateGetAmountsOutCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SimulateGetAmountsOut(decoded));
            }
            if let Ok(decoded) = <SimulateMultiSwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SimulateMultiSwap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SimulatorAbiCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SimulateGetAmountsOut(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SimulateMultiSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SimulatorAbiCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SimulateGetAmountsOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateMultiSwap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SimulateGetAmountsOutCall> for SimulatorAbiCalls {
        fn from(value: SimulateGetAmountsOutCall) -> Self {
            Self::SimulateGetAmountsOut(value)
        }
    }
    impl ::core::convert::From<SimulateMultiSwapCall> for SimulatorAbiCalls {
        fn from(value: SimulateMultiSwapCall) -> Self {
            Self::SimulateMultiSwap(value)
        }
    }
    ///Container type for all return fields from the `simulateGetAmountsOut` function with signature `simulateGetAmountsOut(uint8,address,bytes,uint256)` and selector `0xb4ac9a4b`
    #[derive(
        Clone, ::ethers::contract::EthAbiType, ::ethers::contract::EthAbiCodec, Default, Debug, PartialEq, Eq, Hash,
    )]
    pub struct SimulateGetAmountsOutReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `simulateMultiSwap` function with signature `simulateMultiSwap((uint8,address,address,bytes,uint256,uint256,uint256)[],bool)` and selector `0x4a0b44a2`
    #[derive(
        Clone, ::ethers::contract::EthAbiType, ::ethers::contract::EthAbiCodec, Default, Debug, PartialEq, Eq, Hash,
    )]
    pub struct SimulateMultiSwapReturn(pub ::ethers::core::types::U256);
    ///`OneSwapInfo(uint8,address,address,bytes,uint256,uint256,uint256)`
    #[derive(
        Clone, ::ethers::contract::EthAbiType, ::ethers::contract::EthAbiCodec, Default, Debug, PartialEq, Eq, Hash,
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
