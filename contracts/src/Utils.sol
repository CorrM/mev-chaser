// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.23;

library Utils {
    /**
     * Unsafe increment gas cost optimization
     */
    function unsafeInc(uint256 x) internal pure returns (uint256) {
        unchecked {
            return x + 1;
        }
    }
}