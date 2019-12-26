pub use crate::moves::*;
use crate::parsing::*;
use serde_json::{Value};

/// A single type in the type chart
#[allow(dead_code)]
enum PureType {
    Bug,
    Dark,
    Dragon,
    Electric,
    Fairy,
    Fighting,
    Fire,
    Flying,
    Ghost,
    Grass,
    Ground,
    Ice,
    Normal,
    Poison,
    Psychic,
    Rock,
    Steel,
    Water,
}

/// Pokemon can have either 1 or 2 types.
#[allow(dead_code)]
enum PokemonType {
    Single(PureType),
    Double(PureType, PureType),
}

#[derive(Debug)]
pub struct Stats {
    pub hp: u64,
    pub attack: u64,
    pub defense: u64,
    pub sp_attack: u64,
    pub sp_defense: u64,
    pub speed: u64,
}

impl Stats {
    pub fn from_json(json: &Value) -> Option<Stats> {
        if let Value::Array(vals) = json {
            Some(
                Stats {
                    hp: u64_json(&vals[0])?,
                    attack: u64_json(&vals[1])?,
                    defense: u64_json(&vals[2])?,
                    sp_attack: u64_json(&vals[3])?,
                    sp_defense: u64_json(&vals[4])?,
                    speed: u64_json(&vals[5])?,
                }
            )
        } else {
            None
        }
    }

    pub fn total(&self) -> u64 {
        self.hp
            + self.attack
            + self.defense
            + self.sp_attack
            + self.sp_defense
            + self.speed
    }
}

#[derive(Debug)]
pub struct Pokemon {
    pub name: String,
    pub stage: i64,
    pub galar_dex: Option<u32>,
    pub base_stats: Stats,
    pub ev_yield: Stats,
    pub abilities: Vec<String>,
    pub types: Vec<String>,
    pub items: Vec<(u64,String)>,
    pub exp_group: String,
    pub egg_groups: Vec<String>,
    pub hatch_cycles: u64,
    pub height: f64,
    pub weight: f64,
    pub color: String,
    pub level_up_moves: Vec<(u64, String)>,
    pub egg_moves: Vec<String>,
    pub tms: Vec<TM>,
    pub trs: Vec<TR>,
}

impl Pokemon {
    pub fn from_json(json: &Value) -> Result<Pokemon, String> {

        Ok(Pokemon {
            name: string(&json["name"], "name")?,
            stage: i64_json(&json["stage"], "stage")?,
            galar_dex: parse_dex(&json["galar_dex"])?,
            base_stats: Stats::from_json(&json["base_stats"])
                .ok_or("base_stats".to_string())?,
            ev_yield: Stats::from_json(&json["ev_yield"])
                .ok_or("ev_yield".to_string())?,
            abilities: str_vec(&json["abilities"], "abilities")?,
            types: str_vec(&json["types"], "types")?,
            exp_group: string(&json["exp_group"], "exp_group")?,
            egg_groups: str_vec(&json["egg_groups"], "egg_groups")?,
            hatch_cycles: u64_json(&json["hatch_cycles"])
                .ok_or("hatch_cycles".to_string())?,
            height: f64_json(&json["height"])
                .ok_or("height".to_string())?,
            weight: f64_json(&json["weight"])
                .ok_or("weight".to_string())?,
            color: string(&json["color"], "color")?,
            items: read_items(&json["items"])
                .ok_or("items".to_string())?,
            level_up_moves: read_lvl_moves(&json["level_up_moves"])
                .ok_or("level_up_moves".to_string())?,
            egg_moves: str_vec(&json["egg_moves"], "egg_moves")?,
            tms: tm_array(&json["tms"])
                .ok_or("could not read TMs".to_string())?,
            trs: tr_array(&json["trs"])
                .ok_or("could not read TRs".to_string())?,
        })
    }

    pub fn is_galar(&self) -> bool {
        self.galar_dex.is_some()
    }

    pub fn can_learn(&self, mv: &Move) -> bool {
        self.by_level(mv)
            || self.by_egg(mv)
            || self.by_tm(mv)
            || self.by_tr(mv)
    }

    fn by_level(&self, mv: &Move) -> bool {
        self
            .level_up_moves
            .iter()
            .map(|(_lvl, name)| Move::from_name(name))
            .any(|level_mv| mv == &level_mv)
    }

    fn by_egg(&self, mv: &Move) -> bool {
        self
            .egg_moves
            .iter()
            .map(|name| Move::from_name(name))
            .any(|egg_move| mv == &egg_move)
    }

    fn by_tm(&self, mv: &Move) -> bool {
        self
            .tms
            .iter()
            .any(|tm| &tm.as_move() == mv)
    }

    fn by_tr(&self, mv: &Move) -> bool {
        self
            .trs
            .iter()
            .any(|tr| &tr.as_move() == mv)
    }

}

pub fn pokemon_array(json: &Value) -> Result<Vec<Pokemon>, String> {
    if let Value::Array(vals) = json {
        Ok(vals
            .iter()
            .filter_map(|v| Pokemon::from_json(v).ok())
            .collect::<Vec<_>>()
        )
    } else {
        Err("Top level json is not an array".to_string())
    }
}
