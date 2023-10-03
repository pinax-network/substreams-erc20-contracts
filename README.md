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
  map_balance_changes[map: map_balance_changes];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> map_balance_changes;
  map_valid_balance_changes[map: map_valid_balance_changes];
  map_balance_changes --> map_valid_balance_changes;
  store_known_contracts[store: store_known_contracts];
  map_valid_balance_changes --> store_known_contracts;
  map_contracts[map: map_contracts];
  store_known_contracts -- deltas --> map_contracts;
  graph_out[map: graph_out];
  map_contracts --> graph_out;
  db_out[map: db_out];
  map_contracts --> db_out;
  balance_changes:map_balance_changes[map: balance_changes:map_balance_changes];
  sf.ethereum.type.v2.Block[source: sf.ethereum.type.v2.Block] --> balance_changes:map_balance_changes;
  balance_changes:map_valid_balance_changes[map: balance_changes:map_valid_balance_changes];
  balance_changes:map_balance_changes --> balance_changes:map_valid_balance_changes;
  balance_changes:map_unknown_balance_changes[map: balance_changes:map_unknown_balance_changes];
  balance_changes:map_balance_changes --> balance_changes:map_unknown_balance_changes;
  balance_changes:db_out[map: balance_changes:db_out];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> balance_changes:db_out;
  balance_changes:map_balance_changes --> balance_changes:db_out;
  balance_changes:graph_out[map: balance_changes:graph_out];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> balance_changes:graph_out;
  balance_changes:map_balance_changes --> balance_changes:graph_out;
  balance_changes:store_valid_balance_changes[store: balance_changes:store_valid_balance_changes];
  balance_changes:map_balance_changes --> balance_changes:store_valid_balance_changes;
  balance_changes:balance_change_stats[map: balance_changes:balance_change_stats];
  sf.substreams.v1.Clock[source: sf.substreams.v1.Clock] --> balance_changes:balance_change_stats;
  balance_changes:store_valid_balance_changes --> balance_changes:balance_change_stats;
```

### Modules

```yaml
Package name: erc20_contracts
Version: v0.1.0
Doc: ERC-20 Token Contract Info
Modules:
----
Name: map_balance_changes
Initial block: 0
Kind: map
Output Type: proto:erc20.types.v1.BalanceChanges
Hash: 5b0bcc0bd04653f0b398b3e34fb4cf9fcb8ef148
Doc: Extracts ERC20 balance changes

Name: map_valid_balance_changes
Initial block: 0
Kind: map
Output Type: proto:erc20.types.v1.ValidBalanceChanges
Hash: 68b78a7f5b8c554468a236191c33fdf8dedc3810
Doc: Extracts ERC20 balance changes

Name: store_known_contracts
Initial block: 0
Kind: store
Value Type: string
Update Policy: UPDATE_POLICY_SET_IF_NOT_EXISTS
Hash: 82f03b5fd3efbdbc65c7e5d45798405f33257815
Doc: Stores known contracts

Name: map_contracts
Initial block: 0
Kind: map
Output Type: proto:erc20.contracts.types.v1.Contracts
Hash: 5db9c17c07f9c77340123d40265899b957d69266
Doc: Extracts ERC20 token name, symbol and decimals

Name: graph_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: f6b5404eddf524d3fb0de22ada75aae3959b33d5

Name: db_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.database.v1.DatabaseChanges
Hash: 53b65c3f5f14d17f41dd0e0f5d0defa17d1c07a6

Name: balance_changes:map_balance_changes
Initial block: 0
Kind: map
Output Type: proto:erc20.types.v1.BalanceChanges
Hash: 72c5e86a57ac3dc893e7f8433e87d39dc24d059f
Doc: Extracts ERC20 balance changes

Name: balance_changes:map_valid_balance_changes
Initial block: 0
Kind: map
Output Type: proto:erc20.types.v1.ValidBalanceChanges
Hash: 7e7d1e2e18bf0f240faa936a3cd4ddf197c53340
Doc: Extracts ERC20 balance changes

Name: balance_changes:map_unknown_balance_changes
Initial block: 0
Kind: map
Output Type: proto:erc20.types.v1.UnknownBalanceChanges
Hash: 05062a84a99f182d1c1e0291e24822e6bee02568

Name: balance_changes:db_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.database.v1.DatabaseChanges
Hash: d5c55f368e0c79b3b6ec118b4a4963763bd73f5c

Name: balance_changes:graph_out
Initial block: 0
Kind: map
Output Type: proto:sf.substreams.sink.entity.v1.EntityChanges
Hash: 1e05e3878640cd243d4783808982300b2d87a4b9

Name: balance_changes:store_valid_balance_changes
Initial block: 0
Kind: store
Value Type: bigint
Update Policy: UPDATE_POLICY_ADD
Hash: b054d3dc657fbee6ea68687d980b13161f85f97b

Name: balance_changes:balance_change_stats
Initial block: 0
Kind: map
Output Type: proto:erc20.types.v1.BalanceChangeStats
Hash: e74b51058508548b72e606076b7e8a847ddefe6d
```
