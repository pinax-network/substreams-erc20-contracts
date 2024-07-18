# `ERC-20` Contracts [Substreams](https://substreams.streamingfast.io)

> Extends [ERC-20 Balance Changes](https://github.com/streamingfast/substreams-erc20-balance-changes) with Token Contract information.

### First block containing ERC20 Transfers

| Chain   | Block number |
| ------- | ------------ |
| ETH     | 913 198      |
| Polygon | 2764         |
| BSC     | 64 253       |

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
  store_known_contracts[store: store_known_contracts];
  balance_changes:map_valid_balance_changes --> store_known_contracts;
  store_known_contracts -- deltas --> index_contracts;
  map_contracts[map: map_contracts];
  store_known_contracts -- deltas --> map_contracts;
  graph_out[map: graph_out];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> graph_out;
  map_contracts --> graph_out;
  db_out[map: db_out];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> db_out;
  map_contracts --> db_out;
```

### Modules

Name: store_known_contracts
Initial block: 913198
Kind: store
Input: map: balance_changes:map_valid_balance_changes
Value Type: string
Update Policy: set_if_not_exists
Hash: 8ac2282166033a9f121eb98942dc1974ba948fec
Doc: Stores known contracts

Name: index_contracts
Initial block: 913198
Kind: index
Input: store: store_known_contracts
Output Type: proto:sf.substreams.index.v1.Keys
Hash: 481e8077e33f9725b91d57a06582baeb48451496

Name: map*contracts
Initial block: 913198
Kind: map
Input: store: store_known_contracts
Block Filter: (using \_index_contracts*): `&{contracts}`
Output Type: proto:erc20.contracts.types.v1.Contracts
Hash: 5def69ded574606c05f04580fe43290eab5eda05
Doc: Extracts ERC20 token name, symbol and decimals

Name: graph_out
Initial block: 913198
Kind: map
Input: source: sf.substreams.v1.Clock
Input: map: map_contracts
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: dbe5a2a86b5cc116dbdb6433666ce24ab3e6c1b4

Name: db_out
Initial block: 913198
Kind: map
Input: source: sf.substreams.v1.Clock
Input: map: map_contracts
Output Type: proto:sf.substreams.sink.database.v1.DatabaseChanges
Hash: 8b7dd497e217476bd3351a1b19f03e15f6dee3a6

```yaml
Package name: erc20_contracts
Version: v0.1.0
Doc: ERC-20 Token Contract Info
Modules: ----
```
