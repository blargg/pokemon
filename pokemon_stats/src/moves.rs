use crate::pokemon::PureType;
use serde::Deserialize;

/// A move that a pokemon may know
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct MoveId {
    #[serde(rename="Moves")]
    name: String
}

impl MoveId {
    pub fn from_name(name: &String) -> MoveId {
        MoveId {
            name: name.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct Move {
    #[serde(flatten)]
    id: MoveId,
    #[serde(rename="Type", deserialize_with = "deserialize::de_type")]
    move_type: PureType,
    category: u8,
    power: u32,
    accuracy: u32,
    #[serde(rename="PP")]
    pp: u32,
    priority: u32,
    hit_min: u8,
    hit_max: u8,
    inflict: u16,
    inflict_percent: u8,
    raw_inflict_count: u8,

    turn_min: u8,
    turn_max: u8,
    crit_stage: u8,
    flinch: u32,
    effect_sequence: u32,
    recoil: u32,
    raw_healing: u32,
    raw_target: u32,

    #[serde(rename="Stat1")]
    stat1: u8,
    #[serde(rename="Stat1Stage")]
    stat1_stage: u8,
    #[serde(rename="Stat1Percent")]
    stat1_percent: u8,
    #[serde(rename="Stat2")]
    stat2: u8,
    #[serde(rename="Stat2Stage")]
    stat2_stage: u8,
    #[serde(rename="Stat2Percent")]
    stat2_percent: u8,
    #[serde(rename="Stat3")]
    stat3: u8,
    #[serde(rename="Stat3Stage")]
    stat3_stage: u8,
    #[serde(rename="Stat3Percent")]
    stat3_percent: u8,
    #[serde(rename="GigantimaxPower")]
    dynamax_power: u8,
    target: String,
}

mod deserialize {
    use crate::pokemon::PureType;
    use serde::*;
    use serde::de;
    use core::fmt;

    pub(crate) fn de_type<'de, D>(deserializer: D) -> Result<PureType, D::Error>
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

                    _ => panic!("not a valid pokemon type"),
                }
            }
        }

        deserializer.deserialize_i8(PureTypeVisitor)
    }
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
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
