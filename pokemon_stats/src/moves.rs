use crate::pokemon::{
    PureType,
    Stat,
};
use enumset::EnumSet;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::convert::From;

const MOVES_TSV: &[u8] = include_bytes!("../../data/raw/sword_shield_move_info.tsv");

/// Loads a list of moves as a vector. Returns the parsing error.
pub fn safe_load_moves() -> Result<Vec<Move>, csv::Error> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(MOVES_TSV);
    let mut all_moves = Vec::with_capacity(100);
    for result in reader.deserialize() {
        let mv: Move = result?;
        all_moves.push(mv);
    }

    Ok(all_moves)
}

/// Loads the list of moves as a vector.
pub fn load_moves() -> Vec<Move> {
    safe_load_moves().expect("Could not load moves")
}

/// A move that a pokemon may know
#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
#[serde(transparent)]
pub struct MoveId {
    #[serde(rename="Moves")]
    name: String
}

impl From<&str> for MoveId {
    fn from(s: &str) -> Self {
        MoveId::from_name(s)
    }
}

impl From<String> for MoveId {
    fn from(s: String) -> Self {
        MoveId {
            name: s,
        }
    }
}

impl MoveId {
    pub fn from_name(name: &str) -> MoveId {
        MoveId {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

/// Move Category, dictates the attack type
#[derive(Deserialize_repr, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Category {
    Status = 0,
    Physical = 1,
    Special = 2,
}

/// Represents a percent on the integers from 0 to 100 (inclusive).
type Percent = u8;

/// Describes positions the move is allowed to target.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Target {
    All,
    AllAdjacent,
    AllAdjacentOpponents,
    AllAllies,
    Ally,
    AllyOrSelf,
    AnyExceptSelf,
    Counter,
    Opponent,
    RandomOpponent,
    SideAll,
    SideOpponent,
    SideSelf,
    TargetSelf,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all="PascalCase")]
pub struct Move {
    #[serde(rename="Moves")]
    pub id: MoveId,
    #[serde(rename="CanUseMove", deserialize_with = "deserialize::tf")]
    pub available_in_gen8: bool,
    #[serde(rename="Type", deserialize_with = "deserialize::de_type")]
    pub move_type: PureType,
    pub category: Category,
    pub power: u8,
    pub accuracy: Percent,
    #[serde(rename="PP")]
    pub pp: u8,
    #[serde(deserialize_with="deserialize::u8_to_i8")]
    pub priority: i8,
    pub hit_min: u8,
    pub hit_max: u8,
    pub inflict: u16,
    pub inflict_percent: Percent,
    pub raw_inflict_count: u8,

    pub turn_min: u8,
    pub turn_max: u8,
    pub crit_stage: u8,
    pub flinch: u8,
    pub effect_sequence: u32,
    /// The damage drain percent of the move.
    /// Positive values will heal the user based on the amount of damage done to the opponent.
    /// Negative values (recoil, like in the move double edge) will damage the user.
    #[serde(deserialize_with="deserialize::u8_to_i8")]
    pub recoil: i8,
    pub raw_target: u32,

