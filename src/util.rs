use std::fmt;

#[derive(Debug)]
pub enum TFTTrait {
    EightBit,
    Country,
    Disco,
    Edm,
    Emo,
    Heartsteel,
    Hyperpop,
    Illbeats,
    Jazz,
    Kda,
    Maestro,
    Mixmaster,
    Pentakill,
    Punk,
    TrueDamage,
    Wildcard,
    BigShot,
    Bruiser,
    Breakout,
    CrowdDiver,
    Dazzler,
    Edgelord,
    Executioner,
    Guardian,
    Mosher,
    Rapidfire,
    Sentinel,
    Spellweaver,
    Superfan,
}

impl fmt::Display for TFTTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub struct Unit {
    cost: u32,
    name: String,
    traits: Vec<TFTTrait>,
    is_headliner: bool,
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) | headliner? {}\n{:#?}\n------------",
            self.name,
            self.cost,
            self.is_headliner,
            self.traits,
        )
    }
}

pub fn make_unit(name: &str, cost: u32, traits: Vec<TFTTrait>, is_headliner: bool) -> Unit {
    Unit {
        cost,
        name: String::from(name),
        traits,
        is_headliner,
    }
}
