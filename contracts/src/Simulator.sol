// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.23;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {OneSwapInfo, Swapper} from "./Swapper.sol";

contract Simulator {
    using SafeERC20 for IERC20;

    function simulateMultiSwap(
        OneSwapInfo[] memory swaps,
        bool chainSwaps
    ) external returns (uint256) {
        uint256[] memory amountsOut = Swapper.multiSwap(swaps, chainSwaps, false);
        return amountsOut[amountsOut.length - 1];
    }
}
