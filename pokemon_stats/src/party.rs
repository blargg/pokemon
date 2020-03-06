
use crate::pokemon::*;
use std::collections::BTreeMap;

/// A group of pokemon that can be used all at once in a battle
pub struct Party {
    members: Vec<Pokemon>,
}

impl Party {
    /// Converts from a vector to a team of pokemon
    /// Checks if team size is valid
    pub fn from_vec(members: Vec<Pokemon>) -> Option<Self> {
        if members.len() > 0 && members.len() <= 6 {
            Some(Party {
                members,
            })
        } else {
            None
        }
    }

    /// Solo pokemon team
    pub fn solo(pokemon: Pokemon) -> Self {
        Party {
            members: vec![pokemon],
        }
    }

    /// Owned variant to build a team of pokemon
    pub fn with(mut self, pokemon: Pokemon) -> Self {
        if self.members.len() < 6 {
            self.members.push(pokemon);
        }
        self
    }

    pub fn members(&self) -> &Vec<Pokemon> {
        &self.members
    }

    pub fn parse(s: &str) -> Self {
        let mut members = Vec::new();

        for pokemon_str in s.split("\n\n") {
            let p = Pokemon::parse(pokemon_str);
            if let Some(p) = p {
                members.push(p);
            }
        }

        Party {
            members,
        }
    }

    pub fn type_matchups(&self) -> BTreeMap<(Efficacy, PureType), u32> {
        let mut freq = BTreeMap::new();
        for pokemon in self.members.iter() {
            for ty in PureType::iter() {
                let eff = ty.against(pokemon.species());
                *freq.entry((eff, ty)).or_insert(0) += 1;
            }
        }

        freq
    }

    /// Checks if any member in the team has an effective attack against the target.
    pub fn has_super_effective_attack(&self, p: &Species) -> bool {
        for attacker in self.members.iter() {
            if attacker.has_super_effective_attack(p) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_test() {
        let party = Party::parse(r#"Vaporeon  
            Ability: Water Absorb  
            EVs: 248 HP / 8 SpA / 252 SpD  
            Calm Nature  
            IVs: 0 Atk  
            - Baton Pass  
            - Acid Armor  
            - Aqua Ring  
            - Surf  

            Dreps (Dragapult) @ Assault Vest  
            Ability: Clear Body  
            - Agility  
            - Baton Pass  
            - Sucker Punch  

            Glaceon  
            Ability: Snow Cloak  
            IVs: 0 Atk  
            - Baton Pass  
            "#
        );

        assert_eq!(3, party.members.len());
        assert_eq!("Glaceon", party.members[2].species_name());
    }
}
