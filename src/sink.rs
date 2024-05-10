
use substreams::errors::Error;
use substreams_entity_change::tables::Tables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_database_change::pb::database::{DatabaseChanges, table_change::Operation};
use substreams::pb::substreams::Clock;
use crate::pb::erc20::contracts::types::v1::Contracts;


#[substreams::handlers::map]
pub fn graph_out(clock: Clock,contracts: Contracts)  -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    
    let block = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for event in contracts.items {
        let address = &event.address;
        let row = tables
            .create_row("Contracts", address)
            .set("address", address);

        if !event.name.is_empty() {
            row.set("name", &event.name);
        }
            
        if !event.symbol.is_empty() {
            row.set("symbol", &event.symbol);
        }

        if  event.decimals != 0 {
            row.set("decimals", event.decimals);
        }  

            row.set("block_number", block.clone());
            row.set("timestamp", timestamp.clone());
    }

    Ok(tables.to_entity_changes())
}


#[substreams::handlers::map]
fn db_out(clock: Clock, contracts: Contracts) -> Result<DatabaseChanges, Error> {

    // Initialize Database Changes container
    let mut database_changes: DatabaseChanges = Default::default();

    let block = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for event in contracts.items {
        let address = &event.address;
    // Create row 
    let row = database_changes.push_change("Contracts", address, 0, Operation::Create);
    if !event.name.is_empty() {
        row.change("name", (None, event.name));
    }
    else {
        row.change("name", (None, ""));
    }

    if !event.symbol.is_empty() {
        row.change("symbol", (None, event.symbol));
    }
    else {
        row.change("symbol", (None, ""));
    }

    if  event.decimals != 0 {
        row.change("decimals", (None, event.decimals));
    }
    else {
        row.change("decimals", (None, 0));
    
    }
        row.change("block_number", (None,block.clone()));
        row.change("timestamp", (None, timestamp.clone()));
    }

    
    Ok(database_changes)
}


