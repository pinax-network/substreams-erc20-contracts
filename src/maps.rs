use crate::abi;
use crate::pb::erc20::contracts::types::v1::{Contracts, Contract};
use substreams::log;
use substreams::store::{Deltas, DeltaString};
use substreams::{errors::Error, Hex, scalar::BigInt};

#[substreams::handlers::map]
pub fn map_contracts(store: Deltas<DeltaString>) -> Result<Contracts, Error> {
   let mut items: Vec<Contract> = Vec::new();
    for delta in store.deltas{
        let mut name = "".to_string();
        let mut symbol = "".to_string();
        let mut decimals= 0;
        
        match get_contract_name(delta.key.clone()) {
            Some(name_) => {
                name = name_;
            },
            None => {},
        }

        match get_contract_symbol(delta.key.clone()) {
            Some(symbol_) => {
                symbol = symbol_;
            },
            None => {},
        }

        match get_contract_decimals(delta.key.clone()) {
            Some(decimal_) => {
                decimals = decimal_.into();
            },
            None => {},
        }

            items.push(Contract { address: delta.key, name, symbol, decimals })
    }
    Ok(Contracts { items })
}

// ETH Call to retrieve ERC20 token Name
pub fn get_contract_name(address: String) -> Option<String> {
    let call = abi::erc20::functions::Name{};
    log::info!("get_contract_name: {:?}", address);
    let hex = Hex::decode(address).unwrap();
    call.call(hex)
}

// ETH Call to retrieve ERC20 token Symbol
pub fn get_contract_symbol(address: String) -> Option<String> {
    let call = abi::erc20::functions::Symbol{};
    log::info!("get_contract_symbol: {:?}", address);
    let hex = Hex::decode(address).unwrap();
    call.call(hex)
}

// ETH Call to retrieve ERC20 token Decimal
pub fn get_contract_decimals(address: String) -> Option<BigInt> {
    let call = abi::erc20::functions::Decimals{};
    log::info!("get_contract_decimals: {:?}", address);
    let hex: Vec<u8> = Hex::decode(address).unwrap();
    call.call(hex)
}

