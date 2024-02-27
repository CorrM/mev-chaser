pub use fast_lane_auction_handler_abi::*;

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
pub mod fast_lane_auction_handler_abi {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("clearValidatorPayee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "clearValidatorPayee",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("collectFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("collectFees"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fastBidWrapper"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fastBidWrapper"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgSender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fastPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("searcherToAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("searcherCallData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fulfilledAuctionsMap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "fulfilledAuctionsMap",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fulfilledPGAMap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fulfilledPGAMap"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lowestGasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lowestFastPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lowestTotalPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getValidatorBalance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getValidatorBalance",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_validatorBalance"),
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
                    ::std::borrow::ToOwned::to_owned("getValidatorBlockOfLastWithdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getValidatorBlockOfLastWithdraw",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_blockNumber"),
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
                    ::std::borrow::ToOwned::to_owned("getValidatorPayee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getValidatorPayee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_payee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getValidatorRecipient"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getValidatorRecipient",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_recipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isPayeeTimeLocked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isPayeeTimeLocked"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_isTimeLocked"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isValidPayee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isValidPayee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_payee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_valid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("payValidatorCustom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("payValidatorCustom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("paymentProcessor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("customAllocation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("payValidatorFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("payValidatorFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_payor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("payeeMap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("payeeMap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("simulateFlashBid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateFlashBid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bidAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oppTxHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("searcherToAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("searcherCallData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitFastBid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("submitFastBid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fastGasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("searcherToAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("searcherCallData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitFlashBid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("submitFlashBid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bidAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oppTxHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("searcherToAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("searcherCallData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitFlashBidWithRefund"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "submitFlashBidWithRefund",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bidAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("oppTxHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("searcherToAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("searcherCallData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("syncStuckNativeToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "syncStuckNativeToken",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updateValidatorPayee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateValidatorPayee",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_payee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("updateValidatorRefundShare"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "updateValidatorRefundShare",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("refundShare"),
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
                    ::std::borrow::ToOwned::to_owned("validatorsBalanceMap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "validatorsBalanceMap",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validatorsRefundShareMap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "validatorsRefundShareMap",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validatorsTotal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validatorsTotal"),
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
                    ::std::borrow::ToOwned::to_owned("withdrawStuckERC20"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdrawStuckERC20"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CustomPaymentProcessorPaid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "CustomPaymentProcessorPaid",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("paymentProcessor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("totalAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("customAllocation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("startBlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("endBlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayFastBid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RelayFastBid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("bidAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "searcherContractAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayFeeCollected"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RelayFeeCollected"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payor"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayFlashBid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("RelayFlashBid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oppTxHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("bidAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountPaid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "searcherContractAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayFlashBidWithRefund"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayFlashBidWithRefund",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oppTxHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("bidAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountPaid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "searcherContractAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("refundedAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("refundAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayInvestigateOutcome"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayInvestigateOutcome",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("blockNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("existingBidAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newBidAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("existingGasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newGasPrice"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayProcessingPaidValidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayProcessingPaidValidator",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("validatorPayment"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("initiator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelaySimulatedFlashBid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelaySimulatedFlashBid",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oppTxHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "searcherContractAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayValidatorPayeeUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayValidatorPayeeUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("validator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("initiator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayWithdrawStuckERC20"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayWithdrawStuckERC20",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayWithdrawStuckNativeToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayWithdrawStuckNativeToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("receiver"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("RelayAuctionBidReceivedLate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayAuctionBidReceivedLate",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayAuctionInvalidBid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayAuctionInvalidBid",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayAuctionSearcherNotWinner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayAuctionSearcherNotWinner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("current"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("existing"),
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
                    ::std::borrow::ToOwned::to_owned("RelayCannotBeSelf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("RelayCannotBeSelf"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayCannotBeZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("RelayCannotBeZero"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayImmutableBlockAuthorRate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayImmutableBlockAuthorRate",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayInvalidSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("RelayInvalidSender"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayMustBeSelf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("RelayMustBeSelf"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayNotActiveValidator"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayNotActiveValidator",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayNotRepaid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("RelayNotRepaid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bidAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actualAmount"),
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
                    ::std::borrow::ToOwned::to_owned("RelayPayeeIsTimelocked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayPayeeIsTimelocked",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayPayeeUpdateInvalid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayPayeeUpdateInvalid",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayPermissionSenderNotOrigin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayPermissionSenderNotOrigin",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayProcessorCannotBeZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayProcessorCannotBeZero",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelaySearcherWrongParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelaySearcherWrongParams",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelaySimulatedNotRepaid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelaySimulatedNotRepaid",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("bidAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("actualAmount"),
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
                    ::std::borrow::ToOwned::to_owned(
                        "RelayValidatorNotAcceptingRefundBids",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RelayValidatorNotAcceptingRefundBids",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RelayValueIsZero"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("RelayValueIsZero"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FASTLANEAUCTIONHANDLERABI_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct FastLaneAuctionHandlerAbi<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FastLaneAuctionHandlerAbi<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FastLaneAuctionHandlerAbi<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FastLaneAuctionHandlerAbi<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FastLaneAuctionHandlerAbi<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FastLaneAuctionHandlerAbi))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FastLaneAuctionHandlerAbi<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FASTLANEAUCTIONHANDLERABI_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `clearValidatorPayee` (0x43b1f483) function
        pub fn clear_validator_payee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([67, 177, 244, 131], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `collectFees` (0xc8796572) function
        pub fn collect_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([200, 121, 101, 114], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fastBidWrapper` (0xb4f042b8) function
        pub fn fast_bid_wrapper(
            &self,
            msg_sender: ::ethers::core::types::Address,
            fast_price: ::ethers::core::types::U256,
            searcher_to_address: ::ethers::core::types::Address,
            searcher_call_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [180, 240, 66, 184],
                    (msg_sender, fast_price, searcher_to_address, searcher_call_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fulfilledAuctionsMap` (0x59f5764f) function
        pub fn fulfilled_auctions_map(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([89, 245, 118, 79], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fulfilledPGAMap` (0xd67a7384) function
        pub fn fulfilled_pga_map(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, u64, u64)> {
            self.0
                .method_hash([214, 122, 115, 132], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getValidatorBalance` (0x19d16629) function
        pub fn get_validator_balance(
            &self,
            validator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([25, 209, 102, 41], validator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getValidatorBlockOfLastWithdraw` (0xd7cfc841) function
        pub fn get_validator_block_of_last_withdraw(
            &self,
            validator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([215, 207, 200, 65], validator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getValidatorPayee` (0x83ebbe4b) function
        pub fn get_validator_payee(
            &self,
            validator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([131, 235, 190, 75], validator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getValidatorRecipient` (0x78313b36) function
        pub fn get_validator_recipient(
            &self,
            validator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([120, 49, 59, 54], validator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPayeeTimeLocked` (0xd3550dfa) function
        pub fn is_payee_time_locked(
            &self,
            validator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([211, 85, 13, 250], validator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isValidPayee` (0xaf6ccc0b) function
        pub fn is_valid_payee(
            &self,
            validator: ::ethers::core::types::Address,
            payee: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([175, 108, 204, 11], (validator, payee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payValidatorCustom` (0xc0a2ab03) function
        pub fn pay_validator_custom(
            &self,
            payment_processor: ::ethers::core::types::Address,
            custom_allocation: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [192, 162, 171, 3],
                    (payment_processor, custom_allocation, data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payValidatorFee` (0x43909ed0) function
        pub fn pay_validator_fee(
            &self,
            payor: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([67, 144, 158, 208], payor)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `payeeMap` (0x69660bd6) function
        pub fn payee_map(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([105, 102, 11, 214], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateFlashBid` (0x10baf2bf) function
        pub fn simulate_flash_bid(
            &self,
            bid_amount: ::ethers::core::types::U256,
            opp_tx_hash: [u8; 32],
            searcher_to_address: ::ethers::core::types::Address,
            searcher_call_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [16, 186, 242, 191],
                    (bid_amount, opp_tx_hash, searcher_to_address, searcher_call_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitFastBid` (0xcda5b8ac) function
        pub fn submit_fast_bid(
            &self,
            fast_gas_price: ::ethers::core::types::U256,
            searcher_to_address: ::ethers::core::types::Address,
            searcher_call_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [205, 165, 184, 172],
                    (fast_gas_price, searcher_to_address, searcher_call_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitFlashBid` (0xeac046f8) function
        pub fn submit_flash_bid(
            &self,
            bid_amount: ::ethers::core::types::U256,
            opp_tx_hash: [u8; 32],
            searcher_to_address: ::ethers::core::types::Address,
            searcher_call_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [234, 192, 70, 248],
                    (bid_amount, opp_tx_hash, searcher_to_address, searcher_call_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitFlashBidWithRefund` (0x2f10b549) function
        pub fn submit_flash_bid_with_refund(
            &self,
            bid_amount: ::ethers::core::types::U256,
            opp_tx_hash: [u8; 32],
            refund_address: ::ethers::core::types::Address,
            searcher_to_address: ::ethers::core::types::Address,
            searcher_call_data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [47, 16, 181, 73],
                    (
                        bid_amount,
                        opp_tx_hash,
                        refund_address,
                        searcher_to_address,
                        searcher_call_data,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `syncStuckNativeToken` (0x8533f66e) function
        pub fn sync_stuck_native_token(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 51, 246, 110], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateValidatorPayee` (0xc64dfa0a) function
        pub fn update_validator_payee(
            &self,
            payee: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 77, 250, 10], payee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateValidatorRefundShare` (0x9467ab16) function
        pub fn update_validator_refund_share(
            &self,
            refund_share: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 103, 171, 22], refund_share)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validatorsBalanceMap` (0x8cfdbf5f) function
        pub fn validators_balance_map(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([140, 253, 191, 95], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validatorsRefundShareMap` (0x4c3175b1) function
        pub fn validators_refund_share_map(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([76, 49, 117, 177], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validatorsTotal` (0x03eaca08) function
        pub fn validators_total(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([3, 234, 202, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawStuckERC20` (0x3963510b) function
        pub fn withdraw_stuck_erc20(
            &self,
            token_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 99, 81, 11], token_address)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CustomPaymentProcessorPaid` event
        pub fn custom_payment_processor_paid_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CustomPaymentProcessorPaidFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RelayFastBid` event
        pub fn relay_fast_bid_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RelayFastBidFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RelayFeeCollected` event
        pub fn relay_fee_collected_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RelayFeeCollectedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RelayFlashBid` event
        pub fn relay_flash_bid_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RelayFlashBidFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RelayFlashBidWithRefund` event
        pub fn relay_flash_bid_with_refund_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RelayFlashBidWithRefundFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RelayInvestigateOutcome` event
        pub fn relay_investigate_outcome_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RelayInvestigateOutcomeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RelayProcessingPaidValidator` event
        pub fn relay_processing_paid_validator_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RelayProcessingPaidValidatorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RelaySimulatedFlashBid` event
        pub fn relay_simulated_flash_bid_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RelaySimulatedFlashBidFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RelayValidatorPayeeUpdated` event
        pub fn relay_validator_payee_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RelayValidatorPayeeUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RelayWithdrawStuckERC20` event
        pub fn relay_withdraw_stuck_erc20_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RelayWithdrawStuckERC20Filter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RelayWithdrawStuckNativeToken` event
        pub fn relay_withdraw_stuck_native_token_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RelayWithdrawStuckNativeTokenFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FastLaneAuctionHandlerAbiEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for FastLaneAuctionHandlerAbi<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `RelayAuctionBidReceivedLate` with signature `RelayAuctionBidReceivedLate()` and selector `0xb61e767e`
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
        name = "RelayAuctionBidReceivedLate",
        abi = "RelayAuctionBidReceivedLate()"
    )]
    pub struct RelayAuctionBidReceivedLate;
    ///Custom Error type `RelayAuctionInvalidBid` with signature `RelayAuctionInvalidBid()` and selector `0xa51c0e05`
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
    #[etherror(name = "RelayAuctionInvalidBid", abi = "RelayAuctionInvalidBid()")]
    pub struct RelayAuctionInvalidBid;
    ///Custom Error type `RelayAuctionSearcherNotWinner` with signature `RelayAuctionSearcherNotWinner(uint256,uint256)` and selector `0x5db6f7d9`
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
        name = "RelayAuctionSearcherNotWinner",
        abi = "RelayAuctionSearcherNotWinner(uint256,uint256)"
    )]
    pub struct RelayAuctionSearcherNotWinner {
        pub current: ::ethers::core::types::U256,
        pub existing: ::ethers::core::types::U256,
    }
    ///Custom Error type `RelayCannotBeSelf` with signature `RelayCannotBeSelf()` and selector `0x6a64f641`
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
    #[etherror(name = "RelayCannotBeSelf", abi = "RelayCannotBeSelf()")]
    pub struct RelayCannotBeSelf;
    ///Custom Error type `RelayCannotBeZero` with signature `RelayCannotBeZero()` and selector `0x3c9cfe50`
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
    #[etherror(name = "RelayCannotBeZero", abi = "RelayCannotBeZero()")]
    pub struct RelayCannotBeZero;
    ///Custom Error type `RelayImmutableBlockAuthorRate` with signature `RelayImmutableBlockAuthorRate()` and selector `0xe9271574`
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
        name = "RelayImmutableBlockAuthorRate",
        abi = "RelayImmutableBlockAuthorRate()"
    )]
    pub struct RelayImmutableBlockAuthorRate;
    ///Custom Error type `RelayInvalidSender` with signature `RelayInvalidSender()` and selector `0x3e82c9f4`
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
    #[etherror(name = "RelayInvalidSender", abi = "RelayInvalidSender()")]
    pub struct RelayInvalidSender;
    ///Custom Error type `RelayMustBeSelf` with signature `RelayMustBeSelf()` and selector `0x3ee08eb4`
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
    #[etherror(name = "RelayMustBeSelf", abi = "RelayMustBeSelf()")]
    pub struct RelayMustBeSelf;
    ///Custom Error type `RelayNotActiveValidator` with signature `RelayNotActiveValidator()` and selector `0x68a251a0`
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
    #[etherror(name = "RelayNotActiveValidator", abi = "RelayNotActiveValidator()")]
    pub struct RelayNotActiveValidator;
    ///Custom Error type `RelayNotRepaid` with signature `RelayNotRepaid(uint256,uint256)` and selector `0x53dc88d9`
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
    #[etherror(name = "RelayNotRepaid", abi = "RelayNotRepaid(uint256,uint256)")]
    pub struct RelayNotRepaid {
        pub bid_amount: ::ethers::core::types::U256,
        pub actual_amount: ::ethers::core::types::U256,
    }
    ///Custom Error type `RelayPayeeIsTimelocked` with signature `RelayPayeeIsTimelocked()` and selector `0x9ec568f3`
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
    #[etherror(name = "RelayPayeeIsTimelocked", abi = "RelayPayeeIsTimelocked()")]
    pub struct RelayPayeeIsTimelocked;
    ///Custom Error type `RelayPayeeUpdateInvalid` with signature `RelayPayeeUpdateInvalid()` and selector `0x561d7b2d`
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
    #[etherror(name = "RelayPayeeUpdateInvalid", abi = "RelayPayeeUpdateInvalid()")]
    pub struct RelayPayeeUpdateInvalid;
    ///Custom Error type `RelayPermissionSenderNotOrigin` with signature `RelayPermissionSenderNotOrigin()` and selector `0x5c8a268a`
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
        name = "RelayPermissionSenderNotOrigin",
        abi = "RelayPermissionSenderNotOrigin()"
    )]
    pub struct RelayPermissionSenderNotOrigin;
    ///Custom Error type `RelayProcessorCannotBeZero` with signature `RelayProcessorCannotBeZero()` and selector `0x779f4778`
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
        name = "RelayProcessorCannotBeZero",
        abi = "RelayProcessorCannotBeZero()"
    )]
    pub struct RelayProcessorCannotBeZero;
    ///Custom Error type `RelaySearcherWrongParams` with signature `RelaySearcherWrongParams()` and selector `0x31ae2a9d`
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
    #[etherror(name = "RelaySearcherWrongParams", abi = "RelaySearcherWrongParams()")]
    pub struct RelaySearcherWrongParams;
    ///Custom Error type `RelaySimulatedNotRepaid` with signature `RelaySimulatedNotRepaid(uint256,uint256)` and selector `0xd47ae88a`
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
        name = "RelaySimulatedNotRepaid",
        abi = "RelaySimulatedNotRepaid(uint256,uint256)"
    )]
    pub struct RelaySimulatedNotRepaid {
        pub bid_amount: ::ethers::core::types::U256,
        pub actual_amount: ::ethers::core::types::U256,
    }
    ///Custom Error type `RelayValidatorNotAcceptingRefundBids` with signature `RelayValidatorNotAcceptingRefundBids()` and selector `0x8b2dbdac`
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
        name = "RelayValidatorNotAcceptingRefundBids",
        abi = "RelayValidatorNotAcceptingRefundBids()"
    )]
    pub struct RelayValidatorNotAcceptingRefundBids;
    ///Custom Error type `RelayValueIsZero` with signature `RelayValueIsZero()` and selector `0x7da21207`
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
    #[etherror(name = "RelayValueIsZero", abi = "RelayValueIsZero()")]
    pub struct RelayValueIsZero;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FastLaneAuctionHandlerAbiErrors {
        RelayAuctionBidReceivedLate(RelayAuctionBidReceivedLate),
        RelayAuctionInvalidBid(RelayAuctionInvalidBid),
        RelayAuctionSearcherNotWinner(RelayAuctionSearcherNotWinner),
        RelayCannotBeSelf(RelayCannotBeSelf),
        RelayCannotBeZero(RelayCannotBeZero),
        RelayImmutableBlockAuthorRate(RelayImmutableBlockAuthorRate),
        RelayInvalidSender(RelayInvalidSender),
        RelayMustBeSelf(RelayMustBeSelf),
        RelayNotActiveValidator(RelayNotActiveValidator),
        RelayNotRepaid(RelayNotRepaid),
        RelayPayeeIsTimelocked(RelayPayeeIsTimelocked),
        RelayPayeeUpdateInvalid(RelayPayeeUpdateInvalid),
        RelayPermissionSenderNotOrigin(RelayPermissionSenderNotOrigin),
        RelayProcessorCannotBeZero(RelayProcessorCannotBeZero),
        RelaySearcherWrongParams(RelaySearcherWrongParams),
        RelaySimulatedNotRepaid(RelaySimulatedNotRepaid),
        RelayValidatorNotAcceptingRefundBids(RelayValidatorNotAcceptingRefundBids),
        RelayValueIsZero(RelayValueIsZero),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for FastLaneAuctionHandlerAbiErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <RelayAuctionBidReceivedLate as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayAuctionBidReceivedLate(decoded));
            }
            if let Ok(decoded) = <RelayAuctionInvalidBid as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayAuctionInvalidBid(decoded));
            }
            if let Ok(decoded) = <RelayAuctionSearcherNotWinner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayAuctionSearcherNotWinner(decoded));
            }
            if let Ok(decoded) = <RelayCannotBeSelf as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayCannotBeSelf(decoded));
            }
            if let Ok(decoded) = <RelayCannotBeZero as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayCannotBeZero(decoded));
            }
            if let Ok(decoded) = <RelayImmutableBlockAuthorRate as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayImmutableBlockAuthorRate(decoded));
            }
            if let Ok(decoded) = <RelayInvalidSender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayInvalidSender(decoded));
            }
            if let Ok(decoded) = <RelayMustBeSelf as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayMustBeSelf(decoded));
            }
            if let Ok(decoded) = <RelayNotActiveValidator as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayNotActiveValidator(decoded));
            }
            if let Ok(decoded) = <RelayNotRepaid as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayNotRepaid(decoded));
            }
            if let Ok(decoded) = <RelayPayeeIsTimelocked as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayPayeeIsTimelocked(decoded));
            }
            if let Ok(decoded) = <RelayPayeeUpdateInvalid as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayPayeeUpdateInvalid(decoded));
            }
            if let Ok(decoded) = <RelayPermissionSenderNotOrigin as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayPermissionSenderNotOrigin(decoded));
            }
            if let Ok(decoded) = <RelayProcessorCannotBeZero as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayProcessorCannotBeZero(decoded));
            }
            if let Ok(decoded) = <RelaySearcherWrongParams as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelaySearcherWrongParams(decoded));
            }
            if let Ok(decoded) = <RelaySimulatedNotRepaid as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelaySimulatedNotRepaid(decoded));
            }
            if let Ok(decoded) = <RelayValidatorNotAcceptingRefundBids as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayValidatorNotAcceptingRefundBids(decoded));
            }
            if let Ok(decoded) = <RelayValueIsZero as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RelayValueIsZero(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FastLaneAuctionHandlerAbiErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::RelayAuctionBidReceivedLate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayAuctionInvalidBid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayAuctionSearcherNotWinner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayCannotBeSelf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayCannotBeZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayImmutableBlockAuthorRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayInvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayMustBeSelf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayNotActiveValidator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayNotRepaid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayPayeeIsTimelocked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayPayeeUpdateInvalid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayPermissionSenderNotOrigin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayProcessorCannotBeZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelaySearcherWrongParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelaySimulatedNotRepaid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayValidatorNotAcceptingRefundBids(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RelayValueIsZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for FastLaneAuctionHandlerAbiErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <RelayAuctionBidReceivedLate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayAuctionInvalidBid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayAuctionSearcherNotWinner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayCannotBeSelf as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayCannotBeZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayImmutableBlockAuthorRate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayInvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayMustBeSelf as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayNotActiveValidator as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayNotRepaid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayPayeeIsTimelocked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayPayeeUpdateInvalid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayPermissionSenderNotOrigin as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayProcessorCannotBeZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelaySearcherWrongParams as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelaySimulatedNotRepaid as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayValidatorNotAcceptingRefundBids as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RelayValueIsZero as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for FastLaneAuctionHandlerAbiErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::RelayAuctionBidReceivedLate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayAuctionInvalidBid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayAuctionSearcherNotWinner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayCannotBeSelf(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayCannotBeZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayImmutableBlockAuthorRate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayInvalidSender(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayMustBeSelf(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayNotActiveValidator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayNotRepaid(element) => ::core::fmt::Display::fmt(element, f),
                Self::RelayPayeeIsTimelocked(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayPayeeUpdateInvalid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayPermissionSenderNotOrigin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayProcessorCannotBeZero(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelaySearcherWrongParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelaySimulatedNotRepaid(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayValidatorNotAcceptingRefundBids(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayValueIsZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for FastLaneAuctionHandlerAbiErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<RelayAuctionBidReceivedLate>
    for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayAuctionBidReceivedLate) -> Self {
            Self::RelayAuctionBidReceivedLate(value)
        }
    }
    impl ::core::convert::From<RelayAuctionInvalidBid>
    for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayAuctionInvalidBid) -> Self {
            Self::RelayAuctionInvalidBid(value)
        }
    }
    impl ::core::convert::From<RelayAuctionSearcherNotWinner>
    for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayAuctionSearcherNotWinner) -> Self {
            Self::RelayAuctionSearcherNotWinner(value)
        }
    }
    impl ::core::convert::From<RelayCannotBeSelf> for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayCannotBeSelf) -> Self {
            Self::RelayCannotBeSelf(value)
        }
    }
    impl ::core::convert::From<RelayCannotBeZero> for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayCannotBeZero) -> Self {
            Self::RelayCannotBeZero(value)
        }
    }
    impl ::core::convert::From<RelayImmutableBlockAuthorRate>
    for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayImmutableBlockAuthorRate) -> Self {
            Self::RelayImmutableBlockAuthorRate(value)
        }
    }
    impl ::core::convert::From<RelayInvalidSender> for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayInvalidSender) -> Self {
            Self::RelayInvalidSender(value)
        }
    }
    impl ::core::convert::From<RelayMustBeSelf> for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayMustBeSelf) -> Self {
            Self::RelayMustBeSelf(value)
        }
    }
    impl ::core::convert::From<RelayNotActiveValidator>
    for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayNotActiveValidator) -> Self {
            Self::RelayNotActiveValidator(value)
        }
    }
    impl ::core::convert::From<RelayNotRepaid> for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayNotRepaid) -> Self {
            Self::RelayNotRepaid(value)
        }
    }
    impl ::core::convert::From<RelayPayeeIsTimelocked>
    for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayPayeeIsTimelocked) -> Self {
            Self::RelayPayeeIsTimelocked(value)
        }
    }
    impl ::core::convert::From<RelayPayeeUpdateInvalid>
    for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayPayeeUpdateInvalid) -> Self {
            Self::RelayPayeeUpdateInvalid(value)
        }
    }
    impl ::core::convert::From<RelayPermissionSenderNotOrigin>
    for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayPermissionSenderNotOrigin) -> Self {
            Self::RelayPermissionSenderNotOrigin(value)
        }
    }
    impl ::core::convert::From<RelayProcessorCannotBeZero>
    for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayProcessorCannotBeZero) -> Self {
            Self::RelayProcessorCannotBeZero(value)
        }
    }
    impl ::core::convert::From<RelaySearcherWrongParams>
    for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelaySearcherWrongParams) -> Self {
            Self::RelaySearcherWrongParams(value)
        }
    }
    impl ::core::convert::From<RelaySimulatedNotRepaid>
    for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelaySimulatedNotRepaid) -> Self {
            Self::RelaySimulatedNotRepaid(value)
        }
    }
    impl ::core::convert::From<RelayValidatorNotAcceptingRefundBids>
    for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayValidatorNotAcceptingRefundBids) -> Self {
            Self::RelayValidatorNotAcceptingRefundBids(value)
        }
    }
    impl ::core::convert::From<RelayValueIsZero> for FastLaneAuctionHandlerAbiErrors {
        fn from(value: RelayValueIsZero) -> Self {
            Self::RelayValueIsZero(value)
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
        Hash
    )]
    #[ethevent(
        name = "CustomPaymentProcessorPaid",
        abi = "CustomPaymentProcessorPaid(address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct CustomPaymentProcessorPaidFilter {
        #[ethevent(indexed)]
        pub payor: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub payment_processor: ::ethers::core::types::Address,
        pub total_amount: ::ethers::core::types::U256,
        pub custom_allocation: ::ethers::core::types::U256,
        pub start_block: ::ethers::core::types::U256,
        pub end_block: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RelayFastBid",
        abi = "RelayFastBid(address,address,bool,uint256,address)"
    )]
    pub struct RelayFastBidFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub validator: ::ethers::core::types::Address,
        pub success: bool,
        pub bid_amount: ::ethers::core::types::U256,
        pub searcher_contract_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RelayFeeCollected",
        abi = "RelayFeeCollected(address,address,uint256)"
    )]
    pub struct RelayFeeCollectedFilter {
        #[ethevent(indexed)]
        pub payor: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub payee: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RelayFlashBid",
        abi = "RelayFlashBid(address,bytes32,address,uint256,uint256,address)"
    )]
    pub struct RelayFlashBidFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub opp_tx_hash: [u8; 32],
        #[ethevent(indexed)]
        pub validator: ::ethers::core::types::Address,
        pub bid_amount: ::ethers::core::types::U256,
        pub amount_paid: ::ethers::core::types::U256,
        pub searcher_contract_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RelayFlashBidWithRefund",
        abi = "RelayFlashBidWithRefund(address,bytes32,address,uint256,uint256,address,uint256,address)"
    )]
    pub struct RelayFlashBidWithRefundFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub opp_tx_hash: [u8; 32],
        #[ethevent(indexed)]
        pub validator: ::ethers::core::types::Address,
        pub bid_amount: ::ethers::core::types::U256,
        pub amount_paid: ::ethers::core::types::U256,
        pub searcher_contract_address: ::ethers::core::types::Address,
        pub refunded_amount: ::ethers::core::types::U256,
        pub refund_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RelayInvestigateOutcome",
        abi = "RelayInvestigateOutcome(address,address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct RelayInvestigateOutcomeFilter {
        #[ethevent(indexed)]
        pub validator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub block_number: ::ethers::core::types::U256,
        pub existing_bid_amount: ::ethers::core::types::U256,
        pub new_bid_amount: ::ethers::core::types::U256,
        pub existing_gas_price: ::ethers::core::types::U256,
        pub new_gas_price: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RelayProcessingPaidValidator",
        abi = "RelayProcessingPaidValidator(address,uint256,address)"
    )]
    pub struct RelayProcessingPaidValidatorFilter {
        #[ethevent(indexed)]
        pub validator: ::ethers::core::types::Address,
        pub validator_payment: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub initiator: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RelaySimulatedFlashBid",
        abi = "RelaySimulatedFlashBid(address,uint256,bytes32,address,address)"
    )]
    pub struct RelaySimulatedFlashBidFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub opp_tx_hash: [u8; 32],
        #[ethevent(indexed)]
        pub validator: ::ethers::core::types::Address,
        pub searcher_contract_address: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RelayValidatorPayeeUpdated",
        abi = "RelayValidatorPayeeUpdated(address,address,address)"
    )]
    pub struct RelayValidatorPayeeUpdatedFilter {
        pub validator: ::ethers::core::types::Address,
        pub payee: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub initiator: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RelayWithdrawStuckERC20",
        abi = "RelayWithdrawStuckERC20(address,address,uint256)"
    )]
    pub struct RelayWithdrawStuckERC20Filter {
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RelayWithdrawStuckNativeToken",
        abi = "RelayWithdrawStuckNativeToken(address,uint256)"
    )]
    pub struct RelayWithdrawStuckNativeTokenFilter {
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FastLaneAuctionHandlerAbiEvents {
        CustomPaymentProcessorPaidFilter(CustomPaymentProcessorPaidFilter),
        RelayFastBidFilter(RelayFastBidFilter),
        RelayFeeCollectedFilter(RelayFeeCollectedFilter),
        RelayFlashBidFilter(RelayFlashBidFilter),
        RelayFlashBidWithRefundFilter(RelayFlashBidWithRefundFilter),
        RelayInvestigateOutcomeFilter(RelayInvestigateOutcomeFilter),
        RelayProcessingPaidValidatorFilter(RelayProcessingPaidValidatorFilter),
        RelaySimulatedFlashBidFilter(RelaySimulatedFlashBidFilter),
        RelayValidatorPayeeUpdatedFilter(RelayValidatorPayeeUpdatedFilter),
        RelayWithdrawStuckERC20Filter(RelayWithdrawStuckERC20Filter),
        RelayWithdrawStuckNativeTokenFilter(RelayWithdrawStuckNativeTokenFilter),
    }
    impl ::ethers::contract::EthLogDecode for FastLaneAuctionHandlerAbiEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CustomPaymentProcessorPaidFilter::decode_log(log) {
                return Ok(
                    FastLaneAuctionHandlerAbiEvents::CustomPaymentProcessorPaidFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RelayFastBidFilter::decode_log(log) {
                return Ok(FastLaneAuctionHandlerAbiEvents::RelayFastBidFilter(decoded));
            }
            if let Ok(decoded) = RelayFeeCollectedFilter::decode_log(log) {
                return Ok(
                    FastLaneAuctionHandlerAbiEvents::RelayFeeCollectedFilter(decoded),
                );
            }
            if let Ok(decoded) = RelayFlashBidFilter::decode_log(log) {
                return Ok(FastLaneAuctionHandlerAbiEvents::RelayFlashBidFilter(decoded));
            }
            if let Ok(decoded) = RelayFlashBidWithRefundFilter::decode_log(log) {
                return Ok(
                    FastLaneAuctionHandlerAbiEvents::RelayFlashBidWithRefundFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RelayInvestigateOutcomeFilter::decode_log(log) {
                return Ok(
                    FastLaneAuctionHandlerAbiEvents::RelayInvestigateOutcomeFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RelayProcessingPaidValidatorFilter::decode_log(log) {
                return Ok(
                    FastLaneAuctionHandlerAbiEvents::RelayProcessingPaidValidatorFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RelaySimulatedFlashBidFilter::decode_log(log) {
                return Ok(
                    FastLaneAuctionHandlerAbiEvents::RelaySimulatedFlashBidFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RelayValidatorPayeeUpdatedFilter::decode_log(log) {
                return Ok(
                    FastLaneAuctionHandlerAbiEvents::RelayValidatorPayeeUpdatedFilter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RelayWithdrawStuckERC20Filter::decode_log(log) {
                return Ok(
                    FastLaneAuctionHandlerAbiEvents::RelayWithdrawStuckERC20Filter(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded) = RelayWithdrawStuckNativeTokenFilter::decode_log(log) {
                return Ok(
                    FastLaneAuctionHandlerAbiEvents::RelayWithdrawStuckNativeTokenFilter(
                        decoded,
                    ),
                );
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for FastLaneAuctionHandlerAbiEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CustomPaymentProcessorPaidFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayFastBidFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayFeeCollectedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayFlashBidFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayFlashBidWithRefundFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayInvestigateOutcomeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayProcessingPaidValidatorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelaySimulatedFlashBidFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayValidatorPayeeUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayWithdrawStuckERC20Filter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RelayWithdrawStuckNativeTokenFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CustomPaymentProcessorPaidFilter>
    for FastLaneAuctionHandlerAbiEvents {
        fn from(value: CustomPaymentProcessorPaidFilter) -> Self {
            Self::CustomPaymentProcessorPaidFilter(value)
        }
    }
    impl ::core::convert::From<RelayFastBidFilter> for FastLaneAuctionHandlerAbiEvents {
        fn from(value: RelayFastBidFilter) -> Self {
            Self::RelayFastBidFilter(value)
        }
    }
    impl ::core::convert::From<RelayFeeCollectedFilter>
    for FastLaneAuctionHandlerAbiEvents {
        fn from(value: RelayFeeCollectedFilter) -> Self {
            Self::RelayFeeCollectedFilter(value)
        }
    }
    impl ::core::convert::From<RelayFlashBidFilter> for FastLaneAuctionHandlerAbiEvents {
        fn from(value: RelayFlashBidFilter) -> Self {
            Self::RelayFlashBidFilter(value)
        }
    }
    impl ::core::convert::From<RelayFlashBidWithRefundFilter>
    for FastLaneAuctionHandlerAbiEvents {
        fn from(value: RelayFlashBidWithRefundFilter) -> Self {
            Self::RelayFlashBidWithRefundFilter(value)
        }
    }
    impl ::core::convert::From<RelayInvestigateOutcomeFilter>
    for FastLaneAuctionHandlerAbiEvents {
        fn from(value: RelayInvestigateOutcomeFilter) -> Self {
            Self::RelayInvestigateOutcomeFilter(value)
        }
    }
    impl ::core::convert::From<RelayProcessingPaidValidatorFilter>
    for FastLaneAuctionHandlerAbiEvents {
        fn from(value: RelayProcessingPaidValidatorFilter) -> Self {
            Self::RelayProcessingPaidValidatorFilter(value)
        }
    }
    impl ::core::convert::From<RelaySimulatedFlashBidFilter>
    for FastLaneAuctionHandlerAbiEvents {
        fn from(value: RelaySimulatedFlashBidFilter) -> Self {
            Self::RelaySimulatedFlashBidFilter(value)
        }
    }
    impl ::core::convert::From<RelayValidatorPayeeUpdatedFilter>
    for FastLaneAuctionHandlerAbiEvents {
        fn from(value: RelayValidatorPayeeUpdatedFilter) -> Self {
            Self::RelayValidatorPayeeUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<RelayWithdrawStuckERC20Filter>
    for FastLaneAuctionHandlerAbiEvents {
        fn from(value: RelayWithdrawStuckERC20Filter) -> Self {
            Self::RelayWithdrawStuckERC20Filter(value)
        }
    }
    impl ::core::convert::From<RelayWithdrawStuckNativeTokenFilter>
    for FastLaneAuctionHandlerAbiEvents {
        fn from(value: RelayWithdrawStuckNativeTokenFilter) -> Self {
            Self::RelayWithdrawStuckNativeTokenFilter(value)
        }
    }
    ///Container type for all input parameters for the `clearValidatorPayee` function with signature `clearValidatorPayee()` and selector `0x43b1f483`
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
    #[ethcall(name = "clearValidatorPayee", abi = "clearValidatorPayee()")]
    pub struct ClearValidatorPayeeCall;
    ///Container type for all input parameters for the `collectFees` function with signature `collectFees()` and selector `0xc8796572`
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
    #[ethcall(name = "collectFees", abi = "collectFees()")]
    pub struct CollectFeesCall;
    ///Container type for all input parameters for the `fastBidWrapper` function with signature `fastBidWrapper(address,uint256,address,bytes)` and selector `0xb4f042b8`
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
        name = "fastBidWrapper",
        abi = "fastBidWrapper(address,uint256,address,bytes)"
    )]
    pub struct FastBidWrapperCall {
        pub msg_sender: ::ethers::core::types::Address,
        pub fast_price: ::ethers::core::types::U256,
        pub searcher_to_address: ::ethers::core::types::Address,
        pub searcher_call_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `fulfilledAuctionsMap` function with signature `fulfilledAuctionsMap(bytes32)` and selector `0x59f5764f`
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
    #[ethcall(name = "fulfilledAuctionsMap", abi = "fulfilledAuctionsMap(bytes32)")]
    pub struct FulfilledAuctionsMapCall(pub [u8; 32]);
    ///Container type for all input parameters for the `fulfilledPGAMap` function with signature `fulfilledPGAMap(uint256)` and selector `0xd67a7384`
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
    #[ethcall(name = "fulfilledPGAMap", abi = "fulfilledPGAMap(uint256)")]
    pub struct FulfilledPGAMapCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `getValidatorBalance` function with signature `getValidatorBalance(address)` and selector `0x19d16629`
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
    #[ethcall(name = "getValidatorBalance", abi = "getValidatorBalance(address)")]
    pub struct GetValidatorBalanceCall {
        pub validator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getValidatorBlockOfLastWithdraw` function with signature `getValidatorBlockOfLastWithdraw(address)` and selector `0xd7cfc841`
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
        name = "getValidatorBlockOfLastWithdraw",
        abi = "getValidatorBlockOfLastWithdraw(address)"
    )]
    pub struct GetValidatorBlockOfLastWithdrawCall {
        pub validator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getValidatorPayee` function with signature `getValidatorPayee(address)` and selector `0x83ebbe4b`
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
    #[ethcall(name = "getValidatorPayee", abi = "getValidatorPayee(address)")]
    pub struct GetValidatorPayeeCall {
        pub validator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getValidatorRecipient` function with signature `getValidatorRecipient(address)` and selector `0x78313b36`
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
    #[ethcall(name = "getValidatorRecipient", abi = "getValidatorRecipient(address)")]
    pub struct GetValidatorRecipientCall {
        pub validator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isPayeeTimeLocked` function with signature `isPayeeTimeLocked(address)` and selector `0xd3550dfa`
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
    #[ethcall(name = "isPayeeTimeLocked", abi = "isPayeeTimeLocked(address)")]
    pub struct IsPayeeTimeLockedCall {
        pub validator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isValidPayee` function with signature `isValidPayee(address,address)` and selector `0xaf6ccc0b`
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
    #[ethcall(name = "isValidPayee", abi = "isValidPayee(address,address)")]
    pub struct IsValidPayeeCall {
        pub validator: ::ethers::core::types::Address,
        pub payee: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `payValidatorCustom` function with signature `payValidatorCustom(address,uint256,bytes)` and selector `0xc0a2ab03`
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
        name = "payValidatorCustom",
        abi = "payValidatorCustom(address,uint256,bytes)"
    )]
    pub struct PayValidatorCustomCall {
        pub payment_processor: ::ethers::core::types::Address,
        pub custom_allocation: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `payValidatorFee` function with signature `payValidatorFee(address)` and selector `0x43909ed0`
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
    #[ethcall(name = "payValidatorFee", abi = "payValidatorFee(address)")]
    pub struct PayValidatorFeeCall {
        pub payor: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `payeeMap` function with signature `payeeMap(address)` and selector `0x69660bd6`
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
    #[ethcall(name = "payeeMap", abi = "payeeMap(address)")]
    pub struct PayeeMapCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `simulateFlashBid` function with signature `simulateFlashBid(uint256,bytes32,address,bytes)` and selector `0x10baf2bf`
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
        name = "simulateFlashBid",
        abi = "simulateFlashBid(uint256,bytes32,address,bytes)"
    )]
    pub struct SimulateFlashBidCall {
        pub bid_amount: ::ethers::core::types::U256,
        pub opp_tx_hash: [u8; 32],
        pub searcher_to_address: ::ethers::core::types::Address,
        pub searcher_call_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `submitFastBid` function with signature `submitFastBid(uint256,address,bytes)` and selector `0xcda5b8ac`
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
    #[ethcall(name = "submitFastBid", abi = "submitFastBid(uint256,address,bytes)")]
    pub struct SubmitFastBidCall {
        pub fast_gas_price: ::ethers::core::types::U256,
        pub searcher_to_address: ::ethers::core::types::Address,
        pub searcher_call_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `submitFlashBid` function with signature `submitFlashBid(uint256,bytes32,address,bytes)` and selector `0xeac046f8`
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
        name = "submitFlashBid",
        abi = "submitFlashBid(uint256,bytes32,address,bytes)"
    )]
    pub struct SubmitFlashBidCall {
        pub bid_amount: ::ethers::core::types::U256,
        pub opp_tx_hash: [u8; 32],
        pub searcher_to_address: ::ethers::core::types::Address,
        pub searcher_call_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `submitFlashBidWithRefund` function with signature `submitFlashBidWithRefund(uint256,bytes32,address,address,bytes)` and selector `0x2f10b549`
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
        name = "submitFlashBidWithRefund",
        abi = "submitFlashBidWithRefund(uint256,bytes32,address,address,bytes)"
    )]
    pub struct SubmitFlashBidWithRefundCall {
        pub bid_amount: ::ethers::core::types::U256,
        pub opp_tx_hash: [u8; 32],
        pub refund_address: ::ethers::core::types::Address,
        pub searcher_to_address: ::ethers::core::types::Address,
        pub searcher_call_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `syncStuckNativeToken` function with signature `syncStuckNativeToken()` and selector `0x8533f66e`
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
    #[ethcall(name = "syncStuckNativeToken", abi = "syncStuckNativeToken()")]
    pub struct SyncStuckNativeTokenCall;
    ///Container type for all input parameters for the `updateValidatorPayee` function with signature `updateValidatorPayee(address)` and selector `0xc64dfa0a`
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
    #[ethcall(name = "updateValidatorPayee", abi = "updateValidatorPayee(address)")]
    pub struct UpdateValidatorPayeeCall {
        pub payee: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateValidatorRefundShare` function with signature `updateValidatorRefundShare(uint256)` and selector `0x9467ab16`
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
        name = "updateValidatorRefundShare",
        abi = "updateValidatorRefundShare(uint256)"
    )]
    pub struct UpdateValidatorRefundShareCall {
        pub refund_share: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `validatorsBalanceMap` function with signature `validatorsBalanceMap(address)` and selector `0x8cfdbf5f`
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
    #[ethcall(name = "validatorsBalanceMap", abi = "validatorsBalanceMap(address)")]
    pub struct ValidatorsBalanceMapCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `validatorsRefundShareMap` function with signature `validatorsRefundShareMap(address)` and selector `0x4c3175b1`
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
        name = "validatorsRefundShareMap",
        abi = "validatorsRefundShareMap(address)"
    )]
    pub struct ValidatorsRefundShareMapCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `validatorsTotal` function with signature `validatorsTotal()` and selector `0x03eaca08`
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
    #[ethcall(name = "validatorsTotal", abi = "validatorsTotal()")]
    pub struct ValidatorsTotalCall;
    ///Container type for all input parameters for the `withdrawStuckERC20` function with signature `withdrawStuckERC20(address)` and selector `0x3963510b`
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
    #[ethcall(name = "withdrawStuckERC20", abi = "withdrawStuckERC20(address)")]
    pub struct WithdrawStuckERC20Call {
        pub token_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FastLaneAuctionHandlerAbiCalls {
        ClearValidatorPayee(ClearValidatorPayeeCall),
        CollectFees(CollectFeesCall),
        FastBidWrapper(FastBidWrapperCall),
        FulfilledAuctionsMap(FulfilledAuctionsMapCall),
        FulfilledPGAMap(FulfilledPGAMapCall),
        GetValidatorBalance(GetValidatorBalanceCall),
        GetValidatorBlockOfLastWithdraw(GetValidatorBlockOfLastWithdrawCall),
        GetValidatorPayee(GetValidatorPayeeCall),
        GetValidatorRecipient(GetValidatorRecipientCall),
        IsPayeeTimeLocked(IsPayeeTimeLockedCall),
        IsValidPayee(IsValidPayeeCall),
        PayValidatorCustom(PayValidatorCustomCall),
        PayValidatorFee(PayValidatorFeeCall),
        PayeeMap(PayeeMapCall),
        SimulateFlashBid(SimulateFlashBidCall),
        SubmitFastBid(SubmitFastBidCall),
        SubmitFlashBid(SubmitFlashBidCall),
        SubmitFlashBidWithRefund(SubmitFlashBidWithRefundCall),
        SyncStuckNativeToken(SyncStuckNativeTokenCall),
        UpdateValidatorPayee(UpdateValidatorPayeeCall),
        UpdateValidatorRefundShare(UpdateValidatorRefundShareCall),
        ValidatorsBalanceMap(ValidatorsBalanceMapCall),
        ValidatorsRefundShareMap(ValidatorsRefundShareMapCall),
        ValidatorsTotal(ValidatorsTotalCall),
        WithdrawStuckERC20(WithdrawStuckERC20Call),
    }
    impl ::ethers::core::abi::AbiDecode for FastLaneAuctionHandlerAbiCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ClearValidatorPayeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClearValidatorPayee(decoded));
            }
            if let Ok(decoded) = <CollectFeesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CollectFees(decoded));
            }
            if let Ok(decoded) = <FastBidWrapperCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FastBidWrapper(decoded));
            }
            if let Ok(decoded) = <FulfilledAuctionsMapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FulfilledAuctionsMap(decoded));
            }
            if let Ok(decoded) = <FulfilledPGAMapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FulfilledPGAMap(decoded));
            }
            if let Ok(decoded) = <GetValidatorBalanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetValidatorBalance(decoded));
            }
            if let Ok(decoded) = <GetValidatorBlockOfLastWithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetValidatorBlockOfLastWithdraw(decoded));
            }
            if let Ok(decoded) = <GetValidatorPayeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetValidatorPayee(decoded));
            }
            if let Ok(decoded) = <GetValidatorRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetValidatorRecipient(decoded));
            }
            if let Ok(decoded) = <IsPayeeTimeLockedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsPayeeTimeLocked(decoded));
            }
            if let Ok(decoded) = <IsValidPayeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsValidPayee(decoded));
            }
            if let Ok(decoded) = <PayValidatorCustomCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PayValidatorCustom(decoded));
            }
            if let Ok(decoded) = <PayValidatorFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PayValidatorFee(decoded));
            }
            if let Ok(decoded) = <PayeeMapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PayeeMap(decoded));
            }
            if let Ok(decoded) = <SimulateFlashBidCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateFlashBid(decoded));
            }
            if let Ok(decoded) = <SubmitFastBidCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmitFastBid(decoded));
            }
            if let Ok(decoded) = <SubmitFlashBidCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmitFlashBid(decoded));
            }
            if let Ok(decoded) = <SubmitFlashBidWithRefundCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SubmitFlashBidWithRefund(decoded));
            }
            if let Ok(decoded) = <SyncStuckNativeTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SyncStuckNativeToken(decoded));
            }
            if let Ok(decoded) = <UpdateValidatorPayeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateValidatorPayee(decoded));
            }
            if let Ok(decoded) = <UpdateValidatorRefundShareCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpdateValidatorRefundShare(decoded));
            }
            if let Ok(decoded) = <ValidatorsBalanceMapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidatorsBalanceMap(decoded));
            }
            if let Ok(decoded) = <ValidatorsRefundShareMapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidatorsRefundShareMap(decoded));
            }
            if let Ok(decoded) = <ValidatorsTotalCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidatorsTotal(decoded));
            }
            if let Ok(decoded) = <WithdrawStuckERC20Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawStuckERC20(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FastLaneAuctionHandlerAbiCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ClearValidatorPayee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CollectFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FastBidWrapper(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FulfilledAuctionsMap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FulfilledPGAMap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetValidatorBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetValidatorBlockOfLastWithdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetValidatorPayee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetValidatorRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPayeeTimeLocked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsValidPayee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PayValidatorCustom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PayValidatorFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PayeeMap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateFlashBid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitFastBid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitFlashBid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SubmitFlashBidWithRefund(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SyncStuckNativeToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateValidatorPayee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateValidatorRefundShare(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidatorsBalanceMap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidatorsRefundShareMap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidatorsTotal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawStuckERC20(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FastLaneAuctionHandlerAbiCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ClearValidatorPayee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CollectFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::FastBidWrapper(element) => ::core::fmt::Display::fmt(element, f),
                Self::FulfilledAuctionsMap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FulfilledPGAMap(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetValidatorBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetValidatorBlockOfLastWithdraw(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetValidatorPayee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetValidatorRecipient(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsPayeeTimeLocked(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsValidPayee(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayValidatorCustom(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PayValidatorFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::PayeeMap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateFlashBid(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitFastBid(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitFlashBid(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitFlashBidWithRefund(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SyncStuckNativeToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateValidatorPayee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpdateValidatorRefundShare(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorsBalanceMap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorsRefundShareMap(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidatorsTotal(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawStuckERC20(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ClearValidatorPayeeCall>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: ClearValidatorPayeeCall) -> Self {
            Self::ClearValidatorPayee(value)
        }
    }
    impl ::core::convert::From<CollectFeesCall> for FastLaneAuctionHandlerAbiCalls {
        fn from(value: CollectFeesCall) -> Self {
            Self::CollectFees(value)
        }
    }
    impl ::core::convert::From<FastBidWrapperCall> for FastLaneAuctionHandlerAbiCalls {
        fn from(value: FastBidWrapperCall) -> Self {
            Self::FastBidWrapper(value)
        }
    }
    impl ::core::convert::From<FulfilledAuctionsMapCall>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: FulfilledAuctionsMapCall) -> Self {
            Self::FulfilledAuctionsMap(value)
        }
    }
    impl ::core::convert::From<FulfilledPGAMapCall> for FastLaneAuctionHandlerAbiCalls {
        fn from(value: FulfilledPGAMapCall) -> Self {
            Self::FulfilledPGAMap(value)
        }
    }
    impl ::core::convert::From<GetValidatorBalanceCall>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: GetValidatorBalanceCall) -> Self {
            Self::GetValidatorBalance(value)
        }
    }
    impl ::core::convert::From<GetValidatorBlockOfLastWithdrawCall>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: GetValidatorBlockOfLastWithdrawCall) -> Self {
            Self::GetValidatorBlockOfLastWithdraw(value)
        }
    }
    impl ::core::convert::From<GetValidatorPayeeCall>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: GetValidatorPayeeCall) -> Self {
            Self::GetValidatorPayee(value)
        }
    }
    impl ::core::convert::From<GetValidatorRecipientCall>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: GetValidatorRecipientCall) -> Self {
            Self::GetValidatorRecipient(value)
        }
    }
    impl ::core::convert::From<IsPayeeTimeLockedCall>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: IsPayeeTimeLockedCall) -> Self {
            Self::IsPayeeTimeLocked(value)
        }
    }
    impl ::core::convert::From<IsValidPayeeCall> for FastLaneAuctionHandlerAbiCalls {
        fn from(value: IsValidPayeeCall) -> Self {
            Self::IsValidPayee(value)
        }
    }
    impl ::core::convert::From<PayValidatorCustomCall>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: PayValidatorCustomCall) -> Self {
            Self::PayValidatorCustom(value)
        }
    }
    impl ::core::convert::From<PayValidatorFeeCall> for FastLaneAuctionHandlerAbiCalls {
        fn from(value: PayValidatorFeeCall) -> Self {
            Self::PayValidatorFee(value)
        }
    }
    impl ::core::convert::From<PayeeMapCall> for FastLaneAuctionHandlerAbiCalls {
        fn from(value: PayeeMapCall) -> Self {
            Self::PayeeMap(value)
        }
    }
    impl ::core::convert::From<SimulateFlashBidCall> for FastLaneAuctionHandlerAbiCalls {
        fn from(value: SimulateFlashBidCall) -> Self {
            Self::SimulateFlashBid(value)
        }
    }
    impl ::core::convert::From<SubmitFastBidCall> for FastLaneAuctionHandlerAbiCalls {
        fn from(value: SubmitFastBidCall) -> Self {
            Self::SubmitFastBid(value)
        }
    }
    impl ::core::convert::From<SubmitFlashBidCall> for FastLaneAuctionHandlerAbiCalls {
        fn from(value: SubmitFlashBidCall) -> Self {
            Self::SubmitFlashBid(value)
        }
    }
    impl ::core::convert::From<SubmitFlashBidWithRefundCall>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: SubmitFlashBidWithRefundCall) -> Self {
            Self::SubmitFlashBidWithRefund(value)
        }
    }
    impl ::core::convert::From<SyncStuckNativeTokenCall>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: SyncStuckNativeTokenCall) -> Self {
            Self::SyncStuckNativeToken(value)
        }
    }
    impl ::core::convert::From<UpdateValidatorPayeeCall>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: UpdateValidatorPayeeCall) -> Self {
            Self::UpdateValidatorPayee(value)
        }
    }
    impl ::core::convert::From<UpdateValidatorRefundShareCall>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: UpdateValidatorRefundShareCall) -> Self {
            Self::UpdateValidatorRefundShare(value)
        }
    }
    impl ::core::convert::From<ValidatorsBalanceMapCall>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: ValidatorsBalanceMapCall) -> Self {
            Self::ValidatorsBalanceMap(value)
        }
    }
    impl ::core::convert::From<ValidatorsRefundShareMapCall>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: ValidatorsRefundShareMapCall) -> Self {
            Self::ValidatorsRefundShareMap(value)
        }
    }
    impl ::core::convert::From<ValidatorsTotalCall> for FastLaneAuctionHandlerAbiCalls {
        fn from(value: ValidatorsTotalCall) -> Self {
            Self::ValidatorsTotal(value)
        }
    }
    impl ::core::convert::From<WithdrawStuckERC20Call>
    for FastLaneAuctionHandlerAbiCalls {
        fn from(value: WithdrawStuckERC20Call) -> Self {
            Self::WithdrawStuckERC20(value)
        }
    }
    ///Container type for all return fields from the `collectFees` function with signature `collectFees()` and selector `0xc8796572`
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
    pub struct CollectFeesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `fastBidWrapper` function with signature `fastBidWrapper(address,uint256,address,bytes)` and selector `0xb4f042b8`
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
    pub struct FastBidWrapperReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `fulfilledAuctionsMap` function with signature `fulfilledAuctionsMap(bytes32)` and selector `0x59f5764f`
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
    pub struct FulfilledAuctionsMapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `fulfilledPGAMap` function with signature `fulfilledPGAMap(uint256)` and selector `0xd67a7384`
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
    pub struct FulfilledPGAMapReturn {
        pub lowest_gas_price: u64,
        pub lowest_fast_price: u64,
        pub lowest_total_price: u64,
    }
    ///Container type for all return fields from the `getValidatorBalance` function with signature `getValidatorBalance(address)` and selector `0x19d16629`
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
    pub struct GetValidatorBalanceReturn {
        pub validator_balance: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getValidatorBlockOfLastWithdraw` function with signature `getValidatorBlockOfLastWithdraw(address)` and selector `0xd7cfc841`
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
    pub struct GetValidatorBlockOfLastWithdrawReturn {
        pub block_number: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getValidatorPayee` function with signature `getValidatorPayee(address)` and selector `0x83ebbe4b`
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
    pub struct GetValidatorPayeeReturn {
        pub payee: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getValidatorRecipient` function with signature `getValidatorRecipient(address)` and selector `0x78313b36`
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
    pub struct GetValidatorRecipientReturn {
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `isPayeeTimeLocked` function with signature `isPayeeTimeLocked(address)` and selector `0xd3550dfa`
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
    pub struct IsPayeeTimeLockedReturn {
        pub is_time_locked: bool,
    }
    ///Container type for all return fields from the `isValidPayee` function with signature `isValidPayee(address,address)` and selector `0xaf6ccc0b`
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
    pub struct IsValidPayeeReturn {
        pub valid: bool,
    }
    ///Container type for all return fields from the `payeeMap` function with signature `payeeMap(address)` and selector `0x69660bd6`
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
    pub struct PayeeMapReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `validatorsBalanceMap` function with signature `validatorsBalanceMap(address)` and selector `0x8cfdbf5f`
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
    pub struct ValidatorsBalanceMapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `validatorsRefundShareMap` function with signature `validatorsRefundShareMap(address)` and selector `0x4c3175b1`
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
    pub struct ValidatorsRefundShareMapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `validatorsTotal` function with signature `validatorsTotal()` and selector `0x03eaca08`
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
    pub struct ValidatorsTotalReturn(pub ::ethers::core::types::U256);
}
