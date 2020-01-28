use pokemon_stats::*;

fn main() {
    // parse into a vector of pokemon
    let galar_pokemon = pokemon::load_pokemon()
        .into_iter()
        .filter(|p| p.is_galar());


    // make a list of all the pokemon that know Reflect and Light Screen
    let mut knows_moves = galar_pokemon
        .filter(|p| p.can_learn("Reflect")
            && p.can_learn("Light Screen"))
        .collect::<Vec<_>>();
    // sort by base stats
    knows_moves.sort_by_key(|p| p.base_stats.total());

    // show all the pokemon and their base stat total
    for pokemon in knows_moves {
        println!("{} {}", pokemon.base_stats.total(), pokemon.name());
    }
}
