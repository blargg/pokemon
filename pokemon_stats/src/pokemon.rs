pub use crate::moves::*;
use crate::parsing::*;
use serde_json::{Value};
use std::ops::Mul;
pub use strum::IntoEnumIterator;
use strum_macros::{EnumIter};

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum Efficacy {
    Zero,
    Fourth,
    Half,
    X1,
    X2,
    X4,
}

impl Mul for Efficacy {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        use Efficacy::*;
        match (self, rhs) {
            (X1, r) => r,
            (l, X1) => l,
            (Zero, _) => Zero,
            (_, Zero) => Zero,
            (X4, Half) => X2,
            (Half, X4) => X2,
            (X4, Fourth) => X1,
            (Fourth, X4) => X1,
            (X4, _) => panic!("Efficacy overflow"),
            (_, X4) => panic!("Efficacy overflow"),
            (X2, X2) => X4,
            (X2, Half) => X1,
            (X2, Fourth) => Half,
            (Fourth, X2) => Half,
            (Half, X2) => X1,
            (Half, Half) => Fourth,
            (Half, Fourth) => panic!("Efficacy underflow"),
            (Fourth, Half) => panic!("Efficacy underflow"),
            (Fourth, Fourth) => panic!("Efficacy underflow"),
        }
    }
}

/// A single type in the type chart
#[derive(Copy, Clone, Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum PureType {
    Bug = 0,
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

impl PureType {
    pub fn against(self, pokemon: &Pokemon) -> Efficacy {
        pokemon.types.against(self)
    }

    pub fn efficacy(attack: PureType, defense: PureType) -> Efficacy {
        use PureType::*;
        use Efficacy::*;
        match (attack, defense) {
            (Bug, Fire) => Half,
            (Bug, Grass) => X2,
            (Bug, Fighting) => Half,
            (Bug, Poison) => Half,
            (Bug, Flying) => Half,
            (Bug, Psychic) => X2,
            (Bug, Ghost) => Half,
            (Bug, Dark) => X2,
            (Bug, Steel) => Half,
            (Bug, Fairy) => Half,
            (Bug, _) => X1,

            (Dark, Poison) => Half,
            (Dark, Psychic) => X2,
            (Dark, Ghost) => X2,
            (Dark, Dark) => Half,
            (Dark, Fairy) => Half,
            (Dark, _) => X1,

            (Dragon, Dragon) => X2,
            (Dragon, Steel) => Half,
            (Dragon, Fairy) => Zero,
            (Dragon, _) => X1,

            (Electric, Water) => X2,
            (Electric, Electric) => Half,
            (Electric, Grass) => Half,
            (Electric, Ground) => Zero,
            (Electric, Flying) => X2,
            (Electric, Dragon) => Half,
            (Electric, _) => X1,

            (Fairy, Fire) => Half,
            (Fairy, Fighting) => X2,
            (Fairy, Poison) => Half,
            (Fairy, Dragon) => X2,
            (Fairy, Dark) => X2,
            (Fairy, Steel) => Half,
            (Fairy, _) => X1,

            (Fighting, Normal) => X2,
            (Fighting, Ice) => X2,
            (Fighting, Poison) => Half,
            (Fighting, Flying) => Half,
            (Fighting, Psychic) => Half,
            (Fighting, Bug) => Half,
            (Fighting, Rock) => X2,
            (Fighting, Ghost) => Zero,
            (Fighting, Dark) => X2,
            (Fighting, Steel) => X2,
            (Fighting, Fairy) => Half,
            (Fighting, _) => X1,

            (Fire, Fire) => Half,
            (Fire, Water) => Half,
            (Fire, Grass) => X2,
            (Fire, Ice) => X2,
            (Fire, Bug) => X2,
            (Fire, Steel) => X2,
            (Fire, Rock) => Half,
            (Fire, Dragon) => Half,
            (Fire, _) => X1,

            (Flying, Electric) => Half,
            (Flying, Grass) => X2,
            (Flying, Fighting) => X2,
            (Flying, Bug) => X2,
            (Flying, Rock) => Half,
            (Flying, Steel) => Half,
            (Flying, _) => X1,

            (Ghost, Normal) => Zero,
            (Ghost, Psychic) => X2,
            (Ghost, Ghost) => X2,
            (Ghost, Dark) => Half,
            (Ghost, _) => X1,

            (Grass, Fire) => Half,
            (Grass, Water) => X2,
            (Grass, Poison) => Half,
            (Grass, Ground) => X2,
            (Grass, Flying) => Half,
            (Grass, Bug) => Half,
            (Grass, Rock) => X2,
            (Grass, Dragon) => Half,
            (Grass, Steel) => Half,
            (Grass, _) => X1,

            (Ground, Fire) => X2,
            (Ground, Electric) => X2,
            (Ground, Grass) => Half,
            (Ground, Poison) => X2,
            (Ground, Flying) => Zero,
            (Ground, Bug) => Half,
            (Ground, Rock) => X2,
            (Ground, Steel) => X2,
            (Ground, _) => X1,

            (Ice, Fire) => Half,
            (Ice, Water) => Half,
            (Ice, Ice) => Half,
            (Ice, Steel) => Half,
            (Ice, Grass) => X2,
            (Ice, Ground) => X2,
            (Ice, Flying) => X2,
            (Ice, Dragon) => X2,
            (Ice, _) => X1,

            (Normal, Rock) => Half,
            (Normal, Steel) => Half,
            (Normal, Ghost) => Zero,
            (Normal, _) => X1,

            (Poison, Poison) => Half,
            (Poison, Ground) => Half,
            (Poison, Rock) => Half,
            (Poison, Ghost) => Half,
            (Poison, Grass) => X2,
            (Poison, Fairy) => X2,
            (Poison, _) => X1,

            (Psychic, Psychic) => Half,
            (Psychic, Steel) => Half,
            (Psychic, Fighting) => X2,
            (Psychic, Poison) => X2,
            (Psychic, Dark) => Zero,
            (Psychic, _) => X1,

            (Rock, Fighting) => Half,
            (Rock, Ground) => Half,
            (Rock, Steel) => Half,
            (Rock, Fire) => X2,
            (Rock, Ice) => X2,
            (Rock, Flying) => X2,
            (Rock, Bug) => X2,
            (Rock, _) => X1,

            (Steel, Fire) => Half,
            (Steel, Water) => Half,
            (Steel, Electric) => Half,
            (Steel, Steel) => Half,
            (Steel, Ice) => X2,
            (Steel, Rock) => X2,
            (Steel, Fairy) => X2,
            (Steel, _) => X1,

            (Water, Water) => Half,
            (Water, Grass) => Half,
            (Water, Dragon) => Half,
            (Water, Fire) => X2,
            (Water, Ground) => X2,
            (Water, Rock) => X2,
            (Water, _) => X1,
        }
    }
}

