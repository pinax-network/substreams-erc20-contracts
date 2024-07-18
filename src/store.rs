use crate::pb::erc20::types::v1::ValidBalanceChanges;
use substreams::store::{StoreSetIfNotExists, StoreSetIfNotExistsString};
use substreams::store::StoreNew;

#[substreams::handlers::store]
fn store_known_contracts(balances: ValidBalanceChanges,s: StoreSetIfNotExistsString)
{
     for item in balances.valid_balance_changes {
          s.set_if_not_exists(0, &item.contract, &item.contract);
     }
}
