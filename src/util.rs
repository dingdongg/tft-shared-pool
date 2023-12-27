use std::fmt;
use std::str::FromStr;

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

impl FromStr for TFTTrait {
    type Err = ();
    fn from_str(input: &str) -> Result<TFTTrait, Self::Err> {
        match input {
            "8-bit" => Ok(Self::EightBit),
            "Country" => Ok(Self::Country),
            "Disco" => Ok(Self::Disco),
            "EDM" => Ok(Self::Edm),
            "Emo" => Ok(Self::Emo),
            "Heartsteel" => Ok(Self::Heartsteel),
            "Hyperpop" => Ok(Self::Hyperpop),
            "ILLBEATS" => Ok(Self::Illbeats),
            "Jazz" => Ok(Self::Jazz),
            "K/DA" => Ok(Self::Kda),
            "Maestro" => Ok(Self::Maestro),
            "Mixmaster" => Ok(Self::Mixmaster),
            "Pentakill" => Ok(Self::Pentakill),
            "Punk" => Ok(Self::Punk),
            "True Damage" => Ok(Self::TrueDamage),
            "Wildcard" => Ok(Self::Wildcard),
            "Big Shot" => Ok(Self::BigShot),
            "Bruiser" => Ok(Self::Bruiser),
            "Breakout" => Ok(Self::Breakout),
            "Crowd Diver" => Ok(Self::CrowdDiver),
            "Dazzler" => Ok(Self::Dazzler),
            "Edgelord" => Ok(Self::Edgelord),
            "Executioner" => Ok(Self::Executioner),
            "Guardian" => Ok(Self::Guardian),
            "Mosher" => Ok(Self::Mosher),
            "Rapidfire" => Ok(Self::Rapidfire),
            "Sentinel" => Ok(Self::Sentinel),
            "Spellweaver" => Ok(Self::Spellweaver),
            "Superfan" => Ok(Self::Superfan),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Unit {
    pub cost: u32,
    pub name: String,
    pub traits: Vec<TFTTrait>,
    pub is_headliner: bool,
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