    #[serde(deserialize_with = "deserialize::stat")]
    pub stat1: EnumSet<Stat>,
    pub stat1_stage: u8,
    pub stat1_percent: Percent,
    #[serde(deserialize_with = "deserialize::stat")]
    pub stat2: EnumSet<Stat>,
    pub stat2_stage: u8,
    pub stat2_percent: Percent,
    #[serde(deserialize_with = "deserialize::stat")]
    pub stat3: EnumSet<Stat>,
    pub stat3_stage: u8,
    pub stat3_percent: Percent,
    #[serde(rename="GigantimaxPower")]
    pub dynamax_power: u8,
    #[serde(rename="Flag_MakesContact", deserialize_with = "deserialize::tf")]
    pub makes_contact: bool,
    #[serde(rename="Flag_Charge", deserialize_with = "deserialize::tf")]
    pub charge: bool,
    #[serde(rename="Flag_Recharge", deserialize_with = "deserialize::tf")]
    pub recharge: bool,
    /// Moves like Protect will block this move from taking effect
    #[serde(rename="Flag_Protect", deserialize_with = "deserialize::tf")]
    pub protect_blocks: bool,
    #[serde(rename="Flag_Reflectable", deserialize_with = "deserialize::tf")]
    pub reflectable: bool,
    #[serde(rename="Flag_Snatch", deserialize_with = "deserialize::tf")]
    pub snatch: bool,
    #[serde(rename="Flag_Mirror", deserialize_with = "deserialize::tf")]
    pub mirror: bool,
    #[serde(rename="Flag_Punch", deserialize_with = "deserialize::tf")]
    pub punch: bool,
    /// Indicates if this is a sound based ability
    #[serde(rename="Flag_Sound", deserialize_with = "deserialize::tf")]
    pub sound: bool,
    #[serde(rename="Flag_Gravity", deserialize_with = "deserialize::tf")]
    pub gravity: bool,
    #[serde(rename="Flag_Defrost", deserialize_with = "deserialize::tf")]
    pub defrost: bool,
    #[serde(rename="Flag_DistanceTriple", deserialize_with = "deserialize::tf")]
    pub distance_triple: bool,
    #[serde(rename="Flag_IgnoreSubstitute", deserialize_with = "deserialize::tf")]
    pub ignore_substitute: bool,
    #[serde(rename="Flag_FailSkyBattle", deserialize_with = "deserialize::tf")]
    pub fail_sky_battle: bool,
    #[serde(rename="Flag_AnimateAlly", deserialize_with = "deserialize::tf")]
    pub animate_ally: bool,
    #[serde(rename="Flag_Dance", deserialize_with = "deserialize::tf")]
    pub dance: bool,
    #[serde(rename="Flag_18", deserialize_with = "deserialize::tf")]
    pub flag18: bool,

    /// Indicates that this move is a healing ability
    #[serde(rename="Flag_Heal", deserialize_with = "deserialize::tf")]
    pub heal: bool,
    /// Healing done, as a percent of the users max health
    #[serde(deserialize_with="deserialize::opt_u8_to_i8")]
    pub healing: Option<i8>,
    #[serde(deserialize_with="deserialize::target")]
    pub target: Target,
}

impl Move {
    pub fn name(&self) -> &str {
        self.id.name.as_str()
    }

    pub fn effect_on_stats(&self, stat: Stat) -> Option<(u8, u8)> {
        if self.stat1.contains(stat) {
            return Some((self.stat1_percent, self.stat1_stage));
        }
        if self.stat2.contains(stat) {
            return Some((self.stat2_percent, self.stat2_stage));
        }
        if self.stat3.contains(stat) {
            return Some((self.stat3_percent, self.stat3_stage));
        }

        None
    }

    pub fn stat_effects(&self) -> Vec<(Stat, u8, u8)> {
        let mut effs = Vec::new();

        for stat in self.stat1.iter() {
            effs.push((
                    stat,
                    self.stat1_percent,
                    self.stat1_stage,
            ));
        }
        for stat in self.stat2.iter() {
            effs.push((
                    stat,
                    self.stat2_percent,
                    self.stat2_stage,
            ));
        }
        for stat in self.stat3.iter() {
            effs.push((
                    stat,
                    self.stat3_percent,
                    self.stat3_stage,
            ));
        }

        effs
    }

    /// Determins if the move is an attack.
    pub fn is_attack(&self) -> bool {
        use Category::*;
        match self.category {
            Physical => true,
            Special => true,
            Status => false,
        }
    }
}

mod deserialize {
    use crate::pokemon::{
        PureType,
        Stat,
    };
    use enumset::EnumSet;
    use serde::{
        *,
        de::{
            self,
            Unexpected
        },
    };
    use core::fmt;
    use super::Target;

