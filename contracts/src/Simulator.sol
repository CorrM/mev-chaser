// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.23;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {MultiSwapError, OneSwapInfo, Swapper, AmmProtocol, NotSupportedAmmProtocolError} from "./Swapper.sol";

contract Simulator {
    using SafeERC20 for IERC20;

    receive() external payable {}
    fallback() external payable {}

    function simulateGetAmountsOutUniswapV2(
        address router,
        address[] memory path,
        uint256 amountIn
    ) external returns (uint256) {
        require(router != address(0), "Invalid router address");
        require(path.length >= 2, "Invalid path length");

        IERC20 inputToken = IERC20(path[0]);
        inputToken.safeIncreaseAllowance(router, amountIn);

        IERC20 outputToken = IERC20(path[path.length - 1]);
        uint256 balanceBefore = outputToken.balanceOf(address(this));

        (, bool hasError, string memory errorReason) = Swapper
            .swapExactTokensForTokensUniswapV2(
                router,
                path,
                amountIn,
                0,
                block.timestamp + 60
            );

        if (hasError) {
            revert MultiSwapError(0, errorReason);
        }

        return outputToken.balanceOf(address(this)) - balanceBefore;
    }

    function simulateGetAmountsOutUniswapV3(
        address router,
        address outputToken,
        bytes memory path,
        uint256 amountIn
    ) external returns (uint256) {
        require(router != address(0), "Invalid router address");
        require(path.length > 0, "Path cannot be empty");

        IERC20 outputTokenErc = IERC20(outputToken);
        uint256 balanceBefore = outputTokenErc.balanceOf(address(this));

        Swapper.swapExactInputUniswapV3(
            router,
            path,
            amountIn,
            0,
            block.timestamp + 60
        );

        return outputTokenErc.balanceOf(address(this)) - balanceBefore;
    }

    function simulateMultiSwap(
        OneSwapInfo[] memory swaps,
        bool chainSwaps
    ) external returns (uint256) {
        require(swaps.length > 0, "Swaps array cannot be empty");
        uint256[] memory amountsOut = Swapper.multiSwap(
            swaps,
            chainSwaps,
            false
        );
        return amountsOut[amountsOut.length - 1];
    }
}
