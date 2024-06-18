
use substreams::errors::Error;
use substreams_database_change::pb::database::{DatabaseChanges, table_change::Operation};
use substreams::pb::substreams::Clock;
use crate::pb::erc20::contracts::types::v1::Contracts;


#[substreams::handlers::map]
fn db_out(clock: Clock, contracts: Contracts) -> Result<DatabaseChanges, Error> {

    // Initialize Database Changes container
    let mut database_changes: DatabaseChanges = Default::default();

    let block = clock.number.to_string();
    let timestamp = clock.timestamp.unwrap().seconds.to_string();

    for event in contracts.items {
        let address = &event.address;
    // Create row 
    let row = database_changes.push_change("contracts", address, 0, Operation::Create);
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
        row.change("block_num", (None,block.clone()));
        row.change("timestamp", (None, timestamp.clone()));
    }

    
    Ok(database_changes)
}


