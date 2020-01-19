use pokemon_stats::*;

fn main() {
    // parse into a vector of pokemon
    let pokemon = pokemon::load_pokemon();
    let galar_pokemon = pokemon
        .into_iter()
        .filter(|p| p.is_galar())
        .collect::<Vec<_>>();

    print_population_stats(galar_pokemon);
}
