use std::str::FromStr;
use std::sync::Arc;
use std::usize;

use anyhow::Result;
use ethers::{
    abi::AbiDecode,
    addressbook::Address,
    middleware::Middleware,
    prelude::Bytes,
    types::transaction::eip2718::TypedTransaction,
    types::BigEndianHash,
    types::TransactionRequest,
    types::{H256, U256},
};

use vidger::utilities::block_on;

fn read_address(address: H256) -> Option<Address> {
    let implementation = Address::from(address);
    if implementation == Address::zero() {
        return None;
    }

    Some(implementation)
}

fn parse1167bytecode(code: String) -> Option<Address> {
    const EIP_1167_BYTECODE_PREFIX: &str = "0x363d3d373d3d3d363d";
    const EIP_1167_BYTECODE_SUFFIX: &str = "57fd5bf3";
    const SUFFIX_OFFSET_FROM_ADDRESS_END: usize = 22;

    if !code.starts_with(EIP_1167_BYTECODE_PREFIX) {
        return None;
    }

    // detect length of address (20 bytes non-optimized, 0 < N < 20 bytes for vanity addresses)
    let push_nhex: &str = &code[EIP_1167_BYTECODE_PREFIX.len()..EIP_1167_BYTECODE_PREFIX.len() + 2];

    // push1 ... push20 use opcodes 0x60 ... 0x73
    let address_length: usize = usize::from_str_radix(push_nhex, 16).unwrap() - 0x5f;

    if address_length < 1 || address_length > 20 {
        return None;
    }

    if !code[EIP_1167_BYTECODE_PREFIX.len() + 2 + address_length * 2 + SUFFIX_OFFSET_FROM_ADDRESS_END..]
        .starts_with(EIP_1167_BYTECODE_SUFFIX)
    {
        return None;
    }

    // address length is in bytes, 2 hex chars make up 1 byte
    let address_from_bytecode: &str =
        &code[EIP_1167_BYTECODE_PREFIX.len()..EIP_1167_BYTECODE_PREFIX.len() + 2 + address_length * 2];

    // padStart is needed for vanity addresses
    Some(Address::from_str(&format!("{:0>40}", address_from_bytecode)).unwrap())
}

/// EIP-1167 minimal proxy
fn is_eip_1167_minimal_proxy<M: Middleware + 'static>(provider: &Arc<M>, token: Address) -> Result<Option<Address>> {
    let code: Bytes = block_on(provider.get_code(token, None))?;
    let Some(eip1167_address) = parse1167bytecode(code.to_string()) else {
        return Ok(None);
    };

    Ok(Some(eip1167_address))
}

/// EIP-1967 direct proxy
fn is_eip_1967_direct_proxy<M: Middleware + 'static>(provider: &Arc<M>, token: Address) -> Result<Option<Address>> {
    let eip_1967_logic_slot: U256 =
        U256::from_str("0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc")?;

    let address: H256 = block_on(provider.get_storage_at(token, H256::from_uint(&eip_1967_logic_slot), None))?;
    Ok(read_address(address))
}

/// EIP-1967 beacon proxy
fn is_eip_1967_beacon_proxy<M: Middleware + 'static>(provider: &Arc<M>, token: Address) -> Result<Option<Address>> {
    let eip_1967_beacon_slot: U256 =
        U256::from_str("0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50")?;
    let eip_1167_beacon_methods: [Bytes; 2] = [
        Bytes::from_str("0x5c60da1b00000000000000000000000000000000000000000000000000000000")?,
        Bytes::from_str("0xda52571600000000000000000000000000000000000000000000000000000000")?,
    ];

    let address: H256 = block_on(provider.get_storage_at(token, H256::from_uint(&eip_1967_beacon_slot), None))?;
    let Some(beacon_address) = read_address(address) else {
        return Ok(None);
    };

    for method_bytes in eip_1167_beacon_methods {
        let simulator_tx: TypedTransaction = TransactionRequest::default()
            .to(beacon_address)
            .data(method_bytes.clone())
            .into();

        let result: Bytes = block_on(provider.call(&simulator_tx, None))?;
        if let Ok(result) = H256::decode(&result.0) {
            return Ok(read_address(result));
        };
    }

    Ok(None)
}

/// OpenZeppelin proxy pattern
fn is_open_zeppelin_proxy<M: Middleware + 'static>(provider: &Arc<M>, token: Address) -> Result<Option<Address>> {
    let open_zeppelin_slot: U256 =
        U256::from_str("0x7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c3")?;

    let address: H256 = block_on(provider.get_storage_at(token, H256::from_uint(&open_zeppelin_slot), None))?;
    Ok(read_address(address))
}

