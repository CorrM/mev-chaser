use std::path::Path;

use ethers_contract::Abigen;

fn gen_abi(contract_name: &str, abi_file_name: &str, dist_file_name: &str) {
    let abi_path = Path::new("../../abi");
    let dist_path = Path::new("./src");

    Abigen::new(contract_name, abi_path.join(abi_file_name).to_str().unwrap())
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file(dist_path.join(dist_file_name).to_str().unwrap())
        .unwrap();
}

fn main() {
    gen_abi("ERC20TokenAbi", "ERC20.json", "erc20_token.rs");
    gen_abi("WethAbi", "WETH.json", "weth.rs");
    gen_abi("UniswapV2FactoryAbi", "UniswapV2Factory.json", "uniswap_v2_factory.rs");
    gen_abi("UniswapV2PairAbi", "UniswapV2Pair.json", "uniswap_v2_pair.rs");
    gen_abi("UniswapV2RouterAbi", "UniswapV2Router.json", "uniswap_v2_router.rs");
}
