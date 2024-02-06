Good idea, Create a MEV contract for users to use with small fee, and to optmize the OneSwapInfo struct make a map that store routers as Map<uint32, CUSTOM_STRUCT>
CUSTOM_STRUCT are just (RouterName, RouterAddress) and make a function to get the key by router name

## Foundry

**Foundry is a blazing fast, portable and modular toolkit for Ethereum application development written in Rust.**

Foundry consists of:

- **Forge**: Ethereum testing framework (like Truffle, Hardhat and DappTools).
- **Cast**: Swiss army knife for interacting with EVM smart contracts, sending transactions and getting chain data.
- **Anvil**: Local Ethereum node, akin to Ganache, Hardhat Network.
- **Chisel**: Fast, utilitarian, and verbose solidity REPL.

## Documentation

https://book.getfoundry.sh/

## Usage

### Build

```shell
$ forge build
```

### Test

```shell
$ FORK_URL=https://polygon-mainnet.g.alchemy.com/v2/{API_KEY}
$ forge test -vv --gas-report --fork-url $FORK_URL --match-path test/BalancerFlashLoanRecipient.t.sol
```

### Format

```shell
$ forge fmt
```

### Gas Snapshots

```shell
$ forge snapshot
```

### Anvil

```shell
$ anvil
```

### Deploy

```shell
$ forge script script/Counter.s.sol:CounterScript --rpc-url <your_rpc_url> --private-key <your_private_key>
```

### Cast

```shell
$ cast <subcommand>
```

### Help

```shell
$ forge --help
$ anvil --help
$ cast --help
```
