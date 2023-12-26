use std::collections::HashMap;
use crate::util::Unit;

pub struct UnitsStore {
    pool: HashMap<String, u32>,
}

impl UnitsStore {
    pub fn new() -> Self {
        Self { 
            pool: HashMap::new(),
        }
    }

    /**
     * select 5 random (not necessarily unique) units, and return them
     */
    pub fn roll_shop() -> Vec<Unit> {
        vec![]
    }

    /**
     * client sells the specified unit; unit is returned to the store
     */
    pub fn sell(unit: &Unit) -> () {

    }

    /**
     * client buys the specified unit; unit is given away from the store
     */
    pub fn buy(unit: &Unit) -> () {

    }
}