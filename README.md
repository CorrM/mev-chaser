# MevChaser


# Some errors

- [EVM node returns "Missing trie node"](https://support.chainstack.com/hc/en-us/articles/4440543903257-EVM-node-returns-Missing-trie-node)
- When EVM revert with empty data("0x") mostly its abi.encode or abi.decode problem
  - In REVM case it could mean that the contract or the function needs slot value (like fields initialized by ctor)

# Notes

- [async-what-is-blocking](https://ryhl.io/blog/async-what-is-blocking/#the-rayon-crate)
- To get contract storage you can use foundry cast tool
  - `cast storage 0xc4e595acDD7d12feC385E5dA5D43160e8A0bAC0E -r https://polygon-mainnet.infura.io/v3/c230ccbf294b44bcac907f4a719d06c4 -e GX8ARGFMAK1RJTEY2WN79ID3A3PTD4DR6B > /home/corrm/Desktop/cast.txt`
