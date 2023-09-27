
use substreams::errors::Error;
use substreams_entity_change::tables::Tables;
use substreams_entity_change::pb::entity::EntityChanges;

use crate::pb::erc20::contract::types::v1::Infos;

#[substreams::handlers::map]
pub fn graph_out(infos: Infos)  -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    for event in infos.items {
        let address = &event.address;
        tables
            .create_row("ContractInfo", address)
            .set("address", address)
            .set("name", &event.name)
            .set("symbol", &event.symbol)
            .set_bigint("decimals", &event.decimal);
    }
    Ok(tables.to_entity_changes())
}
