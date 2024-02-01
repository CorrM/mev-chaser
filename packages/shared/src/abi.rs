use std::{fs, path::Path};

use anyhow::Result;
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
    pub fn new(abi_folder_path: &Path) -> Result<Self> {
        let erc20_json: String = fs::read_to_string(abi_folder_path.join("ERC20.json"))?;
        let weth_json: String = fs::read_to_string(abi_folder_path.join("WETH.json"))?;
        let uniswap_v2_factory_json: String = fs::read_to_string(abi_folder_path.join("UniswapV2Factory.json"))?;
        let uniswap_v2_router_json: String = fs::read_to_string(abi_folder_path.join("UniswapV2Router.json"))?;
        let uniswap_v2_pair_json: String = fs::read_to_string(abi_folder_path.join("UniswapV2Pair.json"))?;

        Ok(Self {
            erc20: serde_json::from_str(&erc20_json)?,
            weth: serde_json::from_str(&weth_json)?,
            uniswap_v2_factory: serde_json::from_str(&uniswap_v2_factory_json)?,
            uniswap_v2_router_json: serde_json::from_str(&uniswap_v2_router_json)?,
            uniswap_v2_pair: serde_json::from_str(&uniswap_v2_pair_json)?,
        })
    }
}
