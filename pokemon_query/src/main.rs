use pokemon_stats::*;
use serde_json::{Value};
use std::{
    io::BufReader,
    fs::File,
};

fn main() {
    let f = File::open("data/json/pokemon.json")
        .expect("could not open file");
    let reader = BufReader::new(f);
    let all_json: Value = serde_json::from_reader(reader)
        .expect("couldn't parse the json");

    let pokemon = pokemon_array(&all_json)
        .expect("couldn't convert json to structs");
    let galar_pokemon = pokemon.into_iter().filter(|p| p.is_galar());

    let mut population = galar_pokemon
        .filter(|p| p.can_learn(&MoveId::from_name(&"Trick Room".to_string())))
        .collect::<Vec<_>>();
    population.sort_by(|a, b| a.base_stats.speed.cmp(&b.base_stats.speed));

    for poke in population.iter() {
        println!("{}, {}", poke.base_stats.speed, poke.name);
    }
    print_population_stats(population);
}
