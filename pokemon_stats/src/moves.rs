/// A move that a pokemon may know
#[derive(PartialEq, Eq)]
pub struct Move {
    name: String
}

impl Move {
    pub fn from_name(name: &String) -> Move {
        Move {
            name: name.clone(),
        }
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

    pub fn as_move(self) -> Move {
        Move {
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

    pub fn as_move(self) -> Move {
        Move {
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
