
use substreams::errors::Error;
use substreams_entity_change::tables::Tables;
use substreams_entity_change::pb::entity::EntityChanges;

use crate::pb::erc20::contracts::types::v1::Contracts;

#[substreams::handlers::map]
pub fn graph_out(contracts: Contracts)  -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    for event in contracts.items {
        let address = &event.address;
        tables
            .create_row("Contracts", address)
            .set("address", address)
            .set("name", &event.name)
            .set("symbol", &event.symbol)
            .set_bigint("decimals", &event.decimals.to_string());
    }
    Ok(tables.to_entity_changes())
}
