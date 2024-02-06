// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.23;
pragma abicoder v2;

import "./Interfaces.sol";

// To make tests in sepolia testnetwork use token address in sepolia not in mainnet
// Also if for some reason Balancer changes there vault contract address in sepolia, you should change it too
// You can get the tokent the valut hold by https://sepolia.etherscan.io/address/0xBA12222222228d8Ba445958a75a0704d566BF2C8
// then check `Token Holdings`
// https://docs.balancer.fi/reference/contracts/error-codes.html#vault
// https://solidity-by-example.org/defi/uniswap-v2/

/**
 * Automated Market Maker (AMM) protocols
 */
enum AmmProtocol {
    UniswapV2,
    UniswapV3
}

struct OneSwapInfo {
    AmmProtocol Protocol;
    address Router;
    address TokenIn;
    bytes Path;
    uint256 AmountIn;
    uint256 AmountOutMin;
    uint256 Deadline;
}

// Automated Market Maker errors
error NotSupportedAmmProtocolError(AmmProtocol protocol);

// Loan errors
error InsufficientFundsToRepayLoanError(
    address token,
    uint256 amountOut,
    uint256 amountToPayback
);

// Swap errors
error MultiSwapError(uint256 swapIndex, string errorReason);
error MultiSwapInsufficientFundsToSwapError(
    uint256 swapIndex,
    address token,
    uint256 curBalance,
    uint256 amount
);

// TODO: Add event for profit
// TODO: Split into multiple contracts, dont do every thing in one contract, as this contract are only a "Balancer" FlashLoanRecipient
//       Use another contract to handle more that one provider, with passing callback to the main contract, Don't forget to change the token recipient
//       to main contract not to that contract

library SwapUtils {
    /**
     * Don't call this function before make sure `router` approved to swap `amountIn`
     * and this contract has enough balance of `tokenIn`
     * Use `doSwapChain` instead
     */
    function swapExactTokensForTokensUniswapV2(
        address router,
        address[] memory path,
        uint256 amountIn,
        uint256 amountOutMin,
        uint256 deadline
    ) internal returns (uint256, bool, string memory) {
        IUniswapV2Router02 routerV2 = IUniswapV2Router02(router);
        try
            routerV2.swapExactTokensForTokens(
                amountIn,
                amountOutMin,
                path,
                address(this),
                deadline
            )
        returns (uint256[] memory amounts) {
            // amounts[0] = Input token amount, amounts[1] = Output token amount
            return (amounts[1], false, "");
        } catch Error(string memory reason) {
            return (0, true, reason);
        }

        // return (0, "Unkown error");
    }

    /**
     * Don't call this function before make sure `router` approved to swap `amountIn`
     * and this contract has enough balance of `tokenIn`
     * Use `doSwapChain` instead
     */
    function swapExactInputUniswapV3(
        address router,
        bytes memory path,
        uint256 amountIn,
        uint256 amountOutMin,
        uint256 deadline
    ) internal returns (uint256, bool, string memory) {
        ISwapRouter routerToBuy = ISwapRouter(router);
        ISwapRouter.ExactInputParams memory params = ISwapRouter
            .ExactInputParams({
                path: path,
                recipient: address(this),
                deadline: deadline,
                amountIn: amountIn,
                amountOutMinimum: amountOutMin
            });

        try routerToBuy.exactInput(params) returns (uint256 amountOut) {
            return (amountOut, false, "");
        } catch Error(string memory reason) {
            return (0, true, reason);
        }

        // return (0, "Unkown error");
    }
}

