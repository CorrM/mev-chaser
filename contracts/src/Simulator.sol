// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.23;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {OneSwapInfo, Swapper, AmmProtocol, NotSupportedAmmProtocolError} from "./Swapper.sol";

error EmptyPath(string argName);

contract Simulator {
    using SafeERC20 for IERC20;

    function simulateMultiSwap(
        OneSwapInfo[] memory swaps,
        bool chainSwaps
    ) external returns (uint256) {
        uint256[] memory amountsOut = Swapper.multiSwap(swaps, chainSwaps, false);
        return amountsOut[amountsOut.length - 1];
    }

    function simulateGetAmountsOut(
        AmmProtocol protocol,
        address contractAddress,
        bytes memory path,
        uint256 amountIn
    ) external returns (uint256) {
        if (path.length == 0) {
            revert EmptyPath("path");
        }

        if (protocol == AmmProtocol.UniswapV2) {
            address[] memory pathDecoded = abi.decode(
                path,
                (address[])
            );
            return Swapper.getAmountsOutUniswapV2(contractAddress, pathDecoded, amountIn);
        } else if (protocol == AmmProtocol.UniswapV3) {
            return Swapper.getAmountsOutUniswapV3(contractAddress, path, amountIn);
        }

        revert NotSupportedAmmProtocolError(protocol);
    }
}
