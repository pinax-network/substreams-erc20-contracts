use crate::abi;
use crate::pb::erc20::contracts::types::v1::{Contracts, Contract};
use substreams::log;
use substreams::store::{Deltas, DeltaString};
use substreams::{errors::Error, Hex, scalar::BigInt};
use substreams::pb::sf::substreams::index::v1::Keys;

#[substreams::handlers::map]
fn index_contracts(store: Deltas<DeltaString>) -> Result<Keys, Error> {
    Ok(match store.deltas.is_empty() {
        true => Keys::default(),
        false => Keys {
            keys: vec!["contracts".to_string()]
        },
    })
}



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
                let result: i32  =  decimal_.to_u64() as i32;
                if result < 0 {
                    log::info!("Invalid decimals value: {:?}", result);
                    break;
                }
                else {
                    decimals = result;
                }
            }
            None => {}
   
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
    log::info!("before call decimals:");
    call.call(hex)
}

