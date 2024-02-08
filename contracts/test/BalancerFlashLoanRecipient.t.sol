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
        uniswapV2Router = 0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506; // sushiV2 Polygon
        uniswapV3Router = 0xE592427A0AEce92De3Edee1F18E0157C05861564; // UniswapV3 Polygon
        USDT = 0xc2132D05D31c914a87C6611C10748AEb04B58e8F;
        USDC = 0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174;
        WETH = 0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619;
    }

    function test_tokenFeeOnTransferTokens_shouldFail() public {
        OneSwapInfo[] memory swaps = new OneSwapInfo[](4);

        address[] memory swapAddresses0 = new address[](2);
        swapAddresses0[0] = address(0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063);
        swapAddresses0[1] = address(0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270);
        bytes memory path0 = abi.encode(swapAddresses0);
        swaps[0] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            Router: address(0xC0788A3aD43d79aa53B09c2EaCc313A787d1d607),
            TokenIn: swapAddresses0[0],
            Path: path0,
            AmountIn: 10000000000000000000,
            AmountOutMin: 0,
            Deadline: 0
        });

        address[] memory swapAddresses1 = new address[](2);
        swapAddresses1[0] = address(0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270);
        swapAddresses1[1] = address(0x5eF8aAa4338086d4517E6E486BF0483F21443a27);
        bytes memory path1 = abi.encode(swapAddresses1);
        swaps[1] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            Router: address(0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff),
            TokenIn: swapAddresses1[0],
            Path: path1,
            AmountIn: 0,
            AmountOutMin: 0,
            Deadline: 0
        });

        address[] memory swapAddresses2 = new address[](2);
        swapAddresses2[0] = address(0x5eF8aAa4338086d4517E6E486BF0483F21443a27);
        swapAddresses2[1] = address(0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270);
        bytes memory path2 = abi.encode(swapAddresses2);
        swaps[2] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            Router: address(0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff),
            TokenIn: swapAddresses2[0],
            Path: path2,
            AmountIn: 0,
            AmountOutMin: 0,
            Deadline: 0
        });

        address[] memory swapAddresses3 = new address[](2);
        swapAddresses3[0] = address(0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270);
        swapAddresses3[1] = address(0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063);
        bytes memory path3 = abi.encode(swapAddresses3);
        swaps[3] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            Router: address(0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff),
            TokenIn: swapAddresses3[0],
            Path: path3,
            AmountIn: 0,
            AmountOutMin: 1000000000000000000,
            Deadline: 0
        });

        //vm.expectRevert();
        flashLoan.getLoanThenMultiSwap(swaps, true, false);
    }

    function test_uniswapv2MultiSwap() public {
        OneSwapInfo[] memory swaps = new OneSwapInfo[](4);

        address[] memory swapAddresses0 = new address[](2);
        swapAddresses0[0] = address(0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063);
        swapAddresses0[1] = address(0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270);
        bytes memory path0 = abi.encode(swapAddresses0);
        swaps[0] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            Router: address(0xC0788A3aD43d79aa53B09c2EaCc313A787d1d607),
            TokenIn: swapAddresses0[0],
            Path: path0,
            AmountIn: 10000000000000000000,
            AmountOutMin: 0,
            Deadline: 0
        });

        address[] memory swapAddresses1 = new address[](2);
        swapAddresses1[0] = address(0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270);
        swapAddresses1[1] = address(0xc2132D05D31c914a87C6611C10748AEb04B58e8F);
        bytes memory path1 = abi.encode(swapAddresses1);
        swaps[1] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            Router: address(0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff),
            TokenIn: swapAddresses1[0],
            Path: path1,
            AmountIn: 0,
            AmountOutMin: 0,
            Deadline: 0
        });

        address[] memory swapAddresses2 = new address[](2);
        swapAddresses2[0] = address(0xc2132D05D31c914a87C6611C10748AEb04B58e8F);
        swapAddresses2[1] = address(0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270);
        bytes memory path2 = abi.encode(swapAddresses2);
        swaps[2] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            Router: address(0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff),
            TokenIn: swapAddresses2[0],
            Path: path2,
            AmountIn: 0,
            AmountOutMin: 0,
            Deadline: 0
        });

        address[] memory swapAddresses3 = new address[](2);
        swapAddresses3[0] = address(0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270);
        swapAddresses3[1] = address(0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063);
        bytes memory path3 = abi.encode(swapAddresses3);
        swaps[3] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            Router: address(0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff),
            TokenIn: swapAddresses3[0],
            Path: path3,
            AmountIn: 0,
            AmountOutMin: 1000000000000000000,
            Deadline: 0
        });

        //vm.expectRevert();
        flashLoan.getLoanThenMultiSwap(swaps, true, false);
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
            AmountOutMin: 0,
            Router: uniswapV2Router,
            TokenIn: USDT,
            Deadline: 0,
            Path: path
        });

        //vm.expectRevert();
        flashLoan.getLoanThenMultiSwap(swaps, false, false);
    }
    
    function test_uniswapv2_multi_path() public {
        address[] memory addresses = new address[](4);
        addresses[0] = USDT;
        addresses[1] = USDC;
        addresses[2] = WETH;
        addresses[3] = USDT;

        bytes memory path = abi.encode(addresses);
        OneSwapInfo[] memory swaps = new OneSwapInfo[](1);
        swaps[0] = OneSwapInfo({
            Protocol: AmmProtocol.UniswapV2,
            AmountIn: 20 * (10 ** 6),
            AmountOutMin: 0,
            Router: uniswapV2Router,
            TokenIn: USDT,
            Deadline: 0,
            Path: path
        });

        //vm.expectRevert();
        flashLoan.getLoanThenMultiSwap(swaps, false, false);
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
