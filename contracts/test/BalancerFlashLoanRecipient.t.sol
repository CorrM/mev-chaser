// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.23;

import {Test, console2} from "forge-std/Test.sol";
import "../src/BalancerFlashLoanRecipient.sol";

contract BalancerFlashLoanRecipientTest is Test {
    BalancerFlashLoanRecipient private flashLoan;
    address private uniswapV2Router;
    address private uniswapV3Router;
    address private USDT;
    address private USDC;
    address private WETH;

    function setUp() public {
        flashLoan = new BalancerFlashLoanRecipient();
        uniswapV2Router = 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D; // UniswapV2 Ethereum
        uniswapV3Router = 0xE592427A0AEce92De3Edee1F18E0157C05861564; // UniswapV3 Polygon
        USDT = 0xdAC17F958D2ee523a2206206994597C13D831ec7;
        USDC = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;
        WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
    }

    function test_uniswapv2_one_path() public {
        address[] memory addresses = new address[](2);
        addresses[0] = USDT;
        addresses[1] = USDC;

        bytes memory path = abi.encode(addresses);
        OneSwapInfo[] memory swaps = new OneSwapInfo[](1);
        swaps[0] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            AmountIn: 20 * (10 ** 6),
            AmountOutMin: 19 * (10 ** 6),
            Router: uniswapV2Router,
            TokenIn: USDT,
            Deadline: 0,
            Path: path
        });

        //vm.expectRevert();
        flashLoan.getLoanThenMultiSwap(swaps, false);
    }

    function test_uniswapv2_multi_path() public {
        address[] memory addresses = new address[](3);
        addresses[0] = USDT;
        addresses[1] = USDC;
        addresses[2] = WETH;

        bytes memory path = abi.encode(addresses);
        OneSwapInfo[] memory swaps = new OneSwapInfo[](1);
        swaps[0] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            AmountIn: 20 * (10 ** 6),
            AmountOutMin: 19 * (10 ** 6),
            Router: uniswapV2Router,
            TokenIn: USDT,
            Deadline: 0,
            Path: path
        });

        //vm.expectRevert();
        flashLoan.getLoanThenMultiSwap(swaps, false);
    }

    /*
    function test_getLoanThenSwapChain_uniswap2ToUniswap2_swapFail() public {
        OneSwapInfo[] memory swaps = new OneSwapInfo[](2);

        swaps[0] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            Router: uniswapV2Router,
            TokenIn: PolygonUSDT,
            TokenOut: PolygonUSDC,
            AmountIn: 20 * (10 ** 6),
            AmountOutMin: 19 * (10 ** 6),
            Deadline: 0
        });

        swaps[1] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            Router: uniswapV2Router,
            TokenIn: PolygonUSDC,
            TokenOut: PolygonUSDT,
            AmountIn: 19 * (10 ** 6),
            AmountOutMin: 20 * (10 ** 6),
            Deadline: 0,
            Data: ""
        });

        vm.expectRevert(
            abi.encodeWithSelector(SwapChainError.selector, 1, "UniswapV2Router: INSUFFICIENT_OUTPUT_AMOUNT")
        );
        flashLoan.getLoanThenSwapChain(LoanProvider.Balancer, swaps);
    }

    function test_getLoanThenSwapChain_uniswap2ToUniswap2_repayfall() public {
        OneSwapInfo[] memory swaps = new OneSwapInfo[](2);

        swaps[0] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            Router: uniswapV2Router,
            TokenIn: PolygonUSDT,
            TokenOut: PolygonUSDC,
            AmountIn: 20 * (10 ** 6),
            AmountOutMin: 19 * (10 ** 6),
            Deadline: 0,
            Data: ""
        });

        swaps[1] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            Router: uniswapV2Router,
            TokenIn: PolygonUSDC,
            TokenOut: PolygonUSDT,
            AmountIn: 19 * (10 ** 6),
            AmountOutMin: 18 * (10 ** 6),
            Deadline: 0,
            Data: ""
        });

        vm.expectRevert(InsufficientFundsToRepayLoanError.selector);
        flashLoan.getLoanThenSwapChain(LoanProvider.Balancer, swaps);
    }

    function test_getLoanThenSwapChain_uniswap2ToUniswap3_swapFail() public {
        OneSwapInfo[] memory swaps = new OneSwapInfo[](2);

        swaps[0] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            Router: uniswapV2Router,
            TokenIn: PolygonUSDT,
            TokenOut: PolygonUSDC,
            AmountIn: 20 * (10 ** 6),
            AmountOutMin: 19 * (10 ** 6),
            Deadline: 0,
            Data: ""
        });

        swaps[1] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV3,
            Router: uniswapV3Router,
            TokenIn: PolygonUSDC,
            TokenOut: PolygonUSDT,
            AmountIn: 19 * (10 ** 6),
            AmountOutMin: 18 * (10 ** 6),
            Deadline: 0,
            Data: abi.encode(uint24(100)) // Fee
        });

        vm.expectRevert(InsufficientFundsToRepayLoanError.selector);
        flashLoan.getLoanThenSwapChain(LoanProvider.Balancer, swaps);
    }

    function test_getLoanThenSwapChain_uniswap3ToUniswap3_swapFail() public {
        OneSwapInfo[] memory swaps = new OneSwapInfo[](2);

        // TODO: Wrong fees here (100) will case for infinity loop

        swaps[0] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV3,
            Router: uniswapV3Router,
            TokenIn: 0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619,
            TokenOut: 0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063,
            AmountIn: 1 * (10 ** 18),
            AmountOutMin: 2180 * (10 ** 18),
            Deadline: 0,
            Data: abi.encode(uint24(3000)) // Fee
        });

        swaps[1] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV3,
            Router: uniswapV3Router,
            TokenIn: 0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063,
            TokenOut: 0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619,
            AmountIn: 2180 * (10 ** 6),
            AmountOutMin: 1 * (10 ** 6),
            Deadline: 0,
            Data: abi.encode(uint24(3000)) // Fee
        });

        //vm.expectRevert(InsufficientFundsToRepayLoan.selector);
        flashLoan.getLoanThenSwapChain(LoanProvider.Balancer, swaps);
    }
    */

    // function testFuzz_SetNumber(uint256 x) public {
    //     counter.setNumber(x);
    //     assertEq(counter.number(), x);
    // }
}
