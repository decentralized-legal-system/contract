use crate::contract::Contract;
use crate::map::MAP;

mod map;
mod contract;

// Retrieves the value associated with the given key if it exists.
#[ic_cdk_macros::query]
fn get(key: u128) -> Option<Contract>
{
    MAP.with(|p| p.borrow().get(&key))
}

// Inserts an entry into the map and returns the previous value of the key if it exists.
#[ic_cdk_macros::update]
fn set(key: u128, value: Contract) -> Option<Contract>
{
    MAP.with(|p| p.borrow_mut().insert(key, value))
}