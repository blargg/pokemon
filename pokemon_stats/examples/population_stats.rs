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
        .filter(|p| p.is_galar())
        .collect::<Vec<_>>();

    print_population_stats(galar_pokemon);
}