    pub(super) fn target<'de, D>(deserializer: D) -> Result<Target, D::Error>
        where D: Deserializer<'de>
    {
        use Target::*;

        match <&'de str>::deserialize(deserializer)? {
            "All" => Ok(All),
            "AllAdjacent" => Ok(AllAdjacent),
            "AllAdjacentOpponents" => Ok(AllAdjacentOpponents),
            "AllAllies" => Ok(AllAllies),
            "Ally" => Ok(Ally),
            "AllyOrSelf" => Ok(AllyOrSelf),
            "AnyExceptSelf" => Ok(AnyExceptSelf),
            "Counter" => Ok(Counter),
            "Opponent" => Ok(Opponent),
            "RandomOpponent" => Ok(RandomOpponent),
            "Self" => Ok(TargetSelf),
            "SideAll" => Ok(SideAll),
            "SideOpponent" => Ok(SideOpponent),
            "SideSelf" => Ok(SideSelf),
            s => Err(de::Error::custom(
                    format!("unexpected value: {}", s)
                    )),
        }
    }

    pub(super) fn u8_to_i8<'de, D>(deserializer: D) -> Result<i8, D::Error>
        where D: Deserializer<'de>
    {
        let unsigned = <u8>::deserialize(deserializer)?;
        Ok(unsigned as i8)
    }

    pub(super) fn opt_u8_to_i8<'de, D>(deserializer: D) -> Result<Option<i8>, D::Error>
        where D: Deserializer<'de>
    {
        match <&'de str>::deserialize(deserializer)? {
            "None" => Ok(None),
            s => {
                let num = s.parse::<u8>()
                    .map_err(|_| de::Error::custom("error deserializing optional number"))?;
                Ok(Some(num as i8))
            }
        }
    }

    pub(super) fn de_type<'de, D>(deserializer: D) -> Result<PureType, D::Error>
        where D: Deserializer<'de>
    {
        struct PureTypeVisitor;
        impl<'de> de::Visitor<'de> for PureTypeVisitor {
            type Value = PureType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number representing a pokemon type")
            }

            fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
                where E: de::Error,
            {
                use PureType::*;
                match v {
                    0 => Ok(Normal),
                    1 => Ok(Fighting),
                    2 => Ok(Flying),
                    3 => Ok(Poison),
                    4 => Ok(Ground),
                    5 => Ok(Rock),
                    6 => Ok(Bug),
                    7 => Ok(Ghost),
                    8 => Ok(Steel),
                    9 => Ok(Fire),
                    10 => Ok(Water),
                    11 => Ok(Grass),
                    12 => Ok(Electric),
                    13 => Ok(Psychic),
                    14 => Ok(Ice),
                    15 => Ok(Dragon),
                    16 => Ok(Dark),
                    17 => Ok(Fairy),

                    v => Err(E::invalid_value(de::Unexpected::Unsigned(v as u64), &self)),
                }
            }
        }

        deserializer.deserialize_i8(PureTypeVisitor)
    }

    pub(super) fn stat<'de, D>(deserializer: D) -> Result<EnumSet<Stat>, D::Error>
        where D: Deserializer<'de>
    {
        struct StatVisitor;
        impl<'de> de::Visitor<'de> for StatVisitor {
            type Value = EnumSet<Stat>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number representing a pokemon stat")
            }

            fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
                where E: de::Error,
            {
                use Stat::*;
                match v {
                    0 => Ok(EnumSet::empty()),
                    1 => Ok(EnumSet::only(Attack)),
                    2 => Ok(EnumSet::only(Defense)),
                    3 => Ok(EnumSet::only(SpAttack)),
                    4 => Ok(EnumSet::only(SpDefense)),
                    5 => Ok(EnumSet::only(Speed)),
                    6 => Ok(EnumSet::only(Accuracy)),
                    7 => Ok(EnumSet::only(Evasion)),
                    8 => Ok(Attack | Defense | SpAttack | SpDefense | Speed | EnumSet::empty()),

                    v => Err(E::invalid_value(de::Unexpected::Unsigned(v as u64), &self)),
                }
            }
        }

        deserializer.deserialize_u8(StatVisitor)
    }

    pub(super) fn tf<'de, D>(deserializer: D) -> Result<bool, D::Error>
        where D: Deserializer<'de>
    {
        struct TFVisitor;
        impl<'de> de::Visitor<'de> for TFVisitor {
            type Value = bool;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("\"true\" or \"false\"")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where E: de::Error,
            {
                match v.to_uppercase().as_str() {
                    "TRUE" => Ok(true),
                    "FALSE" => Ok(false),
                    _ => Err(E::invalid_value(Unexpected::Str(v), &self)),
                }
            }
        }

        deserializer.deserialize_str(TFVisitor)
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(transparent)]
pub struct TM {
    id: u64,
}

