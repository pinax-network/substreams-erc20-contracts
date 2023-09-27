use crate::abi;
use crate::pb::erc20::contract::types::v1::{Infos, Info};
use substreams::log;
use substreams::store::{Deltas, DeltaString};
use substreams::{errors::Error, Hex, scalar::BigInt};

#[substreams::handlers::map]
pub fn map_contract_info(store: Deltas<DeltaString>) -> Result<Infos, Error> {
   
   let mut items: Vec<Info> = Vec::new();
    for delta in store.deltas{
        let name;
        let symbol;
        let decimal;

        match get_contract_name(delta.key.clone()) {
            Some(name_) => {
                name = name_;
            },
            None => {
                name = "Name not found".to_string()
            },
        }

        match get_contract_symbol(delta.key.clone()) {
            Some(symbol_) => {
                symbol = symbol_;
            },
            None => {
                symbol = "Symbol not found".to_string()
            },
        }

        match get_contract_decimals(delta.key.clone()) {
            Some(decimal_) => {
                decimal = decimal_.to_string();
            },
            None => {
                decimal = "0".to_string()
            },
        }

       items.push(Info { address: delta.key, name: name, symbol: symbol, decimal: decimal.to_string() })
    }
    Ok(Infos { items })
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

