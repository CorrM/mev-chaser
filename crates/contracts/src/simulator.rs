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
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SIMULATORABI_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[Pa\x11\x08\x80a\0\x1D_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80cJ\x0BD\xA2\x14a\0-W[_\x80\xFD[a\0@a\0;6`\x04a\x0B\xAAV[a\0RV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[_\x80a\0_\x84\x84_a\0\x92V[\x90P\x80`\x01\x82Qa\0p\x91\x90a\r\x0EV[\x81Q\x81\x10a\0\x80Wa\0\x80a\r!V[` \x02` \x01\x01Q\x91PP[\x92\x91PPV[``\x82\x80\x15a\0\xA2WP`\x02\x84Q\x10[\x15a\x01\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FChainSwap requires at least 2 sw`D\x82\x01Rbaps`\xE8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_``\x83a\x01\x0FW`\x01a\x01\x12V[\x85Q[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01*Wa\x01*a\ncV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01SW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P_\x80_[\x88Q\x81\x10\x15a\x04\x15W_\x89\x82\x81Q\x81\x10a\x01vWa\x01va\r!V[` \x02` \x01\x01Q\x90P_`\x01`\x01`\xA0\x1B\x03\x16\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x01\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x14\x9B\xDD]\x19\\\x88\x1A\\\xC8\x1B\x9D[\x1B`\x92\x1B`D\x82\x01R`d\x01a\0\xF7V[_\x81``\x01QQ\x11a\x02\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x14\x18]\x1A\x08\x1A\\\xC8\x1B\x9D[\x1B`\xA2\x1B`D\x82\x01R`d\x01a\0\xF7V[`@\x81\x01Q\x89\x15a\x02\xCBW\x82_\x03a\x02:W\x81`\x80\x01Q\x94P_\x93Pa\x02\xDAV[`\x01\x8BQa\x02H\x91\x90a\r\x0EV[\x83\x03a\x02[W`\xA0\x82\x01Q\x93\x94Pa\x02\xDAV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x9DW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xC1\x91\x90a\r5V[\x94P_\x93Pa\x02\xDAV[\x81`\x80\x01Q\x94P\x81`\xA0\x01Q\x93P[` \x82\x01Qa\x02\xF4\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x87a\x04GV[\x81`\xC0\x01Q_\x03a\x03\x10Wa\x03\nB`<a\rLV[`\xC0\x83\x01R[_\x82Q`\x01\x81\x11\x15a\x03$Wa\x03$a\r_V[\x03a\x03gW_\x82``\x01Q\x80` \x01\x90Q\x81\x01\x90a\x03B\x91\x90a\rsV[\x90Pa\x03Y\x83` \x01Q\x82\x88\x88\x87`\xC0\x01Qa\x04\xD4V[\x90\x99P\x97P\x94Pa\x03\xC2\x90PV[`\x01\x82Q`\x01\x81\x11\x15a\x03|Wa\x03|a\r_V[\x03a\x03\xA6Wa\x03\x9A\x82` \x01Q\x83``\x01Q\x87\x87\x86`\xC0\x01Qa\x06CV[\x90\x98P\x96P\x93Pa\x03\xC2V[\x81Q`@Qc\x95\x9E\xD9\xB9`\xE0\x1B\x81Ra\0\xF7\x91\x90`\x04\x01a\x0E\x13V[\x86\x15a\x03\xE5W\x82\x86`@Qc\x8B3le`\xE0\x1B\x81R`\x04\x01a\0\xF7\x92\x91\x90a\x0E\x86V[\x88\x15a\x04\x0BW\x83\x88\x84\x81Q\x81\x10a\x03\xFEWa\x03\xFEa\r!V[` \x02` \x01\x01\x81\x81RPP[PP`\x01\x01a\x01ZV[P\x85a\x04;W\x80\x85_\x81Q\x81\x10a\x04.Wa\x04.a\r!V[` \x02` \x01\x01\x81\x81RPP[PPPP[\x93\x92PPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R_\x91\x90\x85\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x94W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB8\x91\x90a\r5V[\x90Pa\x04\xCE\x84\x84a\x04\xC9\x85\x85a\rLV[a\x07\xA4V[PPPPV[`@Qc8\xED\x179`\xE0\x1B\x81R_\x90\x81\x90``\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c8\xED\x179\x90a\x05\x11\x90\x8A\x90\x8A\x90\x8D\x900\x90\x8C\x90`\x04\x01a\x0E\xA6V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x05NWP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05K\x91\x90\x81\x01\x90a\x0F\x17V[`\x01[a\x05\xF8Wa\x05Za\x0F\xA3V[\x80c\x08\xC3y\xA0\x03a\x05\x89WPa\x05na\x0F\xBCV[\x80a\x05yWPa\x05\x8BV[_\x94P`\x01\x93P\x91Pa\x068\x90PV[P[=\x80\x80\x15a\x05\xB4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x05\xB9V[``\x91P[P_`\x01`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x13[\xDC\xDD\x1B\x1EH\x1C\x18Z\\\x88\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`Z\x1B\x81RP\x94P\x94P\x94PPPa\x068V[\x80`\x01\x82Qa\x06\x07\x91\x90a\r\x0EV[\x81Q\x81\x10a\x06\x17Wa\x06\x17a\r!V[` \x02` \x01\x01Q_`@Q\x80` \x01`@R\x80_\x81RP\x94P\x94P\x94PPP[\x95P\x95P\x95\x92PPPV[`@\x80Q`\xA0\x81\x01\x82R\x85\x81R0` \x82\x01R\x80\x82\x01\x83\x90R``\x81\x81\x01\x86\x90R`\x80\x82\x01\x85\x90R\x91Qc\xC0K\x8DY`\xE0\x1B\x81R_\x92\x83\x92\x90\x91\x89\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xC0K\x8DY\x90a\x06\x9F\x90\x84\x90`\x04\x01a\x10EV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x06\xD9WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x06\xD6\x91\x81\x01\x90a\r5V[`\x01[a\x07\x85Wa\x06\xE5a\x0F\xA3V[\x80c\x08\xC3y\xA0\x03a\x07\x15WPa\x06\xF9a\x0F\xBCV[\x80a\x07\x04WPa\x07\x17V[_\x95P`\x01\x94P\x92Pa\x068\x91PPV[P[=\x80\x80\x15a\x07@W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07EV[``\x91P[P_`\x01`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x13[\xDC\xDD\x1B\x1EH\x1C\x18Z\\\x88\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`Z\x1B\x81RP\x95P\x95P\x95PPPPa\x068V[\x80_`@Q\x80` \x01`@R\x80_\x81RP\x95P\x95P\x95PPPPa\x068V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x07\xF5\x84\x82a\x08WV[a\x04\xCEW`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16`$\x82\x01R_`D\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x08M\x90\x85\x90a\x08\xF8V[a\x04\xCE\x84\x82a\x08\xF8V[_\x80_\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x08r\x91\x90a\x10\x9CV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x08\xABW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x08\xB0V[``\x91P[P\x91P\x91P\x81\x80\x15a\x08\xDAWP\x80Q\x15\x80a\x08\xDAWP\x80\x80` \x01\x90Q\x81\x01\x90a\x08\xDA\x91\x90a\x10\xB7V[\x80\x15a\x08\xEFWP_\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x95\x94PPPPPV[_a\t\x0C`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\t^V[\x90P\x80Q_\x14\x15\x80\x15a\t0WP\x80\x80` \x01\x90Q\x81\x01\x90a\t.\x91\x90a\x10\xB7V[\x15[\x15a\tYW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\0\xF7V[PPPV[``a\x04@\x83\x83_\x84_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\t\x82\x91\x90a\x10\x9CV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\t\xBCW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\t\xC1V[``\x91P[P\x91P\x91Pa\t\xD1\x86\x83\x83a\t\xDBV[\x96\x95PPPPPPV[``\x82a\t\xF0Wa\t\xEB\x82a\n7V[a\x04@V[\x81Q\x15\x80\x15a\n\x07WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\n0W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\0\xF7V[P\x80a\x04@V[\x80Q\x15a\nGW\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\x9DWa\n\x9Da\ncV[`@RPPV[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xC7Wa\n\xC7a\ncV[`@R\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\n\xE6Wa\n\xE6a\ncV[P`\x05\x1B` \x01\x90V[\x805`\x02\x81\x10a\n\xFEW_\x80\xFD[\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n`W_\x80\xFD[\x805a\n\xFE\x81a\x0B\x03V[_\x82`\x1F\x83\x01\x12a\x0B1W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BKWa\x0BKa\ncV[`@Qa\x0Bb`\x1F\x83\x01`\x1F\x19\x16` \x01\x82a\nwV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0BvW_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x80\x15\x15\x81\x14a\n`W_\x80\xFD[\x805a\n\xFE\x81a\x0B\x92V[_\x80`@\x83\x85\x03\x12\x15a\x0B\xBBW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xD2W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0B\xE5W_\x80\xFD[\x815` a\x0B\xF2\x82a\n\xCDV[`@Qa\x0B\xFF\x82\x82a\nwV[\x83\x81R`\x05\x93\x90\x93\x1B\x85\x01\x82\x01\x92\x82\x81\x01\x91P\x89\x84\x11\x15a\x0C\x1EW_\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0C\xDDW\x805\x86\x81\x11\x15a\x0C8W_\x80\xFD[\x87\x01`\xE0\x81\x8D\x03`\x1F\x19\x01\x12\x15a\x0CMW_\x80\xFD[a\x0CUa\n\xA4V[a\x0C`\x86\x83\x01a\n\xF0V[\x81Ra\x0Cn`@\x83\x01a\x0B\x17V[\x86\x82\x01Ra\x0C~``\x83\x01a\x0B\x17V[`@\x82\x01R`\x80\x80\x83\x015\x89\x81\x11\x15a\x0C\x95W_\x80\xFD[a\x0C\xA3\x8F\x89\x83\x87\x01\x01a\x0B\"V[``\x84\x01RP`\xA0\x83\x81\x015\x91\x83\x01\x91\x90\x91R`\xC0\x80\x84\x015\x91\x83\x01\x91\x90\x91R`\xE0\x90\x92\x015\x91\x81\x01\x91\x90\x91R\x83R\x91\x83\x01\x91\x83\x01a\x0C\"V[P\x96Pa\x0C\xED\x90P\x87\x82\x01a\x0B\x9FV[\x94PPPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\0\x8CWa\0\x8Ca\x0C\xFAV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\rEW_\x80\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\0\x8CWa\0\x8Ca\x0C\xFAV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_` \x80\x83\x85\x03\x12\x15a\r\x84W_\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x9AW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\r\xAAW_\x80\xFD[\x80Qa\r\xB5\x81a\n\xCDV[`@Qa\r\xC2\x82\x82a\nwV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\r\xE1W_\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x0E\x08W\x83Qa\r\xF9\x81a\x0B\x03V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\r\xE6V[\x97\x96PPPPPPPV[` \x81\x01`\x02\x83\x10a\x0E3WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x91\x90R\x90V[_[\x83\x81\x10\x15a\x0ESW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0E;V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x0Er\x81` \x86\x01` \x86\x01a\x0E9V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x82\x81R`@` \x82\x01R_a\x0E\x9E`@\x83\x01\x84a\x0E[V[\x94\x93PPPPV[_`\xA0\x82\x01\x87\x83R` \x87` \x85\x01R`\xA0`@\x85\x01R\x81\x87Q\x80\x84R`\xC0\x86\x01\x91P` \x89\x01\x93P_[\x81\x81\x10\x15a\x0E\xF6W\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x0E\xD1V[PP`\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16``\x85\x01RPPP`\x80\x01R\x93\x92PPPV[_` \x80\x83\x85\x03\x12\x15a\x0F(W_\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F>W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x0FNW_\x80\xFD[\x80Qa\x0FY\x81a\n\xCDV[`@Qa\x0Ff\x82\x82a\nwV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\x0F\x85W_\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x0E\x08W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x0F\x8AV[_`\x03=\x11\x15a\x0F\xB9W`\x04_\x80>P_Q`\xE0\x1C[\x90V[_`D=\x10\x15a\x0F\xC9W\x90V[`@Q`\x03\x19=\x81\x01`\x04\x83>\x81Q=g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`$\x84\x01\x11\x81\x84\x11\x17\x15a\x0F\xF9WPPPPP\x90V[\x82\x85\x01\x91P\x81Q\x81\x81\x11\x15a\x10\x11WPPPPPP\x90V[\x84=\x87\x01\x01` \x82\x85\x01\x01\x11\x15a\x10+WPPPPPP\x90V[a\x10:` \x82\x86\x01\x01\x87a\nwV[P\x90\x95\x94PPPPPV[` \x81R_\x82Q`\xA0` \x84\x01Ra\x10``\xC0\x84\x01\x82a\x0E[V[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`@\x84\x01Q``\x84\x01R``\x84\x01Q`\x80\x84\x01R`\x80\x84\x01Q`\xA0\x84\x01R\x80\x91PP\x92\x91PPV[_\x82Qa\x10\xAD\x81\x84` \x87\x01a\x0E9V[\x91\x90\x91\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x10\xC7W_\x80\xFD[\x81Qa\x04@\x81a\x0B\x92V\xFE\xA2dipfsX\"\x12 5?\x14\xD8@|\xE0r\x04Bg\x07\xBF\x02\xB7\x93\xA7\xE1\n\x91\xFC \xC0\x7Fn[\xB9\xA0}\x96\xDE\xF1dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static SIMULATORABI_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80cJ\x0BD\xA2\x14a\0-W[_\x80\xFD[a\0@a\0;6`\x04a\x0B\xAAV[a\0RV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[_\x80a\0_\x84\x84_a\0\x92V[\x90P\x80`\x01\x82Qa\0p\x91\x90a\r\x0EV[\x81Q\x81\x10a\0\x80Wa\0\x80a\r!V[` \x02` \x01\x01Q\x91PP[\x92\x91PPV[``\x82\x80\x15a\0\xA2WP`\x02\x84Q\x10[\x15a\x01\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FChainSwap requires at least 2 sw`D\x82\x01Rbaps`\xE8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_``\x83a\x01\x0FW`\x01a\x01\x12V[\x85Q[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01*Wa\x01*a\ncV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01SW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P_\x80_[\x88Q\x81\x10\x15a\x04\x15W_\x89\x82\x81Q\x81\x10a\x01vWa\x01va\r!V[` \x02` \x01\x01Q\x90P_`\x01`\x01`\xA0\x1B\x03\x16\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x01\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x14\x9B\xDD]\x19\\\x88\x1A\\\xC8\x1B\x9D[\x1B`\x92\x1B`D\x82\x01R`d\x01a\0\xF7V[_\x81``\x01QQ\x11a\x02\x19W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x14\x18]\x1A\x08\x1A\\\xC8\x1B\x9D[\x1B`\xA2\x1B`D\x82\x01R`d\x01a\0\xF7V[`@\x81\x01Q\x89\x15a\x02\xCBW\x82_\x03a\x02:W\x81`\x80\x01Q\x94P_\x93Pa\x02\xDAV[`\x01\x8BQa\x02H\x91\x90a\r\x0EV[\x83\x03a\x02[W`\xA0\x82\x01Q\x93\x94Pa\x02\xDAV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\x9DW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xC1\x91\x90a\r5V[\x94P_\x93Pa\x02\xDAV[\x81`\x80\x01Q\x94P\x81`\xA0\x01Q\x93P[` \x82\x01Qa\x02\xF4\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x87a\x04GV[\x81`\xC0\x01Q_\x03a\x03\x10Wa\x03\nB`<a\rLV[`\xC0\x83\x01R[_\x82Q`\x01\x81\x11\x15a\x03$Wa\x03$a\r_V[\x03a\x03gW_\x82``\x01Q\x80` \x01\x90Q\x81\x01\x90a\x03B\x91\x90a\rsV[\x90Pa\x03Y\x83` \x01Q\x82\x88\x88\x87`\xC0\x01Qa\x04\xD4V[\x90\x99P\x97P\x94Pa\x03\xC2\x90PV[`\x01\x82Q`\x01\x81\x11\x15a\x03|Wa\x03|a\r_V[\x03a\x03\xA6Wa\x03\x9A\x82` \x01Q\x83``\x01Q\x87\x87\x86`\xC0\x01Qa\x06CV[\x90\x98P\x96P\x93Pa\x03\xC2V[\x81Q`@Qc\x95\x9E\xD9\xB9`\xE0\x1B\x81Ra\0\xF7\x91\x90`\x04\x01a\x0E\x13V[\x86\x15a\x03\xE5W\x82\x86`@Qc\x8B3le`\xE0\x1B\x81R`\x04\x01a\0\xF7\x92\x91\x90a\x0E\x86V[\x88\x15a\x04\x0BW\x83\x88\x84\x81Q\x81\x10a\x03\xFEWa\x03\xFEa\r!V[` \x02` \x01\x01\x81\x81RPP[PP`\x01\x01a\x01ZV[P\x85a\x04;W\x80\x85_\x81Q\x81\x10a\x04.Wa\x04.a\r!V[` \x02` \x01\x01\x81\x81RPP[PPPP[\x93\x92PPPV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R_\x91\x90\x85\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x94W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB8\x91\x90a\r5V[\x90Pa\x04\xCE\x84\x84a\x04\xC9\x85\x85a\rLV[a\x07\xA4V[PPPPV[`@Qc8\xED\x179`\xE0\x1B\x81R_\x90\x81\x90``\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c8\xED\x179\x90a\x05\x11\x90\x8A\x90\x8A\x90\x8D\x900\x90\x8C\x90`\x04\x01a\x0E\xA6V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x05NWP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x05K\x91\x90\x81\x01\x90a\x0F\x17V[`\x01[a\x05\xF8Wa\x05Za\x0F\xA3V[\x80c\x08\xC3y\xA0\x03a\x05\x89WPa\x05na\x0F\xBCV[\x80a\x05yWPa\x05\x8BV[_\x94P`\x01\x93P\x91Pa\x068\x90PV[P[=\x80\x80\x15a\x05\xB4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x05\xB9V[``\x91P[P_`\x01`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x13[\xDC\xDD\x1B\x1EH\x1C\x18Z\\\x88\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`Z\x1B\x81RP\x94P\x94P\x94PPPa\x068V[\x80`\x01\x82Qa\x06\x07\x91\x90a\r\x0EV[\x81Q\x81\x10a\x06\x17Wa\x06\x17a\r!V[` \x02` \x01\x01Q_`@Q\x80` \x01`@R\x80_\x81RP\x94P\x94P\x94PPP[\x95P\x95P\x95\x92PPPV[`@\x80Q`\xA0\x81\x01\x82R\x85\x81R0` \x82\x01R\x80\x82\x01\x83\x90R``\x81\x81\x01\x86\x90R`\x80\x82\x01\x85\x90R\x91Qc\xC0K\x8DY`\xE0\x1B\x81R_\x92\x83\x92\x90\x91\x89\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xC0K\x8DY\x90a\x06\x9F\x90\x84\x90`\x04\x01a\x10EV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x06\xD9WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x06\xD6\x91\x81\x01\x90a\r5V[`\x01[a\x07\x85Wa\x06\xE5a\x0F\xA3V[\x80c\x08\xC3y\xA0\x03a\x07\x15WPa\x06\xF9a\x0F\xBCV[\x80a\x07\x04WPa\x07\x17V[_\x95P`\x01\x94P\x92Pa\x068\x91PPV[P[=\x80\x80\x15a\x07@W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07EV[``\x91P[P_`\x01`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x13[\xDC\xDD\x1B\x1EH\x1C\x18Z\\\x88\x1B\x9B\xDD\x08\x19\x9B\xDD[\x99`Z\x1B\x81RP\x95P\x95P\x95PPPPa\x068V[\x80_`@Q\x80` \x01`@R\x80_\x81RP\x95P\x95P\x95PPPPa\x068V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x07\xF5\x84\x82a\x08WV[a\x04\xCEW`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16`$\x82\x01R_`D\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x08M\x90\x85\x90a\x08\xF8V[a\x04\xCE\x84\x82a\x08\xF8V[_\x80_\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x08r\x91\x90a\x10\x9CV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x08\xABW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x08\xB0V[``\x91P[P\x91P\x91P\x81\x80\x15a\x08\xDAWP\x80Q\x15\x80a\x08\xDAWP\x80\x80` \x01\x90Q\x81\x01\x90a\x08\xDA\x91\x90a\x10\xB7V[\x80\x15a\x08\xEFWP_\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x95\x94PPPPPV[_a\t\x0C`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\t^V[\x90P\x80Q_\x14\x15\x80\x15a\t0WP\x80\x80` \x01\x90Q\x81\x01\x90a\t.\x91\x90a\x10\xB7V[\x15[\x15a\tYW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\0\xF7V[PPPV[``a\x04@\x83\x83_\x84_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\t\x82\x91\x90a\x10\x9CV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\t\xBCW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\t\xC1V[``\x91P[P\x91P\x91Pa\t\xD1\x86\x83\x83a\t\xDBV[\x96\x95PPPPPPV[``\x82a\t\xF0Wa\t\xEB\x82a\n7V[a\x04@V[\x81Q\x15\x80\x15a\n\x07WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\n0W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\0\xF7V[P\x80a\x04@V[\x80Q\x15a\nGW\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\x9DWa\n\x9Da\ncV[`@RPPV[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xC7Wa\n\xC7a\ncV[`@R\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\n\xE6Wa\n\xE6a\ncV[P`\x05\x1B` \x01\x90V[\x805`\x02\x81\x10a\n\xFEW_\x80\xFD[\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n`W_\x80\xFD[\x805a\n\xFE\x81a\x0B\x03V[_\x82`\x1F\x83\x01\x12a\x0B1W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BKWa\x0BKa\ncV[`@Qa\x0Bb`\x1F\x83\x01`\x1F\x19\x16` \x01\x82a\nwV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0BvW_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x80\x15\x15\x81\x14a\n`W_\x80\xFD[\x805a\n\xFE\x81a\x0B\x92V[_\x80`@\x83\x85\x03\x12\x15a\x0B\xBBW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0B\xD2W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x0B\xE5W_\x80\xFD[\x815` a\x0B\xF2\x82a\n\xCDV[`@Qa\x0B\xFF\x82\x82a\nwV[\x83\x81R`\x05\x93\x90\x93\x1B\x85\x01\x82\x01\x92\x82\x81\x01\x91P\x89\x84\x11\x15a\x0C\x1EW_\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0C\xDDW\x805\x86\x81\x11\x15a\x0C8W_\x80\xFD[\x87\x01`\xE0\x81\x8D\x03`\x1F\x19\x01\x12\x15a\x0CMW_\x80\xFD[a\x0CUa\n\xA4V[a\x0C`\x86\x83\x01a\n\xF0V[\x81Ra\x0Cn`@\x83\x01a\x0B\x17V[\x86\x82\x01Ra\x0C~``\x83\x01a\x0B\x17V[`@\x82\x01R`\x80\x80\x83\x015\x89\x81\x11\x15a\x0C\x95W_\x80\xFD[a\x0C\xA3\x8F\x89\x83\x87\x01\x01a\x0B\"V[``\x84\x01RP`\xA0\x83\x81\x015\x91\x83\x01\x91\x90\x91R`\xC0\x80\x84\x015\x91\x83\x01\x91\x90\x91R`\xE0\x90\x92\x015\x91\x81\x01\x91\x90\x91R\x83R\x91\x83\x01\x91\x83\x01a\x0C\"V[P\x96Pa\x0C\xED\x90P\x87\x82\x01a\x0B\x9FV[\x94PPPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\0\x8CWa\0\x8Ca\x0C\xFAV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\rEW_\x80\xFD[PQ\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\0\x8CWa\0\x8Ca\x0C\xFAV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_` \x80\x83\x85\x03\x12\x15a\r\x84W_\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x9AW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\r\xAAW_\x80\xFD[\x80Qa\r\xB5\x81a\n\xCDV[`@Qa\r\xC2\x82\x82a\nwV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\r\xE1W_\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x0E\x08W\x83Qa\r\xF9\x81a\x0B\x03V[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\r\xE6V[\x97\x96PPPPPPPV[` \x81\x01`\x02\x83\x10a\x0E3WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x91\x90R\x90V[_[\x83\x81\x10\x15a\x0ESW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0E;V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x0Er\x81` \x86\x01` \x86\x01a\x0E9V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x82\x81R`@` \x82\x01R_a\x0E\x9E`@\x83\x01\x84a\x0E[V[\x94\x93PPPPV[_`\xA0\x82\x01\x87\x83R` \x87` \x85\x01R`\xA0`@\x85\x01R\x81\x87Q\x80\x84R`\xC0\x86\x01\x91P` \x89\x01\x93P_[\x81\x81\x10\x15a\x0E\xF6W\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x0E\xD1V[PP`\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16``\x85\x01RPPP`\x80\x01R\x93\x92PPPV[_` \x80\x83\x85\x03\x12\x15a\x0F(W_\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F>W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x0FNW_\x80\xFD[\x80Qa\x0FY\x81a\n\xCDV[`@Qa\x0Ff\x82\x82a\nwV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\x0F\x85W_\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x0E\x08W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x0F\x8AV[_`\x03=\x11\x15a\x0F\xB9W`\x04_\x80>P_Q`\xE0\x1C[\x90V[_`D=\x10\x15a\x0F\xC9W\x90V[`@Q`\x03\x19=\x81\x01`\x04\x83>\x81Q=g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81`$\x84\x01\x11\x81\x84\x11\x17\x15a\x0F\xF9WPPPPP\x90V[\x82\x85\x01\x91P\x81Q\x81\x81\x11\x15a\x10\x11WPPPPPP\x90V[\x84=\x87\x01\x01` \x82\x85\x01\x01\x11\x15a\x10+WPPPPPP\x90V[a\x10:` \x82\x86\x01\x01\x87a\nwV[P\x90\x95\x94PPPPPV[` \x81R_\x82Q`\xA0` \x84\x01Ra\x10``\xC0\x84\x01\x82a\x0E[V[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`@\x84\x01Q``\x84\x01R``\x84\x01Q`\x80\x84\x01R`\x80\x84\x01Q`\xA0\x84\x01R\x80\x91PP\x92\x91PPV[_\x82Qa\x10\xAD\x81\x84` \x87\x01a\x0E9V[\x91\x90\x91\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x10\xC7W_\x80\xFD[\x81Qa\x04@\x81a\x0B\x92V\xFE\xA2dipfsX\"\x12 5?\x14\xD8@|\xE0r\x04Bg\x07\xBF\x02\xB7\x93\xA7\xE1\n\x91\xFC \xC0\x7Fn[\xB9\xA0}\x96\xDE\xF1dsolcC\0\x08\x18\x003";
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
