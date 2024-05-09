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
  store_known_contract[store: store_known_contract];
  balance_changes:map_valid_balance_changes --> store_known_contract;
  map_contracts[map: map_contracts];
  store_known_contract -- deltas --> map_contracts;
  graph_out[map: graph_out];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> graph_out;
  map_contracts --> graph_out;
  db_out[map: db_out];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> db_out;
  map_contracts --> db_out;
```

### Modules

Name: store_known_contract
Initial block: 913198
Kind: store
Input: map: balance_changes:map_valid_balance_changes
Value Type: string
Update Policy: set_if_not_exists
Hash: 082cd6a7a8898cba7ffba3e74225bf23ce96e376
Doc: Stores known contracts

Name: map_contracts
Initial block: 913198
Kind: map
Input: store: store_known_contract
Output Type: proto:erc20.contracts.types.v1.Contracts
Hash: cd5da8965cb8a33210b0fb609ae081516591bbad
Doc: Extracts ERC20 token name, symbol and decimals

Name: graph_out
Initial block: 913198
Kind: map
Input: source: sf.substreams.v1.Clock
Input: map: map_contracts
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: 724537eb7d68a9475866543b37a2bec57b4738c8

Name: db_out
Initial block: 913198
Kind: map
Input: source: sf.substreams.v1.Clock
Input: map: map_contracts
Output Type: proto:sf.substreams.sink.database.v1.DatabaseChanges
Hash: dbf75233a4abdd8ae90df14cfe91470305b21ac1

```yaml
Package name: erc20_contracts
Version: v0.1.0
Doc: ERC-20 Token Contract Info
Modules: ----
```
