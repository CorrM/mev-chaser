// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.23;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IUniswapV2Router02, ISwapRouter} from "./Interfaces.sol";
import {Utils} from "./Utils.sol";

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

// Swap errors
error MultiSwapError(uint256 swapIndex, string errorReason);
error MultiSwapInsufficientFundsToSwapError(
    uint256 swapIndex,
    address token,
    uint256 curBalance,
    uint256 amount
);

library Swapper {
    using SafeERC20 for IERC20;

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
            /*
            amounts[0] = Input token amount
            amounts[1] = path[1] token output
            amounts[.] = path[.] token output
            amounts[amounts.length - 1] = Output token amount
            */
            return (amounts[amounts.length - 1], false, "");
        } catch Error(string memory reason) {
            return (0, true, reason);
        } catch (bytes memory /*reason*/) {
            return (0, true, "Mostly pair not found");
        }
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
        } catch (bytes memory /*reason*/) {
            return (0, true, "Mostly pair not found");
        }
    }

    /*
        You should use `chainSwaps` only in tests or simulations or estimate gas
    */
    function multiSwap(
        OneSwapInfo[] memory swaps,
        bool chainSwaps,
        bool returnEveryAmountsOut
    ) internal returns (uint256[] memory amountsOut) {
        if (chainSwaps && swaps.length < 2) {
            revert("ChainSwap requires at least 2 swaps");
        }

        bool hasError = false;
        string memory errorReason;
        amountsOut = new uint256[](returnEveryAmountsOut ? swaps.length : 1);

        uint256 curAmountIn;
        uint256 curAmountOut;
        for (uint256 i = 0; i < swaps.length; i = Utils.unsafeInc(i)) {
            OneSwapInfo memory curSwap = swaps[i];

            //console2.log("for loop: ", i);

            require(curSwap.Router != address(0), "Router is null");
            require(curSwap.Path.length > 0, "Path is null");
            // require(curSwap.AmountIn > 0, "AmountIn == 0"); // Dont check for that, as `chainSwaps` can be true and `curSwap.AmountIn == 0` in that case

            IERC20 inputToken = IERC20(curSwap.TokenIn);

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
                    //curAmountIn = curAmountOut;
                    curAmountIn = inputToken.balanceOf(address(this));
                    curAmountOut = 0;
                }
            } else {
                curAmountIn = curSwap.AmountIn;
                curAmountOut = curSwap.AmountOutMin;
            }

            //console2.log("curAmountIn: ", curAmountIn);
            //console2.log("curAmountOut: ", curAmountOut);
            //console2.log("token in: ", curSwap.TokenIn);

            //uint256 curBalance = tIn20.balanceOf(address(this));
            //if (curAmountIn > curBalance) {
            //    // If that happens, mostly its a scam token, or it submit fees on transfer
            //    // https://polygonscan.com/tx/0xc0e43f4e8213e009d0f0e78791753593063f638f14e0489d3da396b6b19d5ecc
            //    // it use `swapExactTokensForETHSupportingFeeOnTransferTokens` function
            //    revert MultiSwapInsufficientFundsToSwapError(
            //        i,
            //        curSwap.TokenIn,
            //        curBalance,
            //        curAmountIn
            //    );
            //}

            // https://github.com/foundry-rs/foundry/issues/6459
            inputToken.safeIncreaseAllowance(curSwap.Router, curAmountIn);

            if (curSwap.Deadline == 0) {
                curSwap.Deadline = block.timestamp + 60;
            }

            if (curSwap.Protocol == AmmProtocol.UniswapV2) {
                //console2.log("abi.decode: Before");
                //console2.log(curSwap.Path.length);

                address[] memory pathDecoded = abi.decode(
                    curSwap.Path,
                    (address[])
                );

                //console2.log("abi.decode: After");

                (curAmountOut, hasError, errorReason) = swapExactTokensForTokensUniswapV2(
                        curSwap.Router,
                        pathDecoded,
                        curAmountIn,
                        curAmountOut,
                        curSwap.Deadline
                    );
            } else if (curSwap.Protocol == AmmProtocol.UniswapV3) {
                (curAmountOut, hasError, errorReason) = swapExactInputUniswapV3(
                        curSwap.Router,
                        curSwap.Path,
                        curAmountIn,
                        curAmountOut,
                        curSwap.Deadline
                    );
            } else {
                revert NotSupportedAmmProtocolError(curSwap.Protocol);
            }

            //console2.log("swap out: ", curAmountOut);

            // no need for `safeDecreaseAllowance` as it is done when the router do the swap
            //tIn20.safeDecreaseAllowance(curSwap.Router, curAmountIn);

            if (hasError) {
                revert MultiSwapError(i, errorReason);
            }

            //console2.log("Swap done", i);
            //console2.log("============================");

            if (returnEveryAmountsOut) {
                amountsOut[i] = curAmountOut;
            }
        }

        if (!returnEveryAmountsOut) {
            amountsOut[0] = curAmountOut;
        }

        return amountsOut;
    }
}