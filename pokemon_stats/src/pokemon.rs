pub use crate::moves::*;
use crate::parsing::*;
use enumset::EnumSetType;
use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde_json::{Value};
use std::ops::Mul;

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum Efficacy {
    Zero,
    Pow2(i8),
}

impl Mul for Efficacy {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        use Efficacy::*;
        match (self, rhs) {
            (Zero, _) => Zero,
            (_, Zero) => Zero,
            (Pow2(x), Pow2(y)) => Pow2(x + y),
        }
    }
}

/// A single type in the type chart
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
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

    pub fn iter() -> impl Iterator<Item=PureType> {
        (0..PureType::count())
            .filter_map(PureType::from_usize)
    }

    fn count() -> usize {
        18
    }

    pub fn efficacy(attack: PureType, defense: PureType) -> Efficacy {
        use PureType::*;
        use Efficacy::*;
        match (attack, defense) {
            (Bug, Fire) => Pow2(-1),
            (Bug, Grass) => Pow2(1),
            (Bug, Fighting) => Pow2(-1),
            (Bug, Poison) => Pow2(-1),
            (Bug, Flying) => Pow2(-1),
            (Bug, Psychic) => Pow2(1),
            (Bug, Ghost) => Pow2(-1),
            (Bug, Dark) => Pow2(1),
            (Bug, Steel) => Pow2(-1),
            (Bug, Fairy) => Pow2(-1),
            (Bug, _) => Pow2(0),

            (Dark, Poison) => Pow2(-1),
            (Dark, Psychic) => Pow2(1),
            (Dark, Ghost) => Pow2(1),
            (Dark, Dark) => Pow2(-1),
            (Dark, Fairy) => Pow2(-1),
            (Dark, _) => Pow2(0),

            (Dragon, Dragon) => Pow2(1),
            (Dragon, Steel) => Pow2(-1),
            (Dragon, Fairy) => Zero,
            (Dragon, _) => Pow2(0),

            (Electric, Water) => Pow2(1),
            (Electric, Electric) => Pow2(-1),
            (Electric, Grass) => Pow2(-1),
            (Electric, Ground) => Zero,
            (Electric, Flying) => Pow2(1),
            (Electric, Dragon) => Pow2(-1),
            (Electric, _) => Pow2(0),

            (Fairy, Fire) => Pow2(-1),
            (Fairy, Fighting) => Pow2(1),
            (Fairy, Poison) => Pow2(-1),
            (Fairy, Dragon) => Pow2(1),
            (Fairy, Dark) => Pow2(1),
            (Fairy, Steel) => Pow2(-1),
            (Fairy, _) => Pow2(0),

            (Fighting, Normal) => Pow2(1),
            (Fighting, Ice) => Pow2(1),
            (Fighting, Poison) => Pow2(-1),
            (Fighting, Flying) => Pow2(-1),
            (Fighting, Psychic) => Pow2(-1),
            (Fighting, Bug) => Pow2(-1),
            (Fighting, Rock) => Pow2(1),
            (Fighting, Ghost) => Zero,
            (Fighting, Dark) => Pow2(1),
            (Fighting, Steel) => Pow2(1),
            (Fighting, Fairy) => Pow2(-1),
            (Fighting, _) => Pow2(0),

            (Fire, Fire) => Pow2(-1),
            (Fire, Water) => Pow2(-1),
            (Fire, Grass) => Pow2(1),
            (Fire, Ice) => Pow2(1),
            (Fire, Bug) => Pow2(1),
            (Fire, Steel) => Pow2(1),
            (Fire, Rock) => Pow2(-1),
            (Fire, Dragon) => Pow2(-1),
            (Fire, _) => Pow2(0),

            (Flying, Electric) => Pow2(-1),
            (Flying, Grass) => Pow2(1),
            (Flying, Fighting) => Pow2(1),
            (Flying, Bug) => Pow2(1),
            (Flying, Rock) => Pow2(-1),
            (Flying, Steel) => Pow2(-1),
            (Flying, _) => Pow2(0),

            (Ghost, Normal) => Zero,
            (Ghost, Psychic) => Pow2(1),
            (Ghost, Ghost) => Pow2(1),
            (Ghost, Dark) => Pow2(-1),
            (Ghost, _) => Pow2(0),

            (Grass, Fire) => Pow2(-1),
            (Grass, Water) => Pow2(1),
            (Grass, Poison) => Pow2(-1),
            (Grass, Ground) => Pow2(1),
            (Grass, Flying) => Pow2(-1),
            (Grass, Bug) => Pow2(-1),
            (Grass, Rock) => Pow2(1),
            (Grass, Dragon) => Pow2(-1),
            (Grass, Steel) => Pow2(-1),
            (Grass, _) => Pow2(0),

            (Ground, Fire) => Pow2(1),
            (Ground, Electric) => Pow2(1),
            (Ground, Grass) => Pow2(-1),
            (Ground, Poison) => Pow2(1),
            (Ground, Flying) => Zero,
            (Ground, Bug) => Pow2(-1),
            (Ground, Rock) => Pow2(1),
            (Ground, Steel) => Pow2(1),
            (Ground, _) => Pow2(0),

            (Ice, Fire) => Pow2(-1),
            (Ice, Water) => Pow2(-1),
            (Ice, Ice) => Pow2(-1),
            (Ice, Steel) => Pow2(-1),
            (Ice, Grass) => Pow2(1),
            (Ice, Ground) => Pow2(1),
            (Ice, Flying) => Pow2(1),
            (Ice, Dragon) => Pow2(1),
            (Ice, _) => Pow2(0),

            (Normal, Rock) => Pow2(-1),
            (Normal, Steel) => Pow2(-1),
            (Normal, Ghost) => Zero,
            (Normal, _) => Pow2(0),

            (Poison, Poison) => Pow2(-1),
            (Poison, Ground) => Pow2(-1),
            (Poison, Rock) => Pow2(-1),
            (Poison, Ghost) => Pow2(-1),
            (Poison, Grass) => Pow2(1),
            (Poison, Fairy) => Pow2(1),
            (Poison, _) => Pow2(0),

            (Psychic, Psychic) => Pow2(-1),
            (Psychic, Steel) => Pow2(-1),
            (Psychic, Fighting) => Pow2(1),
            (Psychic, Poison) => Pow2(1),
            (Psychic, Dark) => Zero,
            (Psychic, _) => Pow2(0),

            (Rock, Fighting) => Pow2(-1),
            (Rock, Ground) => Pow2(-1),
            (Rock, Steel) => Pow2(-1),
            (Rock, Fire) => Pow2(1),
            (Rock, Ice) => Pow2(1),
            (Rock, Flying) => Pow2(1),
            (Rock, Bug) => Pow2(1),
            (Rock, _) => Pow2(0),

            (Steel, Fire) => Pow2(-1),
            (Steel, Water) => Pow2(-1),
            (Steel, Electric) => Pow2(-1),
            (Steel, Steel) => Pow2(-1),
            (Steel, Ice) => Pow2(1),
            (Steel, Rock) => Pow2(1),
            (Steel, Fairy) => Pow2(1),
            (Steel, _) => Pow2(0),

            (Water, Water) => Pow2(-1),
            (Water, Grass) => Pow2(-1),
            (Water, Dragon) => Pow2(-1),
            (Water, Fire) => Pow2(1),
            (Water, Ground) => Pow2(1),
            (Water, Rock) => Pow2(1),
            (Water, _) => Pow2(0),
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
            .filter(|(_, eff)| eff >= &Efficacy::Pow2(1))
            .map(|(ty, _)| ty)
    }

    pub fn resistances(self) -> impl Iterator<Item = PureType> {
        self
            .type_matchups()
            .filter(|(_, eff)| eff <= &Efficacy::Pow2(-1))
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

        for i in 0..PureType::count() {
            for j in (i + 1)..PureType::count() {
                let first = PureType::from_usize(i).unwrap();
                let second = PureType::from_usize(j).unwrap();
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

#[derive(Debug, EnumSetType)]
#[repr(u8)]
pub enum Stat {
    Hp,
    Attack,
    Defense,
    SpDefense,
    Speed,
    SpAttack,
    Evasion,
    Accuracy,
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

    pub fn moves<'a>(&'a self) -> impl Iterator<Item = MoveId> + 'a {
        MoveIdIterator::new(self)
    }

    pub fn can_learn(&self, mv: &MoveId) -> bool {
        self.by_level(mv)
            || self.by_egg(mv)
            || self.by_tm(mv)
            || self.by_tr(mv)
    }

    fn by_level(&self, mv: &MoveId) -> bool {
        self
            .level_up_moves
            .iter()
            .map(|(_lvl, name)| MoveId::from_name(name))
            .any(|level_mv| mv == &level_mv)
    }

    fn by_egg(&self, mv: &MoveId) -> bool {
        self
            .egg_moves
            .iter()
            .map(|name| MoveId::from_name(name))
            .any(|egg_move| mv == &egg_move)
    }

    fn by_tm(&self, mv: &MoveId) -> bool {
        self
            .tms
            .iter()
            .any(|tm| &tm.as_move() == mv)
    }

    fn by_tr(&self, mv: &MoveId) -> bool {
        self
            .trs
            .iter()
            .any(|tr| &tr.as_move() == mv)
    }

}

pub struct MoveIdIterator<'a> {
    pokemon: &'a Pokemon,
    /// Indicates what source (level up, egg, tms, trs) we are on
    source_index: u32,
    mv_index: usize,
}

impl<'a> MoveIdIterator<'a> {
    fn new(pokemon: &'a Pokemon) -> Self {
        Self {
            pokemon,
            source_index: 0,
            mv_index: 0,
        }
    }
}

impl<'a> Iterator for MoveIdIterator<'a> {
    type Item = MoveId;

    fn next(&mut self) -> Option<MoveId> {
        if self.source_index == 0 {
            if let Some((_lvl, move_id)) = self.pokemon.level_up_moves.get(self.mv_index) {
                let val = move_id;
                self.mv_index += 1;
                Some(MoveId::from_name(val))
            } else {
                self.source_index += 1;
                self.mv_index = 0;
                self.next()
            }
        } else if self.source_index == 1 {
            if let Some(name) = self.pokemon.egg_moves.get(self.mv_index) {
                let val = MoveId::from_name(name);
                self.mv_index += 1;
                Some(val)
            } else {
                self.source_index += 1;
                self.mv_index = 0;
                self.next()
            }
        } else if self.source_index == 2 {
            if let Some(tm) = self.pokemon.tms.get(self.mv_index) {
                self.mv_index += 1;
                Some(tm.as_move())
            } else {
                self.mv_index = 0;
                self.source_index += 1;
                self.next()
            }
        } else if self.source_index == 3 {
            if let Some(tr) = self.pokemon.trs.get(self.mv_index) {
                self.mv_index += 1;
                Some(tr.as_move())
            } else {
                self.mv_index = 0;
                self.source_index += 1;
                self.next()
            }
        } else {
            None
        }
    }
}

pub fn pokemon_array(json: &Value) -> Option<Vec<Pokemon>> {
    if let Value::Array(vals) = json {
        Some(vals
            .iter()
            .filter_map(|v| Pokemon::from_json(v).ok())
            .collect::<Vec<_>>()
        )
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pure_type_count_test() {
        assert!(PureType::count() > 0);
        let last_type = PureType::from_usize(PureType::count() - 1);
        assert!(last_type.is_some(), "The last type in the index exists");
        let after_last = PureType::from_usize(PureType::count());
        assert!(after_last.is_none(), "After the last, no types exist");
    }

    #[test]
    fn efficacy_ord_test() {
        use Efficacy::*;
        assert!(Zero < Pow2(-2), "zero is less than any power of two");
        assert!(Zero < Pow2(-100), "zero is less than any power of two");

        assert!(Pow2(0) < Pow2(1));
        assert!(Pow2(-1) < Pow2(0));
    }

}
