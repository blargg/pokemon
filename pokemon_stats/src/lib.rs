pub mod moves;
pub mod party;
pub mod pokemon;

pub use moves::*;
pub use pokemon::{
    *,
    PureType::*,
};

use statistical;

pub fn print_population_stats(pokemon: &Vec<&Species>) {
    println!("population count = {}", pokemon.len());

    if pokemon.len() > 2 {
        let vals = pokemon
            .iter()
            .map(|p| p.base_stats.hp as f32)
            .collect::<Vec<_>>();
        let (average, std) = average_and_stdev(vals.as_slice());
        print_mean("hp", average, std);

        let vals = pokemon
            .iter()
            .map(|p| p.base_stats.attack as f32)
            .collect::<Vec<_>>();
        let (average, std) = average_and_stdev(vals.as_slice());
        print_mean("attack", average, std);

        let vals = pokemon
            .iter()
            .map(|p| p.base_stats.defense as f32)
            .collect::<Vec<_>>();
        let (average, std) = average_and_stdev(vals.as_slice());
        print_mean("defense", average, std);

        let vals = pokemon
            .iter()
            .map(|p| p.base_stats.sp_attack as f32)
            .collect::<Vec<_>>();
        let (average, std) = average_and_stdev(vals.as_slice());
        print_mean("sp attack", average, std);

        let vals = pokemon
            .iter()
            .map(|p| p.base_stats.sp_defense as f32)
            .collect::<Vec<_>>();
        let (average, std) = average_and_stdev(vals.as_slice());
        print_mean("sp defense", average, std);

        let vals = pokemon
            .iter()
            .map(|p| p.base_stats.speed as f32)
            .collect::<Vec<_>>();
        let (average, std) = average_and_stdev(vals.as_slice());
        print_mean("speed", average, std);

        let vals = pokemon
            .iter()
            .map(|p| p.base_stats.total() as f32)
            .collect::<Vec<_>>();
        let (average, std) = average_and_stdev(vals.as_slice());
        print_mean("total", average, std);
    } else if pokemon.len() == 1{
        println!("only {}", pokemon[0].name);
    } else {
        println!("no pokemon provided");
    }
}

fn print_mean<S: ToString>(measure: S, mean: f32, std_dev: f32) {
    println!("{:<10} = {:>9.5} (σ = {:.5})", measure.to_string(), mean, std_dev);
}

fn average_and_stdev<T>(v: &[T]) -> (T, T)
    where T: num::traits::Float {
    let average = statistical::mean(v);
    let std = statistical::standard_deviation(v, Some(average));
    (average, std)
}