/// EIP-1822 Universal Upgradeable Proxy Standard
fn is_eip_1822_universal_upgradeable_proxy<M: Middleware + 'static>(
    provider: &Arc<M>,
    token: Address,
) -> Result<Option<Address>> {
    let eip_1822_logic_slot: U256 =
        U256::from_str("0xc5f16f0fcc639fa48a6947836d9850f504798523bf8c9a3a87d5876cf622bcf7")?;

    let address: H256 = block_on(provider.get_storage_at(token, H256::from_uint(&eip_1822_logic_slot), None))?;
    Ok(read_address(address))
}

/// EIP-897 DelegateProxy pattern
fn is_eip_897_delegate_proxy<M: Middleware + 'static>(provider: &Arc<M>, token: Address) -> Result<Option<Address>> {
    let eip_897_interface: [Bytes; 1] = [Bytes::from_str(
        "0x5c60da1b00000000000000000000000000000000000000000000000000000000",
    )?];

    let simulator_tx: TypedTransaction = TransactionRequest::default()
        .to(token)
        .data(eip_897_interface[0].clone())
        .into();

    let result: Bytes = block_on(provider.call(&simulator_tx, None))?;
    let Ok(address) = H256::decode(&result.0) else {
        return Ok(None);
    };

    Ok(read_address(address))
}

/// GnosisSafeProxy proxy
fn is_gnosis_safe_proxy<M: Middleware + 'static>(provider: &Arc<M>, token: Address) -> Result<Option<Address>> {
    let gnosis_safe_proxy_interface: [Bytes; 1] = [Bytes::from_str(
        "0xa619486e00000000000000000000000000000000000000000000000000000000",
    )?];

    let simulator_tx: TypedTransaction = TransactionRequest::default()
        .to(token)
        .data(gnosis_safe_proxy_interface[0].clone())
        .into();

    let result: Bytes = block_on(provider.call(&simulator_tx, None))?;
    let Ok(address) = H256::decode(&result.0) else {
        return Ok(None);
    };

    Ok(read_address(address))
}

/// Comptroller proxy
fn is_comptroller_proxy<M: Middleware + 'static>(provider: &Arc<M>, token: Address) -> Result<Option<Address>> {
    let comptroller_proxy_interface: [Bytes; 1] = [Bytes::from_str(
        "0xbb82aa5e00000000000000000000000000000000000000000000000000000000",
    )?];

    let simulator_tx: TypedTransaction = TransactionRequest::default()
        .to(token)
        .data(comptroller_proxy_interface[0].clone())
        .into();

    let result: Bytes = block_on(provider.call(&simulator_tx, None))?;
    let Ok(address) = H256::decode(&result.0) else {
        return Ok(None);
    };

    Ok(read_address(address))
}

pub fn get_proxy_implementation<M: Middleware + 'static>(provider: &Arc<M>, token: Address) -> Result<Option<Address>> {
    // adapted from: https://github.com/gnosis/evm-proxy-detection/blob/main/src/index.ts

    // EIP-1167 minimal proxy
    let proxy_address: Option<Address> = is_eip_1167_minimal_proxy(provider, token)?;
    if proxy_address.is_some() {
        return Ok(proxy_address);
    }

    // EIP-1967 direct proxy
    let proxy_address: Option<Address> = is_eip_1967_direct_proxy(provider, token)?;
    if proxy_address.is_some() {
        return Ok(proxy_address);
    }

    //  EIP-1967 beacon proxy
    let proxy_address: Option<Address> = is_eip_1967_beacon_proxy(provider, token)?;
    if proxy_address.is_some() {
        return Ok(proxy_address);
    }

    // OpenZeppelin proxy
    let proxy_address: Option<Address> = is_open_zeppelin_proxy(provider, token)?;
    if proxy_address.is_some() {
        return Ok(proxy_address);
    }

    // EIP-897 DelegateProxy pattern
    let proxy_address: Option<Address> = is_eip_897_delegate_proxy(provider, token)?;
    if proxy_address.is_some() {
        return Ok(proxy_address);
    }

    // EIP-1822 universal upgradeable proxy
    let proxy_address: Option<Address> = is_eip_1822_universal_upgradeable_proxy(provider, token)?;
    if proxy_address.is_some() {
        return Ok(proxy_address);
    }

    // GnosisSafeProxy proxy
    let proxy_address: Option<Address> = is_gnosis_safe_proxy(provider, token)?;
    if proxy_address.is_some() {
        return Ok(proxy_address);
    }

    // Comptroller proxy
    let proxy_address: Option<Address> = is_comptroller_proxy(provider, token)?;
    if proxy_address.is_some() {
        return Ok(proxy_address);
    }

    Ok(None)
}
