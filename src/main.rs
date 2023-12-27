use std::error::Error;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use pool::UnitsStore;
mod util;
mod pool;

#[derive(Debug, Serialize, Deserialize)]
pub struct BetterHTTPResponse {
    sets: HashMap<u32, BetterSetResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BetterSetResponse {
    champions: Vec<BetterSetChampionsResponse>,
    name: String,
    traits: Vec<BetterSetTraitsResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BetterSetChampionsResponse {
    traits: Vec<String>,
    cost: u32,
    name: String,
    apiName: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BetterSetTraitsResponse {
    apiName: String,
    desc: String,
    icon: String,
    name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let store = UnitsStore::new();
    store.check_pool();

    let samira = store.get_unit_info("Samira");
    println!("{samira}");
    Ok(())
}