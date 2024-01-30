use std::{fs, path::Path};

use ethers_core::abi::Abi;

#[derive(Clone)]
pub struct ABI {
    pub erc20: Abi,
    pub weth: Abi,
    pub uniswap_v2_factory: Abi,
    pub uniswap_v2_router_json: Abi,
    pub uniswap_v2_pair: Abi,
}

impl ABI {
    pub fn new(abi_folder_path: &Path) -> Self {
        let erc20_json: String = fs::read_to_string(abi_folder_path.join("ERC20.json")).unwrap();
        let weth_json: String = fs::read_to_string(abi_folder_path.join("WETH.json")).unwrap();
        let uniswap_v2_factory_json: String = fs::read_to_string(abi_folder_path.join("UniswapV2Factory.json")).unwrap();
        let uniswap_v2_router_json: String = fs::read_to_string(abi_folder_path.join("UniswapV2Router.json")).unwrap();
        let uniswap_v2_pair_json: String = fs::read_to_string(abi_folder_path.join("UniswapV2Pair.json")).unwrap();

        Self {
            erc20: serde_json::from_str(&erc20_json).unwrap(),
            weth: serde_json::from_str(&weth_json).unwrap(),
            uniswap_v2_factory: serde_json::from_str(&uniswap_v2_factory_json).unwrap(),
            uniswap_v2_router_json: serde_json::from_str(&uniswap_v2_router_json).unwrap(),
            uniswap_v2_pair: serde_json::from_str(&uniswap_v2_pair_json).unwrap(),
        }
    }
}
