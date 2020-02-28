
use crate::pokemon::*;
use std::collections::BTreeMap;

/// A group of pokemon that can be used all at once in a battle
pub struct Party {
    members: Vec<Species>,
}

impl Party {
    /// Converts from a vector to a team of pokemon
    /// Checks if team size is valid
    pub fn from_vec(members: Vec<Species>) -> Option<Self> {
        if members.len() > 0 && members.len() <= 6 {
            Some(Party {
                members,
            })
        } else {
            None
        }
    }

    /// Solo pokemon team
    pub fn solo(pokemon: Species) -> Self {
        Party {
            members: vec![pokemon],
        }
    }

    /// Owned variant to build a team of pokemon
    pub fn with(mut self, pokemon: Species) -> Self {
        if self.members.len() < 6 {
            self.members.push(pokemon);
        }
        self
    }


    pub fn type_matchups(&self) -> BTreeMap<(Efficacy, PureType), u32> {
        let mut freq = BTreeMap::new();
        for pokemon in self.members.iter() {
            for ty in PureType::iter() {
                let eff = ty.against(pokemon);
                *freq.entry((eff, ty)).or_insert(0) += 1;
            }
        }

        freq
    }
}
