pub use balancer_flash_loan_recipient_abi::*;
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
pub mod balancer_flash_loan_recipient_abi {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBalance"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getFlashLoanFeePercentage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getFlashLoanFeePercentage",
                            ),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLoanThenMultiSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getLoanThenMultiSwap",
                            ),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("returnOutput"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("profit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTokenBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTokenBalance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balances"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("receiveFlashLoan"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("receiveFlashLoan"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("feeAmounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("userData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdrawToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokens"),
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
                                    name: ::std::borrow::ToOwned::to_owned("amounts"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned(
                        "InsufficientFundsToRepayLoanError",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InsufficientFundsToRepayLoanError",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountToPayback"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
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
                    ::std::borrow::ToOwned::to_owned("SafeERC20FailedDecreaseAllowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SafeERC20FailedDecreaseAllowance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("spender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("currentAllowance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("requestedDecrease"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
    pub static BALANCERFLASHLOANRECIPIENTABI_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x0FW_\x80\xFD[P3a\0lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Fconstructor sender invalid addre`D\x82\x01Rass`\xF0\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[3`\xA0Rs\xBA\x12\"\"\"\"\x8D\x8B\xA4E\x95\x8Au\xA0pMVk\xF2\xC8`\x80R`\x80Q`\xA0Qa%\xC6a\0\xB7_9_a\n\xD7\x01R_\x81\x81a\x02\xCE\x01R\x81\x81a\x03\xAB\x01Ra\x04\x98\x01Ra%\xC6_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0zW_5`\xE0\x1C\x80c\xD8w\x84\\\x11a\0XW\x80c\xD8w\x84\\\x14a\0\xC1W\x80c\xF0O'\x07\x14a\0\xC9W\x80c\xF3\xC9\xC2\xC2\x14a\0\xDCW\x80c\xFFc\x89b\x14a\0\xEFW_\x80\xFD[\x80c\x12\x06_\xE0\x14a\0~W\x80c.\x1A}M\x14a\0\x99W\x80c\x83\xC1\r\x97\x14a\0\xAEW[_\x80\xFD[a\0\x86a\x01\x0FV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xACa\0\xA76`\x04a\x18vV[a\x01\x1EV[\0[a\0\x86a\0\xBC6`\x04a\x18\xE1V[a\x01TV[a\0\x86a\x03\xA8V[a\0\xACa\0\xD76`\x04a\x1A\xD3V[a\x04\x8DV[a\0\xACa\0\xEA6`\x04a\x1B\xDFV[a\x084V[a\x01\x02a\0\xFD6`\x04a\x1CEV[a\t\x7FV[`@Qa\0\x90\x91\x90a\x1D\x1EV[_a\x01\x18a\n\xCCV[PG[\x90V[a\x01&a\n\xCCV[`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01PW=_\x80>=_\xFD[PPV[_a\x01]a\n\xCCV[\x83a\x01\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\"\xB6\xB8:<\x909\xBB\xB0\xB8\x101\xB40\xB4\xB7`\x81\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x85\x85_\x81\x81\x10a\x01\xB5Wa\x01\xB5a\x1D0V[\x90P` \x02\x81\x01\x90a\x01\xC7\x91\x90a\x1DDV[a\x01\xD8\x90``\x81\x01\x90`@\x01a\x1DbV[\x90P_\x86\x86_\x81\x81\x10a\x01\xEDWa\x01\xEDa\x1D0V[\x90P` \x02\x81\x01\x90a\x01\xFF\x91\x90a\x1DDV[`\x80\x015\x90P_\x87\x87\x87\x87`@Q` \x01a\x02\x1D\x94\x93\x92\x91\x90a\x1E&V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R`\x01\x80\x84R\x83\x83\x01\x90\x92R\x92P_\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x83\x81_\x81Q\x81\x10a\x02]Wa\x02]a\x1D0V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P\x83\x81_\x81Q\x81\x10a\x02\xACWa\x02\xACa\x1D0V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc.\x1C\"O`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\\8D\x9E\x90a\x03\t\x900\x90\x86\x90\x86\x90\x89\x90`\x04\x01a\x1F\x81V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x03 W_\x80\xFD[PZ\xF1\x15\x80\x15a\x032W=_\x80>=_\xFD[PPPP\x89\x89_\x81\x81\x10a\x03HWa\x03Ha\x1D0V[\x90P` \x02\x81\x01\x90a\x03Z\x91\x90a\x1DDV[`\x80\x015\x8A\x8Aa\x03k`\x01\x82a \rV[\x81\x81\x10a\x03zWa\x03za\x1D0V[\x90P` \x02\x81\x01\x90a\x03\x8C\x91\x90a\x1DDV[`\xA0\x015a\x03\x9A\x91\x90a  V[\x9A\x99PPPPPPPPPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x94l+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x05W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04)\x91\x90a FV[`\x01`\x01`\xA0\x1B\x03\x16c\xD8w\x84\\`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04dW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x88\x91\x90a aV[\x90P\x90V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x13\xDB\x9B\x1EH\x1D\x98][\x1D\x08\x18\\\x99H\x18[\x1B\x1B\xDD\xD9Y`R\x1B`D\x82\x01R`d\x01a\x01\x99V[\x83Q`\x01\x14a\x05OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FOnly support one to one loan\0\0\0\0`D\x82\x01R`d\x01a\x01\x99V[\x82Q`\x01\x14a\x05\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FOnly support one to one loan\0\0\0\0`D\x82\x01R`d\x01a\x01\x99V[_\x80_\x83\x80` \x01\x90Q\x81\x01\x90a\x05\xB7\x91\x90a \xE5V[\x92P\x92P\x92P_a\x05\xC9\x84\x84\x84a\x0B>V[\x90Pa\x05\xFB`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nMulti swap done`\x88\x1B\x81RPa\x10\xBFV[a\x06\x05\x81Qa\x11\x05V[a\x064\x81`\x01\x83Qa\x06\x17\x91\x90a \rV[\x81Q\x81\x10a\x06'Wa\x06'a\x1D0V[` \x02` \x01\x01Qa\x11\x05V[_\x86_\x81Q\x81\x10a\x06GWa\x06Ga\x1D0V[` \x02` \x01\x01Q\x88_\x81Q\x81\x10a\x06aWa\x06aa\x1D0V[` \x02` \x01\x01Qa\x06s\x91\x90a\"@V[\x90P_\x82`\x01\x84Qa\x06\x85\x91\x90a \rV[\x81Q\x81\x10a\x06\x95Wa\x06\x95a\x1D0V[` \x02` \x01\x01Q\x90P\x80\x82\x11\x15a\x06\xF7W\x89_\x81Q\x81\x10a\x06\xB9Wa\x06\xB9a\x1D0V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\xB1\xE0\xB7\xCB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x01\x99V[a\x07(`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o*9<\x90(0\xBC\x90:42\x9067\xB0\xB7`\x81\x1B\x81RPa\x10\xBFV[_3\x90P\x8A_\x81Q\x81\x10a\x07>Wa\x07>a\x1D0V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x86\x90R\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07\x96W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xBA\x91\x90a\"SV[a\x07\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1C\x99\\\x18^H\x19\x98Z[\x19Y`\xA2\x1B`D\x82\x01R`d\x01a\x01\x99V[a\x08'`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01pPay the loan done`x\x1B\x81RPa\x10\xBFV[PPPPPPPPPPPV[a\x08<a\n\xCCV[\x82\x81\x14a\x08\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FThe length of tokens and amounts`D\x82\x01Rm\x08\x1B]\\\xDD\x08\x18\x99H\x19\\]X[`\x92\x1B`d\x82\x01R`\x84\x01a\x01\x99V[_[\x83\x81\x10\x15a\txW_\x85\x85\x83\x81\x81\x10a\x08\xBFWa\x08\xBFa\x1D0V[\x90P` \x02\x01` \x81\x01\x90a\x08\xD4\x91\x90a\x1DbV[\x90P_\x84\x84\x84\x81\x81\x10a\x08\xE9Wa\x08\xE9a\x1D0V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R` \x90\x91\x02\x92\x90\x92\x015`$\x83\x01\x81\x90R\x92PP`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\t@W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\td\x91\x90a\"SV[PPPa\tq\x81`\x01\x01\x90V[\x90Pa\x08\xA4V[PPPPPV[``a\t\x89a\n\xCCV[_\x82Q\x11a\t\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEmpty tokens`\xA0\x1B`D\x82\x01R`d\x01a\x01\x99V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xE1Wa\t\xE1a\x19DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\nW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82Q\x81\x10\x15a\n\xC5W_\x83\x82\x81Q\x81\x10a\n+Wa\n+a\x1D0V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n{W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9F\x91\x90a aV[\x83\x83\x81Q\x81\x10a\n\xB1Wa\n\xB1a\x1D0V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\n\x0FV[P[\x91\x90PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0B<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt,\xB7\xBA\x900\xB92\x9077\xBA\x10:42\x907\xBB\xB72\xB9`Y\x1B`D\x82\x01R`d\x01a\x01\x99V[V[``\x82\x80\x15a\x0BNWP`\x02\x84Q\x10[\x15a\x0B\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FChainSwap requires at least 2 sw`D\x82\x01Rbaps`\xE8\x1B`d\x82\x01R`\x84\x01a\x01\x99V[_``\x83a\x0B\xB6W`\x01a\x0B\xB9V[\x85Q[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xD0Wa\x0B\xD0a\x19DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xF9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P_\x80_[\x88Q\x81\x10\x15a\x10\x8DW_\x89\x82\x81Q\x81\x10a\x0C\x1CWa\x0C\x1Ca\x1D0V[` \x02` \x01\x01Q\x90Pa\x0CR`@Q\x80`@\x01`@R\x80`\n\x81R` \x01i\x033{\x91\x03c{{\x81\xD1`\xB5\x1B\x81RP\x83a\x11JV[` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x0C\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x14\x9B\xDD]\x19\\\x88\x1A\\\xC8\x1B\x9D[\x1B`\x92\x1B`D\x82\x01R`d\x01a\x01\x99V[_\x81``\x01QQ\x11a\x0C\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x14\x18]\x1A\x08\x1A\\\xC8\x1B\x9D[\x1B`\xA2\x1B`D\x82\x01R`d\x01a\x01\x99V[`@\x81\x01Q\x89\x15a\r\x92W\x82_\x03a\r\x01W\x81`\x80\x01Q\x94P_\x93Pa\r\xA1V[`\x01\x8BQa\r\x0F\x91\x90a \rV[\x83\x03a\r\"W`\xA0\x82\x01Q\x93\x94Pa\r\xA1V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rdW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x88\x91\x90a aV[\x94P_\x93Pa\r\xA1V[\x81`\x80\x01Q\x94P\x81`\xA0\x01Q\x93P[a\r\xD0`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l\x03\x1B\xAB\x92\x0Bk{\xABs\xA2Kq\xD1`\x9D\x1B\x81RP\x86a\x11JV[a\x0E\0`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x03\x1B\xAB\x92\x0Bk{\xABs\xA2{\xAB\xA1\xD1`\x95\x1B\x81RP\x85a\x11JV[a\x0E0`@Q\x80`@\x01`@R\x80`\n\x81R` \x01i\x03\xA3{[+q\x03Kq\xD1`\xB5\x1B\x81RP\x83`@\x01Qa\x11\x8FV[` \x82\x01Qa\x0EJ\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x87a\x11\xD4V[\x81`\xC0\x01Q_\x03a\x0EfWa\x0E`B`<a\"@V[`\xC0\x83\x01R[_\x82Q`\x01\x81\x11\x15a\x0EzWa\x0Eza\x1D\x89V[\x03a\x0F0Wa\x0E\xB2`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qabi.decode: Before`p\x1B\x81RPa\x10\xBFV[a\x0E\xC0\x82``\x01QQa\x11\x05V[_\x82``\x01Q\x80` \x01\x90Q\x81\x01\x90a\x0E\xD9\x91\x90a\"nV[\x90Pa\x0F\r`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p0\xB14\x9722\xB1\xB7\xB22\x9D\x10 \xB3:2\xB9`y\x1B\x81RPa\x10\xBFV[a\x0F\"\x83` \x01Q\x82\x88\x88\x87`\xC0\x01Qa\x12aV[\x90\x99P\x97P\x94Pa\x0F\x8B\x90PV[`\x01\x82Q`\x01\x81\x11\x15a\x0FEWa\x0FEa\x1D\x89V[\x03a\x0FoWa\x0Fc\x82` \x01Q\x83``\x01Q\x87\x87\x86`\xC0\x01Qa\x13\xDAV[\x90\x98P\x96P\x93Pa\x0F\x8BV[\x81Q`@Qc\x95\x9E\xD9\xB9`\xE0\x1B\x81Ra\x01\x99\x91\x90`\x04\x01a#\x02V[a\x0F\xB7`@Q\x80`@\x01`@R\x80`\n\x81R` \x01i\x03\x9B\xBB\x0B\x81\x03{\xAB\xA1\xD1`\xB5\x1B\x81RP\x85a\x11JV[\x86\x15a\x0F\xF4W` \x82\x01Qa\x0F\xD7\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x87a\x14\xD5V[\x82\x86`@Qc\x8B3le`\xE0\x1B\x81R`\x04\x01a\x01\x99\x92\x91\x90a#\x10V[a\x10\x1F`@Q\x80`@\x01`@R\x80`\t\x81R` \x01hSwap done`\xB8\x1B\x81RP\x84a\x11JV[a\x10]`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7F============================\0\0\0\0\x81RPa\x10\xBFV[\x88\x15a\x10\x83W\x83\x88\x84\x81Q\x81\x10a\x10vWa\x10va\x1D0V[` \x02` \x01\x01\x81\x81RPP[PP`\x01\x01a\x0C\0V[P\x85a\x10\xB3W\x80\x85_\x81Q\x81\x10a\x10\xA6Wa\x10\xA6a\x1D0V[` \x02` \x01\x01\x81\x81RPP[PPPP[\x93\x92PPPV[a\x11\x02\x81`@Q`$\x01a\x10\xD3\x91\x90a#0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x15\x8FV[PV[a\x11\x02\x81`@Q`$\x01a\x11\x1B\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xF8,P\xF1`\xE0\x1B\x17\x90Ra\x15\x8FV[a\x01P\x82\x82`@Q`$\x01a\x11`\x92\x91\x90a#BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x15\x8FV[a\x01P\x82\x82`@Q`$\x01a\x11\xA5\x92\x91\x90a#cV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c1\x9A\xF33`\xE0\x1B\x17\x90Ra\x15\x8FV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R_\x91\x90\x85\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12!W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12E\x91\x90a aV[\x90Pa\x12[\x84\x84a\x12V\x85\x85a\"@V[a\x15\x98V[PPPPV[`@Qc8\xED\x179`\xE0\x1B\x81R_\x90\x81\x90``\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c8\xED\x179\x90a\x12\x9E\x90\x8A\x90\x8A\x90\x8D\x900\x90\x8C\x90`\x04\x01a#\x8CV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x12\xDBWP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\xD8\x91\x90\x81\x01\x90a#\xFDV[`\x01[a\x13\x8FWa\x12\xE7a$\x88V[\x80c\x08\xC3y\xA0\x03a\x13\x16WPa\x12\xFBa$\xA0V[\x80a\x13\x06WPa\x13\x18V[_\x94P`\x01\x93P\x91Pa\x13\xCF\x90PV[P[=\x80\x80\x15a\x13AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x13FV[``\x91P[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FMostly UniswapV2 pair not found\0`D\x82\x01R`d\x01a\x01\x99V[\x80`\x01\x82Qa\x13\x9E\x91\x90a \rV[\x81Q\x81\x10a\x13\xAEWa\x13\xAEa\x1D0V[` \x02` \x01\x01Q_`@Q\x80` \x01`@R\x80_\x81RP\x94P\x94P\x94PPP[\x95P\x95P\x95\x92PPPV[`@\x80Q`\xA0\x81\x01\x82R\x85\x81R0` \x82\x01R\x80\x82\x01\x83\x90R``\x81\x81\x01\x86\x90R`\x80\x82\x01\x85\x90R\x91Qc\xC0K\x8DY`\xE0\x1B\x81R_\x92\x83\x92\x90\x91\x89\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xC0K\x8DY\x90a\x146\x90\x84\x90`\x04\x01a%(V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x14pWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x14m\x91\x81\x01\x90a aV[`\x01[a\x14\xB6Wa\x14|a$\x88V[\x80c\x08\xC3y\xA0\x03a\x14\xACWPa\x14\x90a$\xA0V[\x80a\x14\x9BWPa\x14\xAEV[_\x95P`\x01\x94P\x92Pa\x13\xCF\x91PPV[P[=_\x80>=_\xFD[\x80_`@Q\x80` \x01`@R\x80_\x81RP\x95P\x95P\x95PPPPa\x13\xCFV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R_\x91\x90\x85\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\"W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15F\x91\x90a aV[\x90P\x81\x81\x10\x15a\x15\x82W`@Qc\xE5p\x11\x0F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x01\x99V[a\x12[\x84\x84\x84\x84\x03a\x15\x98V[a\x11\x02\x81a\x16KV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x15\xE9\x84\x82a\x16kV[a\x12[W`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16`$\x82\x01R_`D\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x16A\x90\x85\x90a\x17\x0EV[a\x12[\x84\x82a\x17\x0EV[\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[_\x80_\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x16\x86\x91\x90a%\x7FV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x16\xBFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x16\xC4V[``\x91P[P\x91P\x91P\x81\x80\x15a\x16\xEEWP\x80Q\x15\x80a\x16\xEEWP\x80\x80` \x01\x90Q\x81\x01\x90a\x16\xEE\x91\x90a\"SV[\x80\x15a\x17\x03WP_\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x92PPP[\x92\x91PPV[_a\x17\"`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x17tV[\x90P\x80Q_\x14\x15\x80\x15a\x17FWP\x80\x80` \x01\x90Q\x81\x01\x90a\x17D\x91\x90a\"SV[\x15[\x15a\x17oW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x01\x99V[PPPV[``a\x10\xB8\x83\x83_\x84_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x17\x98\x91\x90a%\x7FV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x17\xD2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x17\xD7V[``\x91P[P\x91P\x91Pa\x17\xE7\x86\x83\x83a\x17\xF1V[\x96\x95PPPPPPV[``\x82a\x18\x06Wa\x18\x01\x82a\x18MV[a\x10\xB8V[\x81Q\x15\x80\x15a\x18\x1DWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x18FW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x01\x99V[P\x80a\x10\xB8V[\x80Q\x15a\x18]W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_` \x82\x84\x03\x12\x15a\x18\x86W_\x80\xFD[P5\x91\x90PV[_\x80\x83`\x1F\x84\x01\x12a\x18\x9DW_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xB3W_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x18\xCDW_\x80\xFD[\x92P\x92\x90PV[\x80\x15\x15\x81\x14a\x11\x02W_\x80\xFD[_\x80_\x80``\x85\x87\x03\x12\x15a\x18\xF4W_\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\tW_\x80\xFD[a\x19\x15\x87\x82\x88\x01a\x18\x8DV[\x90\x95P\x93PP` \x85\x015a\x19)\x81a\x18\xD4V[\x91P`@\x85\x015a\x199\x81a\x18\xD4V[\x93\x96\x92\x95P\x90\x93PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x19}Wa\x19}a\x19DV[`@RPPV[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x19\xA6Wa\x19\xA6a\x19DV[`@R\x90V[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x19\xC4Wa\x19\xC4a\x19DV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11\x02W_\x80\xFD[_\x82`\x1F\x83\x01\x12a\x19\xF1W_\x80\xFD[\x815` a\x19\xFE\x82a\x19\xACV[`@Qa\x1A\x0B\x82\x82a\x19XV[\x80\x91P\x83\x81R` \x81\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x1A.W_\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1AJW\x805\x83R\x91\x83\x01\x91\x83\x01a\x1A3V[P\x96\x95PPPPPPV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1AmWa\x1Ama\x19DV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x1A\x8AW_\x80\xFD[\x815a\x1A\x95\x81a\x1AUV[`@Qa\x1A\xA2\x82\x82a\x19XV[\x82\x81R\x85` \x84\x87\x01\x01\x11\x15a\x1A\xB6W_\x80\xFD[\x82` \x86\x01` \x83\x017_\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x1A\xE6W_\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1A\xFCW_\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x1B\x0FW_\x80\xFD[\x815` a\x1B\x1C\x82a\x19\xACV[`@Qa\x1B)\x82\x82a\x19XV[\x83\x81R`\x05\x93\x90\x93\x1B\x85\x01\x82\x01\x92\x82\x81\x01\x91P\x8B\x84\x11\x15a\x1BHW_\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x1BoW\x855a\x1B`\x81a\x19\xCEV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x1BMV[\x98PP\x88\x015\x92PP\x80\x82\x11\x15a\x1B\x84W_\x80\xFD[a\x1B\x90\x88\x83\x89\x01a\x19\xE2V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x1B\xA5W_\x80\xFD[a\x1B\xB1\x88\x83\x89\x01a\x19\xE2V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x1B\xC6W_\x80\xFD[Pa\x1B\xD3\x87\x82\x88\x01a\x1A{V[\x91PP\x92\x95\x91\x94P\x92PV[_\x80_\x80`@\x85\x87\x03\x12\x15a\x1B\xF2W_\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1C\x08W_\x80\xFD[a\x1C\x14\x88\x83\x89\x01a\x18\x8DV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x1C,W_\x80\xFD[Pa\x1C9\x87\x82\x88\x01a\x18\x8DV[\x95\x98\x94\x97P\x95PPPPV[_` \x80\x83\x85\x03\x12\x15a\x1CVW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1CkW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x1C{W_\x80\xFD[\x805a\x1C\x86\x81a\x19\xACV[`@Qa\x1C\x93\x82\x82a\x19XV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\x1C\xB2W_\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x1C\xD9W\x835a\x1C\xCA\x81a\x19\xCEV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x1C\xB7V[\x97\x96PPPPPPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a\x1D\x13W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x1C\xF7V[P\x94\x95\x94PPPPPV[` \x81R_a\x10\xB8` \x83\x01\x84a\x1C\xE4V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`\xDE\x19\x836\x03\x01\x81\x12a\x1DXW_\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1DrW_\x80\xFD[\x815a\x10\xB8\x81a\x19\xCEV[`\x02\x81\x10a\x11\x02W_\x80\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x02\x81\x10a\x1D\xB9WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1D\xD2W_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xF0W_\x80\xFD[\x806\x03\x82\x13\x15a\x18\xCDW_\x80\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[``\x80\x82R\x81\x81\x01\x85\x90R_\x90`\x80\x80\x84\x01`\x05\x88\x90\x1B\x85\x01\x82\x01\x89\x85[\x8A\x81\x10\x15a\x1F\rW\x87\x83\x03`\x7F\x19\x01\x84R\x8156\x8D\x90\x03`\xDE\x19\x01\x81\x12a\x1EiW_\x80\xFD[\x8C\x01`\xE0\x815a\x1Ex\x81a\x1D}V[a\x1E\x82\x86\x82a\x1D\x9DV[P` \x80\x83\x015a\x1E\x92\x81a\x19\xCEV[`\x01`\x01`\xA0\x1B\x03\x16\x86\x82\x01R`@\x83\x81\x015a\x1E\xAE\x81a\x19\xCEV[`\x01`\x01`\xA0\x1B\x03\x16\x90\x87\x01Ra\x1E\xC7\x83\x8A\x01\x84a\x1D\xBDV[\x83\x8B\x89\x01Ra\x1E\xD9\x84\x89\x01\x82\x84a\x1D\xFEV[\x85\x8B\x015\x89\x8C\x01R`\xA0\x80\x87\x015\x90\x8A\x01R`\xC0\x95\x86\x015\x95\x90\x98\x01\x94\x90\x94RP\x95\x86\x01\x95\x93\x90\x93\x01\x92PP`\x01\x01a\x1EDV[PP\x87\x15\x15` \x87\x01R\x93Pa\x1F\"\x92PPPV[\x82\x15\x15`@\x83\x01R[\x95\x94PPPPPV[_[\x83\x81\x10\x15a\x1FNW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1F6V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x1Fm\x81` \x86\x01` \x86\x01a\x1F4V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R`\x80` \x80\x84\x01\x82\x90R\x86Q\x91\x84\x01\x82\x90R_\x92\x87\x82\x01\x92\x90\x91\x90`\xA0\x86\x01\x90\x85[\x81\x81\x10\x15a\x1F\xCEW\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a\x1F\xB0V[PP\x85\x81\x03`@\x87\x01Ra\x1F\xE2\x81\x89a\x1C\xE4V[\x93PPPP\x82\x81\x03``\x84\x01Ra\x1C\xD9\x81\x85a\x1FVV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x17\x08Wa\x17\x08a\x1F\xF9V[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a ?Wa ?a\x1F\xF9V[P\x92\x91PPV[_` \x82\x84\x03\x12\x15a VW_\x80\xFD[\x81Qa\x10\xB8\x81a\x19\xCEV[_` \x82\x84\x03\x12\x15a qW_\x80\xFD[PQ\x91\x90PV[\x80Qa\n\xC7\x81a\x1D}V[\x80Qa\n\xC7\x81a\x19\xCEV[_\x82`\x1F\x83\x01\x12a \x9DW_\x80\xFD[\x81Qa \xA8\x81a\x1AUV[`@Qa \xB5\x82\x82a\x19XV[\x82\x81R\x85` \x84\x87\x01\x01\x11\x15a \xC9W_\x80\xFD[a\x1F+\x83` \x83\x01` \x88\x01a\x1F4V[\x80Qa\n\xC7\x81a\x18\xD4V[_\x80_``\x84\x86\x03\x12\x15a \xF7W_\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a!\rW_\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a! W_\x80\xFD[\x81Q` a!-\x82a\x19\xACV[`@Qa!:\x82\x82a\x19XV[\x83\x81R`\x05\x93\x90\x93\x1B\x85\x01\x82\x01\x92\x82\x81\x01\x91P\x8A\x84\x11\x15a!YW_\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\"\x15W\x80Q\x86\x81\x11\x15a!sW_\x80\xFD[\x87\x01`\xE0\x81\x8E\x03`\x1F\x19\x01\x12\x15a!\x88W_\x80\xFD[a!\x90a\x19\x84V[a!\x9B\x86\x83\x01a xV[\x81Ra!\xA9`@\x83\x01a \x83V[\x86\x82\x01Ra!\xB9``\x83\x01a \x83V[`@\x82\x01R`\x80\x82\x01Q\x88\x81\x11\x15a!\xCFW_\x80\xFD[a!\xDD\x8F\x88\x83\x86\x01\x01a \x8EV[``\x83\x01RP`\xA0\x82\x81\x01Q`\x80\x83\x01R`\xC0\x80\x84\x01Q\x91\x83\x01\x91\x90\x91R`\xE0\x90\x92\x01Q\x91\x81\x01\x91\x90\x91R\x83R\x91\x83\x01\x91\x83\x01a!]V[P\x97Pa\"%\x90P\x88\x82\x01a \xDAV[\x95PPPPPa\"7`@\x85\x01a \xDAV[\x90P\x92P\x92P\x92V[\x80\x82\x01\x80\x82\x11\x15a\x17\x08Wa\x17\x08a\x1F\xF9V[_` \x82\x84\x03\x12\x15a\"cW_\x80\xFD[\x81Qa\x10\xB8\x81a\x18\xD4V[_` \x80\x83\x85\x03\x12\x15a\"\x7FW_\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x94W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\"\xA4W_\x80\xFD[\x80Qa\"\xAF\x81a\x19\xACV[`@Qa\"\xBC\x82\x82a\x19XV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\"\xDBW_\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x1C\xD9W\x83Qa\"\xF3\x81a\x19\xCEV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\"\xE0V[` \x81\x01a\x17\x08\x82\x84a\x1D\x9DV[\x82\x81R`@` \x82\x01R_a#(`@\x83\x01\x84a\x1FVV[\x94\x93PPPPV[` \x81R_a\x10\xB8` \x83\x01\x84a\x1FVV[`@\x81R_a#T`@\x83\x01\x85a\x1FVV[\x90P\x82` \x83\x01R\x93\x92PPPV[`@\x81R_a#u`@\x83\x01\x85a\x1FVV[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[_`\xA0\x82\x01\x87\x83R` \x87` \x85\x01R`\xA0`@\x85\x01R\x81\x87Q\x80\x84R`\xC0\x86\x01\x91P` \x89\x01\x93P_[\x81\x81\x10\x15a#\xDCW\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a#\xB7V[PP`\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16``\x85\x01RPPP`\x80\x01R\x93\x92PPPV[_` \x80\x83\x85\x03\x12\x15a$\x0EW_\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a$#W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a$3W_\x80\xFD[\x80Qa$>\x81a\x19\xACV[`@Qa$K\x82\x82a\x19XV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a$jW_\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x1C\xD9W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a$oV[_`\x03=\x11\x15a\x01\x1BW`\x04_\x80>P_Q`\xE0\x1C\x90V[_`D=\x10\x15a$\xADW\x90V[`@Q`\x03\x19=\x81\x01`\x04\x83>\x81Q=`\x01`\x01`@\x1B\x03\x81`$\x84\x01\x11\x81\x84\x11\x17\x15a$\xDCWPPPPP\x90V[\x82\x85\x01\x91P\x81Q\x81\x81\x11\x15a$\xF4WPPPPPP\x90V[\x84=\x87\x01\x01` \x82\x85\x01\x01\x11\x15a%\x0EWPPPPPP\x90V[a%\x1D` \x82\x86\x01\x01\x87a\x19XV[P\x90\x95\x94PPPPPV[` \x81R_\x82Q`\xA0` \x84\x01Ra%C`\xC0\x84\x01\x82a\x1FVV[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`@\x84\x01Q``\x84\x01R``\x84\x01Q`\x80\x84\x01R`\x80\x84\x01Q`\xA0\x84\x01R\x80\x91PP\x92\x91PPV[_\x82Qa\x1DX\x81\x84` \x87\x01a\x1F4V\xFE\xA2dipfsX\"\x12 \xACK\xBC\\\x1D\x84\xC4\xE2c\x14U\xCE\xB3\xF1\xDB[\xF5\x90\xBF\xC0\x17\xA4\xE6\xD4_\xFDwB\xDA\xEB\x19\tdsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static BALANCERFLASHLOANRECIPIENTABI_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0zW_5`\xE0\x1C\x80c\xD8w\x84\\\x11a\0XW\x80c\xD8w\x84\\\x14a\0\xC1W\x80c\xF0O'\x07\x14a\0\xC9W\x80c\xF3\xC9\xC2\xC2\x14a\0\xDCW\x80c\xFFc\x89b\x14a\0\xEFW_\x80\xFD[\x80c\x12\x06_\xE0\x14a\0~W\x80c.\x1A}M\x14a\0\x99W\x80c\x83\xC1\r\x97\x14a\0\xAEW[_\x80\xFD[a\0\x86a\x01\x0FV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xACa\0\xA76`\x04a\x18vV[a\x01\x1EV[\0[a\0\x86a\0\xBC6`\x04a\x18\xE1V[a\x01TV[a\0\x86a\x03\xA8V[a\0\xACa\0\xD76`\x04a\x1A\xD3V[a\x04\x8DV[a\0\xACa\0\xEA6`\x04a\x1B\xDFV[a\x084V[a\x01\x02a\0\xFD6`\x04a\x1CEV[a\t\x7FV[`@Qa\0\x90\x91\x90a\x1D\x1EV[_a\x01\x18a\n\xCCV[PG[\x90V[a\x01&a\n\xCCV[`@Q3\x90\x82\x15a\x08\xFC\x02\x90\x83\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x01PW=_\x80>=_\xFD[PPV[_a\x01]a\n\xCCV[\x83a\x01\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\"\xB6\xB8:<\x909\xBB\xB0\xB8\x101\xB40\xB4\xB7`\x81\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x85\x85_\x81\x81\x10a\x01\xB5Wa\x01\xB5a\x1D0V[\x90P` \x02\x81\x01\x90a\x01\xC7\x91\x90a\x1DDV[a\x01\xD8\x90``\x81\x01\x90`@\x01a\x1DbV[\x90P_\x86\x86_\x81\x81\x10a\x01\xEDWa\x01\xEDa\x1D0V[\x90P` \x02\x81\x01\x90a\x01\xFF\x91\x90a\x1DDV[`\x80\x015\x90P_\x87\x87\x87\x87`@Q` \x01a\x02\x1D\x94\x93\x92\x91\x90a\x1E&V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R`\x01\x80\x84R\x83\x83\x01\x90\x92R\x92P_\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x83\x81_\x81Q\x81\x10a\x02]Wa\x02]a\x1D0V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P\x83\x81_\x81Q\x81\x10a\x02\xACWa\x02\xACa\x1D0V[` \x90\x81\x02\x91\x90\x91\x01\x01R`@Qc.\x1C\"O`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\\8D\x9E\x90a\x03\t\x900\x90\x86\x90\x86\x90\x89\x90`\x04\x01a\x1F\x81V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x03 W_\x80\xFD[PZ\xF1\x15\x80\x15a\x032W=_\x80>=_\xFD[PPPP\x89\x89_\x81\x81\x10a\x03HWa\x03Ha\x1D0V[\x90P` \x02\x81\x01\x90a\x03Z\x91\x90a\x1DDV[`\x80\x015\x8A\x8Aa\x03k`\x01\x82a \rV[\x81\x81\x10a\x03zWa\x03za\x1D0V[\x90P` \x02\x81\x01\x90a\x03\x8C\x91\x90a\x1DDV[`\xA0\x015a\x03\x9A\x91\x90a  V[\x9A\x99PPPPPPPPPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x94l+`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x05W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04)\x91\x90a FV[`\x01`\x01`\xA0\x1B\x03\x16c\xD8w\x84\\`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04dW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x88\x91\x90a aV[\x90P\x90V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x13\xDB\x9B\x1EH\x1D\x98][\x1D\x08\x18\\\x99H\x18[\x1B\x1B\xDD\xD9Y`R\x1B`D\x82\x01R`d\x01a\x01\x99V[\x83Q`\x01\x14a\x05OW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FOnly support one to one loan\0\0\0\0`D\x82\x01R`d\x01a\x01\x99V[\x82Q`\x01\x14a\x05\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FOnly support one to one loan\0\0\0\0`D\x82\x01R`d\x01a\x01\x99V[_\x80_\x83\x80` \x01\x90Q\x81\x01\x90a\x05\xB7\x91\x90a \xE5V[\x92P\x92P\x92P_a\x05\xC9\x84\x84\x84a\x0B>V[\x90Pa\x05\xFB`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nMulti swap done`\x88\x1B\x81RPa\x10\xBFV[a\x06\x05\x81Qa\x11\x05V[a\x064\x81`\x01\x83Qa\x06\x17\x91\x90a \rV[\x81Q\x81\x10a\x06'Wa\x06'a\x1D0V[` \x02` \x01\x01Qa\x11\x05V[_\x86_\x81Q\x81\x10a\x06GWa\x06Ga\x1D0V[` \x02` \x01\x01Q\x88_\x81Q\x81\x10a\x06aWa\x06aa\x1D0V[` \x02` \x01\x01Qa\x06s\x91\x90a\"@V[\x90P_\x82`\x01\x84Qa\x06\x85\x91\x90a \rV[\x81Q\x81\x10a\x06\x95Wa\x06\x95a\x1D0V[` \x02` \x01\x01Q\x90P\x80\x82\x11\x15a\x06\xF7W\x89_\x81Q\x81\x10a\x06\xB9Wa\x06\xB9a\x1D0V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\xB1\xE0\xB7\xCB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x01\x99V[a\x07(`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o*9<\x90(0\xBC\x90:42\x9067\xB0\xB7`\x81\x1B\x81RPa\x10\xBFV[_3\x90P\x8A_\x81Q\x81\x10a\x07>Wa\x07>a\x1D0V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`$\x82\x01\x86\x90R\x90\x91\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07\x96W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xBA\x91\x90a\"SV[a\x07\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x1C\x99\\\x18^H\x19\x98Z[\x19Y`\xA2\x1B`D\x82\x01R`d\x01a\x01\x99V[a\x08'`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01pPay the loan done`x\x1B\x81RPa\x10\xBFV[PPPPPPPPPPPV[a\x08<a\n\xCCV[\x82\x81\x14a\x08\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FThe length of tokens and amounts`D\x82\x01Rm\x08\x1B]\\\xDD\x08\x18\x99H\x19\\]X[`\x92\x1B`d\x82\x01R`\x84\x01a\x01\x99V[_[\x83\x81\x10\x15a\txW_\x85\x85\x83\x81\x81\x10a\x08\xBFWa\x08\xBFa\x1D0V[\x90P` \x02\x01` \x81\x01\x90a\x08\xD4\x91\x90a\x1DbV[\x90P_\x84\x84\x84\x81\x81\x10a\x08\xE9Wa\x08\xE9a\x1D0V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R` \x90\x91\x02\x92\x90\x92\x015`$\x83\x01\x81\x90R\x92PP`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\t@W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\td\x91\x90a\"SV[PPPa\tq\x81`\x01\x01\x90V[\x90Pa\x08\xA4V[PPPPPV[``a\t\x89a\n\xCCV[_\x82Q\x11a\t\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEmpty tokens`\xA0\x1B`D\x82\x01R`d\x01a\x01\x99V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xE1Wa\t\xE1a\x19DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\nW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82Q\x81\x10\x15a\n\xC5W_\x83\x82\x81Q\x81\x10a\n+Wa\n+a\x1D0V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n{W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9F\x91\x90a aV[\x83\x83\x81Q\x81\x10a\n\xB1Wa\n\xB1a\x1D0V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\n\x0FV[P[\x91\x90PV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0B<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt,\xB7\xBA\x900\xB92\x9077\xBA\x10:42\x907\xBB\xB72\xB9`Y\x1B`D\x82\x01R`d\x01a\x01\x99V[V[``\x82\x80\x15a\x0BNWP`\x02\x84Q\x10[\x15a\x0B\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FChainSwap requires at least 2 sw`D\x82\x01Rbaps`\xE8\x1B`d\x82\x01R`\x84\x01a\x01\x99V[_``\x83a\x0B\xB6W`\x01a\x0B\xB9V[\x85Q[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xD0Wa\x0B\xD0a\x19DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xF9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x92P_\x80_[\x88Q\x81\x10\x15a\x10\x8DW_\x89\x82\x81Q\x81\x10a\x0C\x1CWa\x0C\x1Ca\x1D0V[` \x02` \x01\x01Q\x90Pa\x0CR`@Q\x80`@\x01`@R\x80`\n\x81R` \x01i\x033{\x91\x03c{{\x81\xD1`\xB5\x1B\x81RP\x83a\x11JV[` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x0C\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x14\x9B\xDD]\x19\\\x88\x1A\\\xC8\x1B\x9D[\x1B`\x92\x1B`D\x82\x01R`d\x01a\x01\x99V[_\x81``\x01QQ\x11a\x0C\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x14\x18]\x1A\x08\x1A\\\xC8\x1B\x9D[\x1B`\xA2\x1B`D\x82\x01R`d\x01a\x01\x99V[`@\x81\x01Q\x89\x15a\r\x92W\x82_\x03a\r\x01W\x81`\x80\x01Q\x94P_\x93Pa\r\xA1V[`\x01\x8BQa\r\x0F\x91\x90a \rV[\x83\x03a\r\"W`\xA0\x82\x01Q\x93\x94Pa\r\xA1V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rdW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x88\x91\x90a aV[\x94P_\x93Pa\r\xA1V[\x81`\x80\x01Q\x94P\x81`\xA0\x01Q\x93P[a\r\xD0`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l\x03\x1B\xAB\x92\x0Bk{\xABs\xA2Kq\xD1`\x9D\x1B\x81RP\x86a\x11JV[a\x0E\0`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x03\x1B\xAB\x92\x0Bk{\xABs\xA2{\xAB\xA1\xD1`\x95\x1B\x81RP\x85a\x11JV[a\x0E0`@Q\x80`@\x01`@R\x80`\n\x81R` \x01i\x03\xA3{[+q\x03Kq\xD1`\xB5\x1B\x81RP\x83`@\x01Qa\x11\x8FV[` \x82\x01Qa\x0EJ\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x87a\x11\xD4V[\x81`\xC0\x01Q_\x03a\x0EfWa\x0E`B`<a\"@V[`\xC0\x83\x01R[_\x82Q`\x01\x81\x11\x15a\x0EzWa\x0Eza\x1D\x89V[\x03a\x0F0Wa\x0E\xB2`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01qabi.decode: Before`p\x1B\x81RPa\x10\xBFV[a\x0E\xC0\x82``\x01QQa\x11\x05V[_\x82``\x01Q\x80` \x01\x90Q\x81\x01\x90a\x0E\xD9\x91\x90a\"nV[\x90Pa\x0F\r`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01p0\xB14\x9722\xB1\xB7\xB22\x9D\x10 \xB3:2\xB9`y\x1B\x81RPa\x10\xBFV[a\x0F\"\x83` \x01Q\x82\x88\x88\x87`\xC0\x01Qa\x12aV[\x90\x99P\x97P\x94Pa\x0F\x8B\x90PV[`\x01\x82Q`\x01\x81\x11\x15a\x0FEWa\x0FEa\x1D\x89V[\x03a\x0FoWa\x0Fc\x82` \x01Q\x83``\x01Q\x87\x87\x86`\xC0\x01Qa\x13\xDAV[\x90\x98P\x96P\x93Pa\x0F\x8BV[\x81Q`@Qc\x95\x9E\xD9\xB9`\xE0\x1B\x81Ra\x01\x99\x91\x90`\x04\x01a#\x02V[a\x0F\xB7`@Q\x80`@\x01`@R\x80`\n\x81R` \x01i\x03\x9B\xBB\x0B\x81\x03{\xAB\xA1\xD1`\xB5\x1B\x81RP\x85a\x11JV[\x86\x15a\x0F\xF4W` \x82\x01Qa\x0F\xD7\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x87a\x14\xD5V[\x82\x86`@Qc\x8B3le`\xE0\x1B\x81R`\x04\x01a\x01\x99\x92\x91\x90a#\x10V[a\x10\x1F`@Q\x80`@\x01`@R\x80`\t\x81R` \x01hSwap done`\xB8\x1B\x81RP\x84a\x11JV[a\x10]`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7F============================\0\0\0\0\x81RPa\x10\xBFV[\x88\x15a\x10\x83W\x83\x88\x84\x81Q\x81\x10a\x10vWa\x10va\x1D0V[` \x02` \x01\x01\x81\x81RPP[PP`\x01\x01a\x0C\0V[P\x85a\x10\xB3W\x80\x85_\x81Q\x81\x10a\x10\xA6Wa\x10\xA6a\x1D0V[` \x02` \x01\x01\x81\x81RPP[PPPP[\x93\x92PPPV[a\x11\x02\x81`@Q`$\x01a\x10\xD3\x91\x90a#0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x15\x8FV[PV[a\x11\x02\x81`@Q`$\x01a\x11\x1B\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xF8,P\xF1`\xE0\x1B\x17\x90Ra\x15\x8FV[a\x01P\x82\x82`@Q`$\x01a\x11`\x92\x91\x90a#BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x15\x8FV[a\x01P\x82\x82`@Q`$\x01a\x11\xA5\x92\x91\x90a#cV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c1\x9A\xF33`\xE0\x1B\x17\x90Ra\x15\x8FV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R_\x91\x90\x85\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12!W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12E\x91\x90a aV[\x90Pa\x12[\x84\x84a\x12V\x85\x85a\"@V[a\x15\x98V[PPPPV[`@Qc8\xED\x179`\xE0\x1B\x81R_\x90\x81\x90``\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c8\xED\x179\x90a\x12\x9E\x90\x8A\x90\x8A\x90\x8D\x900\x90\x8C\x90`\x04\x01a#\x8CV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x12\xDBWP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\xD8\x91\x90\x81\x01\x90a#\xFDV[`\x01[a\x13\x8FWa\x12\xE7a$\x88V[\x80c\x08\xC3y\xA0\x03a\x13\x16WPa\x12\xFBa$\xA0V[\x80a\x13\x06WPa\x13\x18V[_\x94P`\x01\x93P\x91Pa\x13\xCF\x90PV[P[=\x80\x80\x15a\x13AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x13FV[``\x91P[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FMostly UniswapV2 pair not found\0`D\x82\x01R`d\x01a\x01\x99V[\x80`\x01\x82Qa\x13\x9E\x91\x90a \rV[\x81Q\x81\x10a\x13\xAEWa\x13\xAEa\x1D0V[` \x02` \x01\x01Q_`@Q\x80` \x01`@R\x80_\x81RP\x94P\x94P\x94PPP[\x95P\x95P\x95\x92PPPV[`@\x80Q`\xA0\x81\x01\x82R\x85\x81R0` \x82\x01R\x80\x82\x01\x83\x90R``\x81\x81\x01\x86\x90R`\x80\x82\x01\x85\x90R\x91Qc\xC0K\x8DY`\xE0\x1B\x81R_\x92\x83\x92\x90\x91\x89\x91\x90`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\xC0K\x8DY\x90a\x146\x90\x84\x90`\x04\x01a%(V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x14pWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x14m\x91\x81\x01\x90a aV[`\x01[a\x14\xB6Wa\x14|a$\x88V[\x80c\x08\xC3y\xA0\x03a\x14\xACWPa\x14\x90a$\xA0V[\x80a\x14\x9BWPa\x14\xAEV[_\x95P`\x01\x94P\x92Pa\x13\xCF\x91PPV[P[=_\x80>=_\xFD[\x80_`@Q\x80` \x01`@R\x80_\x81RP\x95P\x95P\x95PPPPa\x13\xCFV[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`$\x83\x01R_\x91\x90\x85\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\"W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15F\x91\x90a aV[\x90P\x81\x81\x10\x15a\x15\x82W`@Qc\xE5p\x11\x0F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x01\x99V[a\x12[\x84\x84\x84\x84\x03a\x15\x98V[a\x11\x02\x81a\x16KV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x15\xE9\x84\x82a\x16kV[a\x12[W`@\x80Q`\x01`\x01`\xA0\x1B\x03\x85\x16`$\x82\x01R_`D\x80\x83\x01\x91\x90\x91R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\t^\xA7\xB3`\xE0\x1B\x17\x90Ra\x16A\x90\x85\x90a\x17\x0EV[a\x12[\x84\x82a\x17\x0EV[\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[_\x80_\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x16\x86\x91\x90a%\x7FV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x16\xBFW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x16\xC4V[``\x91P[P\x91P\x91P\x81\x80\x15a\x16\xEEWP\x80Q\x15\x80a\x16\xEEWP\x80\x80` \x01\x90Q\x81\x01\x90a\x16\xEE\x91\x90a\"SV[\x80\x15a\x17\x03WP_\x85`\x01`\x01`\xA0\x1B\x03\x16;\x11[\x92PPP[\x92\x91PPV[_a\x17\"`\x01`\x01`\xA0\x1B\x03\x84\x16\x83a\x17tV[\x90P\x80Q_\x14\x15\x80\x15a\x17FWP\x80\x80` \x01\x90Q\x81\x01\x90a\x17D\x91\x90a\"SV[\x15[\x15a\x17oW`@QcRt\xAF\xE7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x01\x99V[PPPV[``a\x10\xB8\x83\x83_\x84_\x80\x85`\x01`\x01`\xA0\x1B\x03\x16\x84\x86`@Qa\x17\x98\x91\x90a%\x7FV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x17\xD2W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x17\xD7V[``\x91P[P\x91P\x91Pa\x17\xE7\x86\x83\x83a\x17\xF1V[\x96\x95PPPPPPV[``\x82a\x18\x06Wa\x18\x01\x82a\x18MV[a\x10\xB8V[\x81Q\x15\x80\x15a\x18\x1DWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x18FW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x01\x99V[P\x80a\x10\xB8V[\x80Q\x15a\x18]W\x80Q\x80\x82` \x01\xFD[`@Qc\n\x12\xF5!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_` \x82\x84\x03\x12\x15a\x18\x86W_\x80\xFD[P5\x91\x90PV[_\x80\x83`\x1F\x84\x01\x12a\x18\x9DW_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xB3W_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x18\xCDW_\x80\xFD[\x92P\x92\x90PV[\x80\x15\x15\x81\x14a\x11\x02W_\x80\xFD[_\x80_\x80``\x85\x87\x03\x12\x15a\x18\xF4W_\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\tW_\x80\xFD[a\x19\x15\x87\x82\x88\x01a\x18\x8DV[\x90\x95P\x93PP` \x85\x015a\x19)\x81a\x18\xD4V[\x91P`@\x85\x015a\x199\x81a\x18\xD4V[\x93\x96\x92\x95P\x90\x93PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x19}Wa\x19}a\x19DV[`@RPPV[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x19\xA6Wa\x19\xA6a\x19DV[`@R\x90V[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x19\xC4Wa\x19\xC4a\x19DV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11\x02W_\x80\xFD[_\x82`\x1F\x83\x01\x12a\x19\xF1W_\x80\xFD[\x815` a\x19\xFE\x82a\x19\xACV[`@Qa\x1A\x0B\x82\x82a\x19XV[\x80\x91P\x83\x81R` \x81\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x1A.W_\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1AJW\x805\x83R\x91\x83\x01\x91\x83\x01a\x1A3V[P\x96\x95PPPPPPV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1AmWa\x1Ama\x19DV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x1A\x8AW_\x80\xFD[\x815a\x1A\x95\x81a\x1AUV[`@Qa\x1A\xA2\x82\x82a\x19XV[\x82\x81R\x85` \x84\x87\x01\x01\x11\x15a\x1A\xB6W_\x80\xFD[\x82` \x86\x01` \x83\x017_\x92\x81\x01` \x01\x92\x90\x92RP\x93\x92PPPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x1A\xE6W_\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1A\xFCW_\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x1B\x0FW_\x80\xFD[\x815` a\x1B\x1C\x82a\x19\xACV[`@Qa\x1B)\x82\x82a\x19XV[\x83\x81R`\x05\x93\x90\x93\x1B\x85\x01\x82\x01\x92\x82\x81\x01\x91P\x8B\x84\x11\x15a\x1BHW_\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x1BoW\x855a\x1B`\x81a\x19\xCEV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x1BMV[\x98PP\x88\x015\x92PP\x80\x82\x11\x15a\x1B\x84W_\x80\xFD[a\x1B\x90\x88\x83\x89\x01a\x19\xE2V[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a\x1B\xA5W_\x80\xFD[a\x1B\xB1\x88\x83\x89\x01a\x19\xE2V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x1B\xC6W_\x80\xFD[Pa\x1B\xD3\x87\x82\x88\x01a\x1A{V[\x91PP\x92\x95\x91\x94P\x92PV[_\x80_\x80`@\x85\x87\x03\x12\x15a\x1B\xF2W_\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1C\x08W_\x80\xFD[a\x1C\x14\x88\x83\x89\x01a\x18\x8DV[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a\x1C,W_\x80\xFD[Pa\x1C9\x87\x82\x88\x01a\x18\x8DV[\x95\x98\x94\x97P\x95PPPPV[_` \x80\x83\x85\x03\x12\x15a\x1CVW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1CkW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x1C{W_\x80\xFD[\x805a\x1C\x86\x81a\x19\xACV[`@Qa\x1C\x93\x82\x82a\x19XV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\x1C\xB2W_\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x1C\xD9W\x835a\x1C\xCA\x81a\x19\xCEV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\x1C\xB7V[\x97\x96PPPPPPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a\x1D\x13W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x1C\xF7V[P\x94\x95\x94PPPPPV[` \x81R_a\x10\xB8` \x83\x01\x84a\x1C\xE4V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`\xDE\x19\x836\x03\x01\x81\x12a\x1DXW_\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1DrW_\x80\xFD[\x815a\x10\xB8\x81a\x19\xCEV[`\x02\x81\x10a\x11\x02W_\x80\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x02\x81\x10a\x1D\xB9WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1D\xD2W_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xF0W_\x80\xFD[\x806\x03\x82\x13\x15a\x18\xCDW_\x80\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[``\x80\x82R\x81\x81\x01\x85\x90R_\x90`\x80\x80\x84\x01`\x05\x88\x90\x1B\x85\x01\x82\x01\x89\x85[\x8A\x81\x10\x15a\x1F\rW\x87\x83\x03`\x7F\x19\x01\x84R\x8156\x8D\x90\x03`\xDE\x19\x01\x81\x12a\x1EiW_\x80\xFD[\x8C\x01`\xE0\x815a\x1Ex\x81a\x1D}V[a\x1E\x82\x86\x82a\x1D\x9DV[P` \x80\x83\x015a\x1E\x92\x81a\x19\xCEV[`\x01`\x01`\xA0\x1B\x03\x16\x86\x82\x01R`@\x83\x81\x015a\x1E\xAE\x81a\x19\xCEV[`\x01`\x01`\xA0\x1B\x03\x16\x90\x87\x01Ra\x1E\xC7\x83\x8A\x01\x84a\x1D\xBDV[\x83\x8B\x89\x01Ra\x1E\xD9\x84\x89\x01\x82\x84a\x1D\xFEV[\x85\x8B\x015\x89\x8C\x01R`\xA0\x80\x87\x015\x90\x8A\x01R`\xC0\x95\x86\x015\x95\x90\x98\x01\x94\x90\x94RP\x95\x86\x01\x95\x93\x90\x93\x01\x92PP`\x01\x01a\x1EDV[PP\x87\x15\x15` \x87\x01R\x93Pa\x1F\"\x92PPPV[\x82\x15\x15`@\x83\x01R[\x95\x94PPPPPV[_[\x83\x81\x10\x15a\x1FNW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1F6V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x1Fm\x81` \x86\x01` \x86\x01a\x1F4V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R`\x80` \x80\x84\x01\x82\x90R\x86Q\x91\x84\x01\x82\x90R_\x92\x87\x82\x01\x92\x90\x91\x90`\xA0\x86\x01\x90\x85[\x81\x81\x10\x15a\x1F\xCEW\x85Q\x85\x16\x83R\x94\x83\x01\x94\x91\x83\x01\x91`\x01\x01a\x1F\xB0V[PP\x85\x81\x03`@\x87\x01Ra\x1F\xE2\x81\x89a\x1C\xE4V[\x93PPPP\x82\x81\x03``\x84\x01Ra\x1C\xD9\x81\x85a\x1FVV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x17\x08Wa\x17\x08a\x1F\xF9V[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a ?Wa ?a\x1F\xF9V[P\x92\x91PPV[_` \x82\x84\x03\x12\x15a VW_\x80\xFD[\x81Qa\x10\xB8\x81a\x19\xCEV[_` \x82\x84\x03\x12\x15a qW_\x80\xFD[PQ\x91\x90PV[\x80Qa\n\xC7\x81a\x1D}V[\x80Qa\n\xC7\x81a\x19\xCEV[_\x82`\x1F\x83\x01\x12a \x9DW_\x80\xFD[\x81Qa \xA8\x81a\x1AUV[`@Qa \xB5\x82\x82a\x19XV[\x82\x81R\x85` \x84\x87\x01\x01\x11\x15a \xC9W_\x80\xFD[a\x1F+\x83` \x83\x01` \x88\x01a\x1F4V[\x80Qa\n\xC7\x81a\x18\xD4V[_\x80_``\x84\x86\x03\x12\x15a \xF7W_\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a!\rW_\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a! W_\x80\xFD[\x81Q` a!-\x82a\x19\xACV[`@Qa!:\x82\x82a\x19XV[\x83\x81R`\x05\x93\x90\x93\x1B\x85\x01\x82\x01\x92\x82\x81\x01\x91P\x8A\x84\x11\x15a!YW_\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\"\x15W\x80Q\x86\x81\x11\x15a!sW_\x80\xFD[\x87\x01`\xE0\x81\x8E\x03`\x1F\x19\x01\x12\x15a!\x88W_\x80\xFD[a!\x90a\x19\x84V[a!\x9B\x86\x83\x01a xV[\x81Ra!\xA9`@\x83\x01a \x83V[\x86\x82\x01Ra!\xB9``\x83\x01a \x83V[`@\x82\x01R`\x80\x82\x01Q\x88\x81\x11\x15a!\xCFW_\x80\xFD[a!\xDD\x8F\x88\x83\x86\x01\x01a \x8EV[``\x83\x01RP`\xA0\x82\x81\x01Q`\x80\x83\x01R`\xC0\x80\x84\x01Q\x91\x83\x01\x91\x90\x91R`\xE0\x90\x92\x01Q\x91\x81\x01\x91\x90\x91R\x83R\x91\x83\x01\x91\x83\x01a!]V[P\x97Pa\"%\x90P\x88\x82\x01a \xDAV[\x95PPPPPa\"7`@\x85\x01a \xDAV[\x90P\x92P\x92P\x92V[\x80\x82\x01\x80\x82\x11\x15a\x17\x08Wa\x17\x08a\x1F\xF9V[_` \x82\x84\x03\x12\x15a\"cW_\x80\xFD[\x81Qa\x10\xB8\x81a\x18\xD4V[_` \x80\x83\x85\x03\x12\x15a\"\x7FW_\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x94W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\"\xA4W_\x80\xFD[\x80Qa\"\xAF\x81a\x19\xACV[`@Qa\"\xBC\x82\x82a\x19XV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a\"\xDBW_\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x1C\xD9W\x83Qa\"\xF3\x81a\x19\xCEV[\x82R\x92\x84\x01\x92\x90\x84\x01\x90a\"\xE0V[` \x81\x01a\x17\x08\x82\x84a\x1D\x9DV[\x82\x81R`@` \x82\x01R_a#(`@\x83\x01\x84a\x1FVV[\x94\x93PPPPV[` \x81R_a\x10\xB8` \x83\x01\x84a\x1FVV[`@\x81R_a#T`@\x83\x01\x85a\x1FVV[\x90P\x82` \x83\x01R\x93\x92PPPV[`@\x81R_a#u`@\x83\x01\x85a\x1FVV[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[_`\xA0\x82\x01\x87\x83R` \x87` \x85\x01R`\xA0`@\x85\x01R\x81\x87Q\x80\x84R`\xC0\x86\x01\x91P` \x89\x01\x93P_[\x81\x81\x10\x15a#\xDCW\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a#\xB7V[PP`\x01`\x01`\xA0\x1B\x03\x96\x90\x96\x16``\x85\x01RPPP`\x80\x01R\x93\x92PPPV[_` \x80\x83\x85\x03\x12\x15a$\x0EW_\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a$#W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a$3W_\x80\xFD[\x80Qa$>\x81a\x19\xACV[`@Qa$K\x82\x82a\x19XV[\x82\x81R`\x05\x92\x90\x92\x1B\x83\x01\x84\x01\x91\x84\x81\x01\x91P\x87\x83\x11\x15a$jW_\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a\x1C\xD9W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a$oV[_`\x03=\x11\x15a\x01\x1BW`\x04_\x80>P_Q`\xE0\x1C\x90V[_`D=\x10\x15a$\xADW\x90V[`@Q`\x03\x19=\x81\x01`\x04\x83>\x81Q=`\x01`\x01`@\x1B\x03\x81`$\x84\x01\x11\x81\x84\x11\x17\x15a$\xDCWPPPPP\x90V[\x82\x85\x01\x91P\x81Q\x81\x81\x11\x15a$\xF4WPPPPPP\x90V[\x84=\x87\x01\x01` \x82\x85\x01\x01\x11\x15a%\x0EWPPPPPP\x90V[a%\x1D` \x82\x86\x01\x01\x87a\x19XV[P\x90\x95\x94PPPPPV[` \x81R_\x82Q`\xA0` \x84\x01Ra%C`\xC0\x84\x01\x82a\x1FVV[\x90P`\x01\x80`\xA0\x1B\x03` \x85\x01Q\x16`@\x84\x01R`@\x84\x01Q``\x84\x01R``\x84\x01Q`\x80\x84\x01R`\x80\x84\x01Q`\xA0\x84\x01R\x80\x91PP\x92\x91PPV[_\x82Qa\x1DX\x81\x84` \x87\x01a\x1F4V\xFE\xA2dipfsX\"\x12 \xACK\xBC\\\x1D\x84\xC4\xE2c\x14U\xCE\xB3\xF1\xDB[\xF5\x90\xBF\xC0\x17\xA4\xE6\xD4_\xFDwB\xDA\xEB\x19\tdsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static BALANCERFLASHLOANRECIPIENTABI_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct BalancerFlashLoanRecipientAbi<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for BalancerFlashLoanRecipientAbi<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BalancerFlashLoanRecipientAbi<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BalancerFlashLoanRecipientAbi<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BalancerFlashLoanRecipientAbi<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(BalancerFlashLoanRecipientAbi))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BalancerFlashLoanRecipientAbi<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    BALANCERFLASHLOANRECIPIENTABI_ABI.clone(),
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
                BALANCERFLASHLOANRECIPIENTABI_ABI.clone(),
                BALANCERFLASHLOANRECIPIENTABI_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getBalance` (0x12065fe0) function
        pub fn get_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([18, 6, 95, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getFlashLoanFeePercentage` (0xd877845c) function
        pub fn get_flash_loan_fee_percentage(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([216, 119, 132, 92], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLoanThenMultiSwap` (0x83c10d97) function
        pub fn get_loan_then_multi_swap(
            &self,
            swaps: ::std::vec::Vec<OneSwapInfo>,
            chain_swaps: bool,
            return_output: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([131, 193, 13, 151], (swaps, chain_swaps, return_output))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenBalance` (0xff638962) function
        pub fn get_token_balance(
            &self,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([255, 99, 137, 98], tokens)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `receiveFlashLoan` (0xf04f2707) function
        pub fn receive_flash_loan(
            &self,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            fee_amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            user_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 79, 39, 7], (tokens, amounts, fee_amounts, user_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x2e1a7d4d) function
        pub fn withdraw(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawToken` (0xf3c9c2c2) function
        pub fn withdraw_token(
            &self,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 201, 194, 194], (tokens, amounts))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for BalancerFlashLoanRecipientAbi<M> {
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
    ///Custom Error type `InsufficientFundsToRepayLoanError` with signature `InsufficientFundsToRepayLoanError(address,uint256,uint256)` and selector `0xb1e0b7cb`
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
        name = "InsufficientFundsToRepayLoanError",
        abi = "InsufficientFundsToRepayLoanError(address,uint256,uint256)"
    )]
    pub struct InsufficientFundsToRepayLoanError {
        pub token: ::ethers::core::types::Address,
        pub amount_out: ::ethers::core::types::U256,
        pub amount_to_payback: ::ethers::core::types::U256,
    }
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
    ///Custom Error type `SafeERC20FailedDecreaseAllowance` with signature `SafeERC20FailedDecreaseAllowance(address,uint256,uint256)` and selector `0xe570110f`
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
        name = "SafeERC20FailedDecreaseAllowance",
        abi = "SafeERC20FailedDecreaseAllowance(address,uint256,uint256)"
    )]
    pub struct SafeERC20FailedDecreaseAllowance {
        pub spender: ::ethers::core::types::Address,
        pub current_allowance: ::ethers::core::types::U256,
        pub requested_decrease: ::ethers::core::types::U256,
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
    pub enum BalancerFlashLoanRecipientAbiErrors {
        AddressEmptyCode(AddressEmptyCode),
        AddressInsufficientBalance(AddressInsufficientBalance),
        FailedInnerCall(FailedInnerCall),
        InsufficientFundsToRepayLoanError(InsufficientFundsToRepayLoanError),
        MultiSwapError(MultiSwapError),
        NotSupportedAmmProtocolError(NotSupportedAmmProtocolError),
        SafeERC20FailedDecreaseAllowance(SafeERC20FailedDecreaseAllowance),
        SafeERC20FailedOperation(SafeERC20FailedOperation),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for BalancerFlashLoanRecipientAbiErrors {
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
            if let Ok(decoded) = <InsufficientFundsToRepayLoanError as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InsufficientFundsToRepayLoanError(decoded));
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
            if let Ok(decoded) = <SafeERC20FailedDecreaseAllowance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedDecreaseAllowance(decoded));
            }
            if let Ok(decoded) = <SafeERC20FailedOperation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeERC20FailedOperation(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BalancerFlashLoanRecipientAbiErrors {
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
                Self::InsufficientFundsToRepayLoanError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MultiSwapError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotSupportedAmmProtocolError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedDecreaseAllowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for BalancerFlashLoanRecipientAbiErrors {
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
                    == <InsufficientFundsToRepayLoanError as ::ethers::contract::EthError>::selector() => {
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
                    == <SafeERC20FailedDecreaseAllowance as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for BalancerFlashLoanRecipientAbiErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddressInsufficientBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FailedInnerCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientFundsToRepayLoanError(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MultiSwapError(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotSupportedAmmProtocolError(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeERC20FailedDecreaseAllowance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SafeERC20FailedOperation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for BalancerFlashLoanRecipientAbiErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode>
    for BalancerFlashLoanRecipientAbiErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<AddressInsufficientBalance>
    for BalancerFlashLoanRecipientAbiErrors {
        fn from(value: AddressInsufficientBalance) -> Self {
            Self::AddressInsufficientBalance(value)
        }
    }
    impl ::core::convert::From<FailedInnerCall> for BalancerFlashLoanRecipientAbiErrors {
        fn from(value: FailedInnerCall) -> Self {
            Self::FailedInnerCall(value)
        }
    }
    impl ::core::convert::From<InsufficientFundsToRepayLoanError>
    for BalancerFlashLoanRecipientAbiErrors {
        fn from(value: InsufficientFundsToRepayLoanError) -> Self {
            Self::InsufficientFundsToRepayLoanError(value)
        }
    }
    impl ::core::convert::From<MultiSwapError> for BalancerFlashLoanRecipientAbiErrors {
        fn from(value: MultiSwapError) -> Self {
            Self::MultiSwapError(value)
        }
    }
    impl ::core::convert::From<NotSupportedAmmProtocolError>
    for BalancerFlashLoanRecipientAbiErrors {
        fn from(value: NotSupportedAmmProtocolError) -> Self {
            Self::NotSupportedAmmProtocolError(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedDecreaseAllowance>
    for BalancerFlashLoanRecipientAbiErrors {
        fn from(value: SafeERC20FailedDecreaseAllowance) -> Self {
            Self::SafeERC20FailedDecreaseAllowance(value)
        }
    }
    impl ::core::convert::From<SafeERC20FailedOperation>
    for BalancerFlashLoanRecipientAbiErrors {
        fn from(value: SafeERC20FailedOperation) -> Self {
            Self::SafeERC20FailedOperation(value)
        }
    }
    ///Container type for all input parameters for the `getBalance` function with signature `getBalance()` and selector `0x12065fe0`
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
    #[ethcall(name = "getBalance", abi = "getBalance()")]
    pub struct GetBalanceCall;
    ///Container type for all input parameters for the `getFlashLoanFeePercentage` function with signature `getFlashLoanFeePercentage()` and selector `0xd877845c`
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
    #[ethcall(name = "getFlashLoanFeePercentage", abi = "getFlashLoanFeePercentage()")]
    pub struct GetFlashLoanFeePercentageCall;
    ///Container type for all input parameters for the `getLoanThenMultiSwap` function with signature `getLoanThenMultiSwap((uint8,address,address,bytes,uint256,uint256,uint256)[],bool,bool)` and selector `0x83c10d97`
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
        name = "getLoanThenMultiSwap",
        abi = "getLoanThenMultiSwap((uint8,address,address,bytes,uint256,uint256,uint256)[],bool,bool)"
    )]
    pub struct GetLoanThenMultiSwapCall {
        pub swaps: ::std::vec::Vec<OneSwapInfo>,
        pub chain_swaps: bool,
        pub return_output: bool,
    }
    ///Container type for all input parameters for the `getTokenBalance` function with signature `getTokenBalance(address[])` and selector `0xff638962`
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
    #[ethcall(name = "getTokenBalance", abi = "getTokenBalance(address[])")]
    pub struct GetTokenBalanceCall {
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `receiveFlashLoan` function with signature `receiveFlashLoan(address[],uint256[],uint256[],bytes)` and selector `0xf04f2707`
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
        name = "receiveFlashLoan",
        abi = "receiveFlashLoan(address[],uint256[],uint256[],bytes)"
    )]
    pub struct ReceiveFlashLoanCall {
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub fee_amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub user_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256)` and selector `0x2e1a7d4d`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawToken` function with signature `withdrawToken(address[],uint256[])` and selector `0xf3c9c2c2`
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
    #[ethcall(name = "withdrawToken", abi = "withdrawToken(address[],uint256[])")]
    pub struct WithdrawTokenCall {
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum BalancerFlashLoanRecipientAbiCalls {
        GetBalance(GetBalanceCall),
        GetFlashLoanFeePercentage(GetFlashLoanFeePercentageCall),
        GetLoanThenMultiSwap(GetLoanThenMultiSwapCall),
        GetTokenBalance(GetTokenBalanceCall),
        ReceiveFlashLoan(ReceiveFlashLoanCall),
        Withdraw(WithdrawCall),
        WithdrawToken(WithdrawTokenCall),
    }
    impl ::ethers::core::abi::AbiDecode for BalancerFlashLoanRecipientAbiCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBalance(decoded));
            }
            if let Ok(decoded) = <GetFlashLoanFeePercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFlashLoanFeePercentage(decoded));
            }
            if let Ok(decoded) = <GetLoanThenMultiSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLoanThenMultiSwap(decoded));
            }
            if let Ok(decoded) = <GetTokenBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTokenBalance(decoded));
            }
            if let Ok(decoded) = <ReceiveFlashLoanCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReceiveFlashLoan(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            if let Ok(decoded) = <WithdrawTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawToken(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for BalancerFlashLoanRecipientAbiCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetFlashLoanFeePercentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLoanThenMultiSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReceiveFlashLoan(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for BalancerFlashLoanRecipientAbiCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetFlashLoanFeePercentage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetLoanThenMultiSwap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetTokenBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReceiveFlashLoan(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawToken(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetBalanceCall> for BalancerFlashLoanRecipientAbiCalls {
        fn from(value: GetBalanceCall) -> Self {
            Self::GetBalance(value)
        }
    }
    impl ::core::convert::From<GetFlashLoanFeePercentageCall>
    for BalancerFlashLoanRecipientAbiCalls {
        fn from(value: GetFlashLoanFeePercentageCall) -> Self {
            Self::GetFlashLoanFeePercentage(value)
        }
    }
    impl ::core::convert::From<GetLoanThenMultiSwapCall>
    for BalancerFlashLoanRecipientAbiCalls {
        fn from(value: GetLoanThenMultiSwapCall) -> Self {
            Self::GetLoanThenMultiSwap(value)
        }
    }
    impl ::core::convert::From<GetTokenBalanceCall>
    for BalancerFlashLoanRecipientAbiCalls {
        fn from(value: GetTokenBalanceCall) -> Self {
            Self::GetTokenBalance(value)
        }
    }
    impl ::core::convert::From<ReceiveFlashLoanCall>
    for BalancerFlashLoanRecipientAbiCalls {
        fn from(value: ReceiveFlashLoanCall) -> Self {
            Self::ReceiveFlashLoan(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for BalancerFlashLoanRecipientAbiCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    impl ::core::convert::From<WithdrawTokenCall>
    for BalancerFlashLoanRecipientAbiCalls {
        fn from(value: WithdrawTokenCall) -> Self {
            Self::WithdrawToken(value)
        }
    }
    ///Container type for all return fields from the `getBalance` function with signature `getBalance()` and selector `0x12065fe0`
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
    pub struct GetBalanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getFlashLoanFeePercentage` function with signature `getFlashLoanFeePercentage()` and selector `0xd877845c`
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
    pub struct GetFlashLoanFeePercentageReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getLoanThenMultiSwap` function with signature `getLoanThenMultiSwap((uint8,address,address,bytes,uint256,uint256,uint256)[],bool,bool)` and selector `0x83c10d97`
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
    pub struct GetLoanThenMultiSwapReturn {
        pub profit: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `getTokenBalance` function with signature `getTokenBalance(address[])` and selector `0xff638962`
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
    pub struct GetTokenBalanceReturn {
        pub balances: ::std::vec::Vec<::ethers::core::types::U256>,
    }
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