impl TM {
    pub fn new(id: u64) -> TM {
        assert!(id < 100);
        TM {
            id,
        }
    }

    pub fn as_move(self) -> MoveId {
        MoveId {
            name: TM_S[self.id as usize].to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Hash)]
#[serde(transparent)]
pub struct TR {
    id: u64,
}

impl TR {
    pub fn new(id: u64) -> TR {
        assert!(id < 100);
        TR {
            id,
        }
    }

    pub fn as_move(self) -> MoveId {
        MoveId {
            name: TR_S[self.id as usize].to_string(),
        }
    }
}

pub const TM_S: &'static[&'static str] = &[
    "Mega Punch",
    "Mega Kick",
    "Pay Day",
    "Fire Punch",
    "Ice Punch",
    "Thunder Punch",
    "Fly",
    "Pin Missile",
    "Hyper Beam",
    "Giga Impact",
    "Magical Leaf",
    "Solar Beam",
    "Solar Blade",
    "Fire Spin",
    "Thunder Wave",
    "Dig",
    "Screech",
    "Light Screen",
    "Reflect",
    "Safeguard",
    "Self-Destruct",
    "Rest",
    "Rock Slide",
    "Thief",
    "Snore",
    "Protect",
    "Scary Face",
    "Icy Wind",
    "Giga Drain",
    "Charm",
    "Steel Wing",
    "Attract",
    "Sandstorm",
    "Rain Dance",
    "Sunny Day",
    "Hail",
    "Whirlpool",
    "Beat Up",
    "Will-O-Wisp",
    "Facade",
    "Swift",
    "Helping Hand",
    "Revenge",
    "Brick Break",
    "Imprison",
    "Dive",
    "Weather Ball",
    "Fake Tears",
    "Rock Tomb",
    "Sand Tomb",
    "Bullet Seed",
    "Icicle Spear",
    "Bounce",
    "Mud Shot",
    "Rock Blast",
    "Brine",
    "U-turn",
    "Payback",
    "Assurance",
    "Fling",
    "Power Swap",
    "Guard Swap",
    "Speed Swap",
    "Drain Punch",
    "Avalanche",
    "Shadow Claw",
    "Thunder Fang",
    "Ice Fang",
    "Fire Fang",
    "Psycho Cut",
    "Trick Room",
    "Wonder Room",
    "Magic Room",
    "Cross Poison",
    "Venoshock",
    "Low Sweep",
    "Round",
    "Hex",
    "Acrobatics",
    "Retaliate",
    "Volt Switch",
    "Bulldoze",
    "Electroweb",
    "Razor Shell",
    "Tail Slap",
    "Snarl",
    "Phantom Force",
    "Draining Kiss",
    "Grassy Terrain",
    "Misty Terrain",
    "Electric Terrain",
    "Psychic Terrain",
    "Mystical Fire",
    "Eerie Impulse",
    "False Swipe",
    "Air Slash",
    "Smart Strike",
    "Brutal Swing",
    "Stomping Tantrum",
    "Breaking Swipe",
];