/// Pokemon can have either 1 or 2 types.
#[derive(Copy, Clone, Debug)]
pub enum PokemonType {
    Single(PureType),
    Double(PureType, PureType),
}

impl PokemonType {
    /// PokemonType enum can represent meaningless values, like `Double(Fire, Fire)`.
    /// This reduces those cases to their normal form: `Single(Fire)`.
    ///
    /// 1. Double(x, x) -> Single(x)
    /// 2. Double(y, x) -> Double(x, y), ordered alphabetically
    fn normalize(self) -> Self {
        use PokemonType::*;

        match self {
            Double(x, y) => {
                if x == y {
                    Single(x)
                } else if x < y{
                    Double(x, y)
                } else {
                    Double(y, x)
                }
            }
            ty => ty
        }
    }

    pub fn efficacy(attack: PureType, defense: PokemonType) -> Efficacy {
        match defense.normalize() {
            PokemonType::Single(d) => PureType::efficacy(attack, d),
            PokemonType::Double(d1, d2) => PureType::efficacy(attack, d1) * PureType::efficacy(attack, d2),
        }
    }

    pub fn against(self, attack: PureType) -> Efficacy {
        PokemonType::efficacy(attack, self)
    }

    pub fn type_matchups(self) -> impl Iterator<Item = (PureType, Efficacy)> {
        PureType::iter().map(move |attack| (attack, self.against(attack)))
    }

    pub fn weaknesses(self) -> impl Iterator<Item = PureType> {
        self
            .type_matchups()
            .filter(|(_, eff)| eff >= &Efficacy::X2)
            .map(|(ty, _)| ty)
    }

    pub fn resistances(self) -> impl Iterator<Item = PureType> {
        self
            .type_matchups()
            .filter(|(_, eff)| eff <= &Efficacy::Half)
            .map(|(ty, _)| ty)
    }

    pub fn iter() -> impl Iterator<Item = PokemonType> {
        let vals = PokemonType::all_vec();
        vals.into_iter()
    }

    fn all_vec() -> Vec<PokemonType> {
        let mut all = Vec::new();

        for first in PureType::iter() {
            all.push(PokemonType::Single(first));
        }

        for first in PureType::iter() {
            for second in PureType::iter() {
                all.push(PokemonType::Double(first, second));
            }
        }

        all
    }
}

impl PartialEq for PokemonType {
    fn eq(&self, other: &PokemonType) -> bool {
        use PokemonType::*;
        match (self.normalize(), other.normalize()) {
            (Single(x), Single(y)) => x == y,
            (Double(x, y), Double(w, z)) => x == w && y == z,
            _ => false,
        }
    }
}

impl Eq for PokemonType { }

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Pokemon {
    pub name: String,
    pub stage: i64,
    pub galar_dex: Option<u32>,
    pub base_stats: Stats,
    pub ev_yield: Stats,
    pub abilities: Vec<String>,
    pub types: PokemonType,
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
            types: pokemon_type(&json["types"])
                .ok_or("types".to_string())?,
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

    /// Checks if the two pokemon can breed with each other
    pub fn breeds_with(&self, other: &Pokemon) -> bool {
        if self.egg_groups.iter().any(|group| group.as_str() == "Undiscovered")
            || other.egg_groups.iter().any(|group| group.as_str() == "Undiscovered") {
            return false;
        }

        // Any pokemon can breed with ditto, except itself and pokemon with the "Undiscovered" egg group
        if (self.name == "Ditto".to_string()) && (other.name == "Ditto".to_string()) {
            return false;
        }
        if (self.name == "Ditto".to_string()) ^ (other.name == "Ditto".to_string()) {
            return true;
        }

        self.common_egg_group(other)
    }

    fn common_egg_group(&self, other: &Pokemon) -> bool {
        for group1 in self.egg_groups.iter() {
            for group2 in other.egg_groups.iter() {
                if group1 == group2 {
                    return true;
                }
            }
        }

        false
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
