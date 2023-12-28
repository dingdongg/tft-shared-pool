use std::{collections::HashMap, time::SystemTime};
use std::fs;
use crate::util::{Unit, TFTTrait, make_unit};
use std::str::FromStr;
use crate::BetterHTTPResponse;
use indexmap::IndexMap;
use rand::{Rng, thread_rng};

const TFT_SET_IDENTIFIER: &str = "TFT10";

const ONE_COST_UNITS_COUNT:   u32 = 22;
const TWO_COST_UNITS_COUNT:   u32 = 20;
const THREE_COST_UNITS_COUNT: u32 = 17;
const FOUR_COST_UNITS_COUNT:  u32 = 10;
const FIVE_COST_UNITS_COUNT:  u32 =  9;

const UNIT_COUNTS: [u32;5] = [
    ONE_COST_UNITS_COUNT,
    TWO_COST_UNITS_COUNT,
    THREE_COST_UNITS_COUNT,
    FOUR_COST_UNITS_COUNT,
    FIVE_COST_UNITS_COUNT,
];

pub struct UnitsStore {
    pool: IndexMap<String, u32>,
    units_index: HashMap<String, Unit>,
}

impl UnitsStore {
    pub fn new() -> Self {
        let units_index = Self::init_index();
        let mut pool = IndexMap::new();

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
            if champion.apiName.contains(TFT_SET_IDENTIFIER) && champion.traits.len() != 0 && champion.cost <= 5 {
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

    pub fn get_unit_count(&self, name: &str) -> u32 {
        *self.pool.get(name).unwrap()
    }
    /**
     * select 5 random (not necessarily unique) units, and return them
     */
    pub fn roll_shop(&self) -> Vec<&Unit> {
        let pool_size = self.pool.len();
        let mut rng = thread_rng();
        let mut ret = Vec::new();

        for _ in 0..5 {
            let random_index = rng.gen_range(0..pool_size);
            let (unit_name, unit_count) = self.pool.get_index(random_index).unwrap();

            ret.push(self.get_unit_info(unit_name));
        }

        ret
    }

    /**
     * client sells the specified unit; unit is returned to the store
     */
    pub fn sell(&mut self, unit: &Unit) -> () {
        println!("Selling... {:#?}", SystemTime::now());
        let index = self.units_index.get(&unit.name).unwrap();
        let prev_count = self.pool.get(&unit.name).unwrap();
        let max_count = &UNIT_COUNTS[(index.cost - 1) as usize];

        if prev_count < max_count {
            self.pool.insert(unit.name.clone(), prev_count + 1);
            println!("{} count: {}", &unit.name, self.get_unit_count(&unit.name));
        } else {
            println!("{} at full capacity already!", &unit.name);
        }
    }

    /**
     * client buys the specified unit; unit is given away from the store
     */
    pub fn buy(&mut self, unit: &Unit) -> () {
        println!("Buying... {:#?}", SystemTime::now());
        let prev_count = self.pool.get(&unit.name).unwrap();

        if prev_count > &0 {
            self.pool.insert(unit.name.clone(), prev_count - 1);
            println!("{} count: {}", &unit.name, self.get_unit_count(&unit.name));
        } else {
            println!("No more copies of {}!", &unit.name);
        }
    }
}