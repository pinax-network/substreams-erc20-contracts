
use substreams::errors::Error;
use substreams_entity_change::tables::Tables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

use crate::pb::erc20::contracts::types::v1::Contracts;

#[substreams::handlers::map]
pub fn graph_out(contracts: Contracts)  -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    for event in contracts.items {
        let address = &event.address;
        let row = tables
            .create_row("Contracts", address)
            .set("address", address);

        if !event.name.is_empty() {
            row.set("name", &event.name);
        }
            
        if !event.name.is_empty() {
            row.set("symbol", &event.symbol);
        }

        if  event.decimals != 0 {
            row .set_bigint("decimals", &event.decimals.to_string());
        }  
    }
    Ok(tables.to_entity_changes())
}

/*#[substreams::handlers::map]
pub fn db_out(contracts: Contracts) -> Result<DatabaseChanges, Error> {

    let mut database_changes: DatabaseChanges = Default::default();
  
    for event in contracts.items {
        let address = &event.address;
       
        database_changes
        .push_change("contracts", address.clone(), 0, Operation::Create)
        .change("address", (None, address))
        .change("name", (None, event.name))
        .change("symbol", (None,event.symbol ))
        .change("decimals", (None, event.decimals));
    }
    Ok(database_changes)
}*/


