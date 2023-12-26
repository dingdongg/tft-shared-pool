use util::{TFTTrait, make_unit};
use pool::UnitsStore;
mod util;
mod pool;

fn main() {
    let blitz = make_unit(
        "blitzcrank", 
        4, 
        vec![TFTTrait::Disco, TFTTrait::Sentinel], 
        false,
    );

    println!("{}", blitz);

    let ziggs = make_unit(
        "ziggs", 
        5, 
        vec![TFTTrait::Hyperpop, TFTTrait::Dazzler], 
        false,
    );

    println!("{ziggs}");

    let pool = UnitsStore::new();
    println!("{:#?}", pool);
}