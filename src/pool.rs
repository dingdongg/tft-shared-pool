use std::collections::HashMap;
use std::fs;
use crate::util::{Unit, TFTTrait, make_unit};
use std::str::FromStr;
use crate::BetterHTTPResponse;

const ONE_COST_UNITS_COUNT: u32 = 22;
const TWO_COST_UNITS_COUNT: u32 = 20;
const THREE_COST_UNITS_COUNT: u32 = 17;
const FOUR_COST_UNITS_COUNT: u32 = 10;
const FIVE_COST_UNITS_COUNT: u32 = 9;

const UNIT_COUNTS: [u32;5] = [
    ONE_COST_UNITS_COUNT,
    TWO_COST_UNITS_COUNT,
    THREE_COST_UNITS_COUNT,
    FOUR_COST_UNITS_COUNT,
    FIVE_COST_UNITS_COUNT,
];

pub struct UnitsStore {
    pool: HashMap<String, u32>,
    units_index: HashMap<String, Unit>,
}

impl UnitsStore {
    pub fn new() -> Self {
        let units_index = Self::init_index();
        let mut pool = HashMap::new();

        for unit in units_index.values() {
            pool.insert(unit.name.clone(), UNIT_COUNTS[unit.cost as usize - 1]);
        }

        Self { pool, units_index }
    }

    fn init_index() -> HashMap<String, Unit> {
        let mut map = HashMap::new();
        let file = fs::read_to_string("content.json").expect("FILE NOT FOUND");
        let res_json: BetterHTTPResponse = serde_json::from_str(&file).expect("ILL-FORMATTED JSON");

        let champions_slice = &res_json.sets.get(&10).unwrap().champions;

        for champion in champions_slice {
            // println!("{}", champion.apiName);
            if champion.apiName.contains("TFT10") && champion.traits.len() != 0 && champion.cost <= 5 {
                let parsed_traits: Vec<TFTTrait> = champion.traits
                    .clone()
                    .into_iter()
                    .map(|t| TFTTrait::from_str(&t).unwrap())
                    .collect();

                let cloned_name = champion.name.clone();
                let unit = make_unit(&champion.name, champion.cost, parsed_traits, false);

                map.insert(cloned_name, unit);
            }
        }

        map
    }

    pub fn check_pool(&self) -> () {
        println!("{:#?}", self.pool)
    }

    pub fn get_unit_info(&self, name: &str) -> &Unit {
        self.units_index.get(name).unwrap()
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