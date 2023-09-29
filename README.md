# `ERC-20` Contracts [Substreams](https://substreams.streamingfast.io)


> Extends [ERC-20 Balance Changes](https://github.com/streamingfast/substreams-erc20-balance-changes) with Token Contract information.

## Quickstart

```
$ gh repo clone pinax-network/substreams-erc20-contracts
$ cd substreams-erc20-contracts
$ make
$ make gui
```

## Releases `.spkg`

- https://github.com/pinax-network/substreams-erc20-contracts/releases

## References
- [Ethereum Docs: ERC-20 Token Standard](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/)
- [EIPS: ERC-20 Token Standard ](https://eips.ethereum.org/EIPS/eip-20)
- [OpenZeppelin implementation](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/9b3710465583284b8c4c5d2245749246bb2e0094/contracts/token/ERC20/ERC20.sol)
- [ConsenSys implementation](https://github.com/ConsenSys/Tokens/blob/fdf687c69d998266a95f15216b1955a4965a0a6d/contracts/eip20/EIP20.sol)

## Map Outputs

### `map_token_supply`

```json
{
  "items": [
    {
      "address": "dac17f958d2ee523a2206206994597c13d831ec7",
      "name": "TetherUSD",
      "symbol": "USDT",
      "decimals": 6
    },
    {
      "address": "6b175474e89094c44da98b954eedeac495271d0f",
      "name": "DaiStablecoin",
      "symbol": "DAI",
      "decimals": 18
    },
    ...
  ]
}
```

### Mermaid graph

```mermaid
graph TD;
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_balance_changes;
  map_balance_changes --> map_valid_balance_changes;
  store_known_contract[store: store_known_contract];
  map_valid_balance_changes --> store_known_contract;
  map_contract_info[map: map_contract_info];
  store_known_contract -- deltas --> map_contract_info;
  graph_out[map: graph_out];
  map_contract_info --> graph_out;
```

### Modules

```yaml
Package name: erc20_contract_info
Version: v0.1.0
Doc: ERC-20 Token Contract Info
Modules:
----
Name: store_known_contract
Initial block: 0
Kind: store
Value Type: string
Update Policy: UPDATE_POLICY_SET_IF_NOT_EXISTS
Hash: b7c1419bf3ea6b80e77dada14df6e21e6d62788c
Doc: Stores known contracts

Name: map_contract_info
Initial block: 0
Kind: map
Output Type: proto:erc20.contract.types.v1.Infos
Hash: e8604040f9b9dc4898aa255745eb7d9b1b081187
Doc: Extracts ERC20 token name, symbol and decimals

Name: graph_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: 6bec7f920834700e2761585e235d60e98e0ac9c2
```
