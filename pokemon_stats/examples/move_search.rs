use pokemon_stats::*;
use std::{
    io::BufReader,
    fs::File,
};

fn main() {
    // open the data file
    let f = File::open("data/json/pokemon.json")
        .expect("could not open file");
    let reader = BufReader::new(f);

    // parse into a vector of pokemon
    let galar_pokemon = serde_json::from_reader::<_, Vec<Pokemon>>(reader)
        .expect("couldn't parse the json")
        .into_iter()
        .filter(|p| p.is_galar());


    // make a list of all the pokemon that know Reflect and Light Screen
    let mut knows_moves = galar_pokemon
        .filter(|p| p.can_learn_by_name("Reflect")
            && p.can_learn_by_name("Light Screen"))
        .collect::<Vec<_>>();
    // sort by base stats
    knows_moves.sort_by_key(|p| p.base_stats.total());

    // show all the pokemon and their base stat total
    for pokemon in knows_moves {
        println!("{} {}", pokemon.base_stats.total(), pokemon.name());
    }
}