pub const TR_S: &'static[&'static str] = &[
    "Swords Dance",
    "Body Slam",
    "Flamethrower",
    "Hydro Pump",
    "Surf",
    "Ice Beam",
    "Blizzard",
    "Low Kick",
    "Thunderbolt",
    "Thunder",
    "Earthquake",
    "Psychic",
    "Agility",
    "Focus Energy",
    "Metronome",
    "Fire Blast",
    "Waterfall",
    "Amnesia",
    "Leech Life",
    "Tri Attack",
    "Substitute",
    "Reversal",
    "Sludge Bomb",
    "Spikes",
    "Outrage",
    "Psyshock",
    "Endure",
    "Sleep Talk",
    "Megahorn",
    "Baton Pass",
    "Encore",
    "Iron Tail",
    "Crunch",
    "Shadow Ball",
    "Future Sight",
    "Uproar",
    "Heat Wave",
    "Taunt",
    "Trick",
    "Superpower",
    "Skill Swap",
    "Blaze Kick",
    "Hyper Voice",
    "Overheat",
    "Cosmic Power",
    "Muddy Water",
    "Iron Defense",
    "Dragon Claw",
    "Bulk Up",
    "Calm Mind",
    "Leaf Blade",
    "Dragon Dance",
    "Gyro Ball",
    "Close Combat",
    "Toxic Spikes",
    "Flare Blitz",
    "Aura Sphere",
    "Poison Jab",
    "Dark Pulse",
    "Seed Bomb",
    "X-Scissor",
    "Bug Buzz",
    "Dragon Pulse",
    "Power Gem",
    "Focus Blast",
    "Energy Ball",
    "Brave Bird",
    "Earth Power",
    "Nasty Plot",
    "Zen Headbutt",
    "Flash Cannon",
    "Leaf Storm",
    "Power Whip",
    "Gunk Shot",
    "Iron Head",
    "Stone Edge",
    "Stealth Rock",
    "Grass Knot",
    "Sludge Wave",
    "Heavy Slam",
    "Electro Ball",
    "Foul Play",
    "Stored Power",
    "Ally Switch",
    "Scald",
    "Work Up",
    "Wild Charge",
    "Drill Run",
    "Heat Crash",
    "Hurricane",
    "Play Rough",
    "Venom Drench",
    "Dazzling Gleam",
    "Darkest Lariat",
    "High Horsepower",
    "Throat Chop",
    "Pollen Puff",
    "Psychic Fangs",
    "Liquidation",
    "Body Press",
];

#[cfg(test)]
mod test {
    use super::*;

    const TEST_DATA: &'static str = r#"
Index	Moves	Version	MoveID	CanUseMove	Type	Quality	Category	Power	Accuracy	PP	Priority	HitMin	HitMax	Inflict	InflictPercent	RawInflictCount	TurnMin	TurnMax	CritStage	Flinch	EffectSequence	Recoil	RawHealing	RawTarget	Stat1	Stat2	Stat3	Stat1Stage	Stat2Stage	Stat3Stage	Stat1Percent	Stat2Percent	Stat3Percent	GigantimaxPower	Flag_MakesContact	Flag_Charge	Flag_Recharge	Flag_Protect	Flag_Reflectable	Flag_Snatch	Flag_Mirror	Flag_Punch	Flag_Sound	Flag_Gravity	Flag_Defrost	Flag_DistanceTriple	Flag_Heal	Flag_IgnoreSubstitute	Flag_FailSkyBattle	Flag_AnimateAlly	Flag_Dance	Flag_18	InflictCount	Healing	Target
1	Pound	5	1	True	0	0	1	40	100	35	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	90	True	False	False	True	False	False	True	False	False	False	False	False	False	False	False	False	False	True	None	None	AnyExceptSelf
105	Recover	5	105	True	0	3	0	0	101	10	0	0	0	0	0	0	0	0	0	0	32	0	50	7	0	0	0	0	0	0	0	0	0	0	False	False	False	False	False	True	False	False	False	False	False	False	True	False	False	False	False	True	None	50	Self
"#;

    #[test]
    fn move_parse_test() {
        let mut parser = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(TEST_DATA.as_bytes());
        let moves = parser.deserialize().collect::<Result<Vec<Move>,_>>()
            .expect("Failed to parse all the test data");
        let pound = moves.iter()
            .find(|m| m.name() == "Pound")
            .expect("Could not find the move named pound");
        assert!(pound.makes_contact);
        assert_eq!(None, pound.healing);

        let recover = moves.iter()
            .find(|m| m.name() == "Recover")
            .expect("Could not find the move named pound");
        assert_eq!(Some(50), recover.healing);
    }

    #[test]
    fn load_moves_test() {
        match safe_load_moves() {
            Ok(_) => {/* pass test */}
            Err(e) => {
                println!("{:?}", e);
                panic!("Failed to load moves");
            }
        }
    }
}
