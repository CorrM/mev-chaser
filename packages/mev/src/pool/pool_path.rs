use amm::UniswapV2Simulator;
use ethers_core::types::U256;

use super::PoolPathItem;

pub struct PoolPath {
    path: Vec<PoolPathItem>,
}

impl PoolPath {
    pub fn new(path: Vec<PoolPathItem>) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Vec<PoolPathItem> {
        &self.path
    }

    pub fn simulate_v2_path(
        &self,
        count_in: U256,
    ) -> Option<U256> {
        let input_path_item: &PoolPathItem = &self.path[0];
        let token_in_decimals: u8 = if input_path_item.zero_are_input {
            input_path_item.pool.read().unwrap().token0().decimals()
        } else {
            input_path_item.pool.read().unwrap().token1().decimals()
        };
        let unit: U256 = U256::from(10).pow(U256::from(token_in_decimals));
        let mut amount_out: U256 = count_in * unit;

        for path_item in &self.path {
            let reserve0: U256 = path_item.pool.read().unwrap().reserve0();
            let reserve1: U256 = path_item.pool.read().unwrap().reserve1();

            let reserve_in: U256;
            let reserve_out: U256;
            if path_item.zero_are_input {
                reserve_in = reserve0;
                reserve_out = reserve1;
            } else {
                reserve_in = reserve1;
                reserve_out = reserve0;
            }

            //let fee: U256 = U256::from(path_item.pool.read().unwrap().dex().fees);
            let fee: U256 = U256::from(300);
            amount_out = UniswapV2Simulator::get_amount_out(amount_out, reserve_in, reserve_out, fee)?;
        }

        Some(amount_out)
    }
}