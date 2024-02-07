use ethers::types::U256;

use crate::AmmPool;

pub struct UniswapV2Simulator;

impl UniswapV2Simulator {
    pub fn reserves_to_price(pool: &dyn AmmPool, token0_in: bool) -> f64 {
        let r0 = pool.reserve0().as_u128() as f64;
        let r1 = pool.reserve1().as_u128() as f64;
        let d0 = pool.token0().decimals() as i32;
        let d1 = pool.token1().decimals() as i32;
        let mult = (10.0_f64).powi(d0 - d1);

        if r1 == 0.0 || r0 == 0.0 {
            return 0.0;
        }

        let price = (r1 / r0) * mult;
        if token0_in {
            price
        } else {
            1_f64 / price
        }
    }

    pub fn get_amount_out(amount_in: U256, reserve_in: U256, reserve_out: U256, fee: U256) -> Option<U256> {
        let fee: U256 = fee / U256::from(100);
        let amount_in_with_fee: U256 = amount_in * (U256::from(1000) - fee);
        let numerator: U256 = amount_in_with_fee * reserve_out;
        let denominator: U256 = (reserve_in * 1000) + amount_in_with_fee;
        let ret: Option<U256> = numerator.checked_div(denominator);
        ret
    }
}