// OPTMIZATION https://certik.medium.com/gas-optimization-in-ethereum-smart-contracts-10-best-practices-cbd57548bdf0
contract BalancerFlashLoanRecipient is IFlashLoanRecipient {
    using SafeERC20 for IERC20;

    IVault private immutable _vault;
    address private immutable _owner;

    constructor() {
        require(msg.sender != address(0), "invalid address");

        _owner = msg.sender;
        _vault = IVault(0xBA12222222228d8Ba445958a75a0704d566BF2C8);
    }

    modifier onlyOwner() {
        _checkOwner();
        _;
    }

    function _checkOwner() internal view virtual {
        require(msg.sender == _owner, "You are not the owner");
    }

    /**
     * Unsafe increment gas cost optimization
     */
    function unsafeInc(uint256 x) private pure returns (uint256) {
        unchecked {
            return x + 1;
        }
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
        for (uint256 i = 0; i < tokens.length; i = unsafeInc(i)) {
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

        for (uint256 i = 0; i < tokens.length; i = unsafeInc(i)) {
            address tokenContract = tokens[i];
            uint256 amount = amounts[i];

            IERC20(tokenContract).transfer(msg.sender, amount);
        }
    }

    /*
        You should use `chainSwaps` only in tests or simulations or estimate gas
    */
    function doMultiSwap(
        OneSwapInfo[] memory swaps,
        bool chainSwaps,
        bool returnOutput
    ) private returns (uint256[] memory amountsOut) {
        if (chainSwaps && swaps.length < 2) {
            revert("ChainSwap requires at least 2 swaps");
        }

        bool hasError = false;
        string memory errorReason;

        if (returnOutput) {
            amountsOut = new uint256[](swaps.length);
        }

        uint256 curAmountIn;
        uint256 curAmountOut;
        for (uint256 i = 0; i < swaps.length; i = unsafeInc(i)) {
            OneSwapInfo memory curSwap = swaps[i];

            require(curSwap.Router != address(0), "Router is null");
            require(curSwap.Path.length > 0, "Path is null");
            require(curSwap.AmountIn > 0, "AmountIn == 0");

            IERC20 tIn20 = IERC20(curSwap.TokenIn);
            uint256 curBalance = tIn20.balanceOf(address(this));

            if (curSwap.AmountIn > curBalance) {
                revert MultiSwapInsufficientFundsToSwapError(
                    i,
                    curSwap.TokenIn,
                    curBalance,
                    curSwap.AmountIn
                );
            }

            // https://github.com/foundry-rs/foundry/issues/6459
            tIn20.safeIncreaseAllowance(curSwap.Router, curSwap.AmountIn);

            if (curSwap.Deadline == 0) {
                curSwap.Deadline = block.timestamp;
            }

            if (chainSwaps) {
                if (i == 0) {
                    // If this is the first swap in the chain
                    curAmountIn = curSwap.AmountIn;
                    curAmountOut = 0;
                } else if (i == swaps.length - 1) {
                    // If this is the last swap
                    curAmountIn = curAmountOut;
                    curAmountOut = curSwap.AmountOutMin;
                } else {
                    // If this is a middle swap
                    curAmountIn = curAmountOut;
                    curAmountOut = 0;
                }
            } else {
                curAmountIn = curSwap.AmountIn;
                curAmountOut = curSwap.AmountOutMin;
            }

            if (curSwap.Protocol == AmmProtocol.UniswapV2) {
                address[] memory pathDecoded = abi.decode(
                    curSwap.Path,
                    (address[])
                );

                (curAmountOut, hasError, errorReason) = SwapUtils
                    .swapExactTokensForTokensUniswapV2(
                        curSwap.Router,
                        pathDecoded,
                        curAmountIn,
                        curAmountOut,
                        curSwap.Deadline
                    );
            } else if (curSwap.Protocol == AmmProtocol.UniswapV3) {
                (curAmountOut, hasError, errorReason) = SwapUtils
                    .swapExactInputUniswapV3(
                        curSwap.Router,
                        curSwap.Path,
                        curAmountIn,
                        curAmountOut,
                        curSwap.Deadline
                    );
            } else {
                revert NotSupportedAmmProtocolError(curSwap.Protocol);
            }

            if (hasError) {
                revert MultiSwapError(i, errorReason);
            }

            if (returnOutput) {
                amountsOut[i] = curAmountOut;
            }
        }

        return amountsOut;
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

        (OneSwapInfo[] memory swaps, bool chainSwaps, bool returnOutput) = abi.decode(
            userData,
            (OneSwapInfo[], bool, bool)
        );

        // Do swaps
        uint256[] memory amountsOut = doMultiSwap(swaps, chainSwaps, returnOutput);

        // Repay
        uint256 amountToPayback = amounts[0] + feeAmounts[0];
        uint256 lastAmountOut = amountsOut[swaps.length - 1];
        // TODO: Should check if the input token is the last token so the amount check are valid
        //       if not then that a vulnerability to let the contract pay the loan
        if (amountToPayback > lastAmountOut) {
            revert InsufficientFundsToRepayLoanError(
                address(tokens[0]),
                lastAmountOut,
                amountToPayback
            );
        }

        address payable vault = payable(msg.sender); // same as vault
        require(tokens[0].transfer(vault, amountToPayback), "repay failed");
    }
}
