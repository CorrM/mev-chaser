// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.23;
pragma abicoder v2;

import "./Interfaces.sol";
import "./Utils.sol";
import {OneSwapInfo, Swapper} from "./Swapper.sol";
//import {console2} from "forge-std/Test.sol";

// To make tests in sepolia testnetwork use token address in sepolia not in mainnet
// Also if for some reason Balancer changes there vault contract address in sepolia, you should change it too
// You can get the tokent the valut hold by https://sepolia.etherscan.io/address/0xBA12222222228d8Ba445958a75a0704d566BF2C8
// then check `Token Holdings`
// https://docs.balancer.fi/reference/contracts/error-codes.html#vault
// https://solidity-by-example.org/defi/uniswap-v2/

// Loan errors
error InsufficientFundsToRepayLoanError(
    address token,
    uint256 amountOut,
    uint256 amountToPayback
);

// TODO: Split into multiple contracts, dont do every thing in one contract, as this contract are only a "Balancer" FlashLoanRecipient
//       Use another contract to handle more that one PROVIDER, with passing callback to the main contract, Don't forget to change the token recipient
//       to main contract not to that contract


// OPTMIZATION https://certik.medium.com/gas-optimization-in-ethereum-smart-contracts-10-best-practices-cbd57548bdf0
contract BalancerFlashLoanRecipient is IFlashLoanRecipient {
    using SafeERC20 for IERC20;

    IVault private immutable _vault;
    address private immutable _owner;
    IERC20 private immutable WMATIC;

    constructor() {
        require(msg.sender != address(0), "constructor sender invalid address");

        _owner = msg.sender;
        _vault = IVault(0xBA12222222228d8Ba445958a75a0704d566BF2C8);
        WMATIC = IERC20(0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270);
    }

    modifier onlyOwner() {
        _checkOwner();
        _;
    }

    receive() external payable {}

    function _checkOwner() internal view virtual {
        require(msg.sender == _owner, "You are not the owner");
    }

    function getFlashLoanFeePercentage() public view returns (uint256) {
        return _vault.getProtocolFeesCollector().getFlashLoanFeePercentage();
    }

    function getBalance() external view onlyOwner returns (uint256) {
        return address(this).balance;
    }

    function getTokenBalance(
        address[] memory tokens
    ) external view onlyOwner returns (uint256[] memory balances) {
        require(tokens.length > 0, "Empty tokens");

        balances = new uint256[](tokens.length);
        for (uint256 i = 0; i < tokens.length; i = Utils.unsafeInc(i)) {
            address tokenContract = tokens[i];

            balances[i] = IERC20(tokenContract).balanceOf(address(this));
        }

        return balances;
    }

    function withdraw(uint256 amount) external onlyOwner {
        payable(msg.sender).transfer(amount);
    }

    function withdrawToken(
        address[] calldata tokens,
        uint256[] calldata amounts
    ) external onlyOwner {
        require(
            tokens.length == amounts.length,
            "The length of tokens and amounts must be equal"
        );

        for (uint256 i = 0; i < tokens.length; i = Utils.unsafeInc(i)) {
            address tokenContract = tokens[i];
            uint256 amount = amounts[i];

            IERC20(tokenContract).transfer(msg.sender, amount);
        }
    }

    function getLoanThenMultiSwap(
        OneSwapInfo[] calldata swaps,
        bool chainSwaps,
        bool returnOutput
    ) external onlyOwner returns (int256 profit) {
        require(swaps.length > 0, "Empty swap chain");

        IERC20 loanToken = IERC20(swaps[0].TokenIn);
        uint256 loanAmount = swaps[0].AmountIn;
        bytes memory data = abi.encode(swaps, chainSwaps, returnOutput);

        IERC20[] memory tokens = new IERC20[](1);
        tokens[0] = loanToken;

        uint256[] memory amounts = new uint256[](1);
        amounts[0] = loanAmount;

        _vault.flashLoan(this, tokens, amounts, data);

        // TODO: Should check if the input token is the last token so the amount check are valid
        //       if not then that a vulnerability to let the contract pay the loan
        return
            int256(swaps[swaps.length - 1].AmountOutMin) -
            int256(swaps[0].AmountIn);
    }

    function receiveFlashLoan(
        IERC20[] memory tokens,
        uint256[] memory amounts,
        uint256[] memory feeAmounts,
        bytes memory userData
    ) external override {
        require(msg.sender == address(_vault), "Only vault are allowed");
        require(tokens.length == 1, "Only support one to one loan");
        require(amounts.length == 1, "Only support one to one loan");

        (OneSwapInfo[] memory swaps, bool chainSwaps, bool returnEveryAmountsOut) = abi
            .decode(userData, (OneSwapInfo[], bool, bool));

        // Do swaps
        uint256[] memory amountsOut = Swapper.multiSwap(
            swaps,
            chainSwaps,
            false
        );

        //console2.log("Multi swap done");
        //console2.log(amountsOut.length);
        //console2.log(amountsOut[amountsOut.length - 1]);

        // Repay
        uint256 amountToPayback = amounts[0] + feeAmounts[0];
        uint256 lastAmountOut = amountsOut[amountsOut.length - 1];
        // TODO: Should check if the input token is the last token so the amount check are valid
        //       if not then that a vulnerability to let the contract pay the loan
        if (amountToPayback > lastAmountOut) {
            revert InsufficientFundsToRepayLoanError(
                address(tokens[0]),
                lastAmountOut,
                amountToPayback
            );
        }

        //console2.log("Try Pay the loan");

        address payable vault = payable(msg.sender); // same as vault
        require(tokens[0].transfer(vault, amountToPayback), "repay failed");

        //console2.log("Pay the loan done");
    }

    function fastLaneCall(
        address,
        uint256 bidAmount,
        bytes memory data
    ) external payable returns (bool, bytes memory) {
        address payable fastLaneAddress = payable(msg.sender);

        // Call the function
        // https://medium.com/@solidity101/100daysofsolidity-understanding-the-call-function-in-solidity-interacting-with-contracts-4ccd216b1dfe
        (bool success, bytes memory returnedData) = address(this).call(data);
        if (!success) {
            return (false, returnedData);
        }

        // Check the balance
        uint256 balance = WMATIC.balanceOf(address(this));

        require(balance > bidAmount, "<B");
        fastLaneAddress.transfer(bidAmount);

        return (true, new bytes(0));
    }
}
