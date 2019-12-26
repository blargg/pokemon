mod parsing;
pub mod pokemon;
pub mod moves;

pub use moves::*;
pub use pokemon::*;

use statistical;

pub fn print_population_stats(pokemon: Vec<Pokemon>) {
    let vals = pokemon
        .iter()
        .map(|p| p.base_stats.hp as f32)
        .collect::<Vec<_>>();
    let (average, std) = average_and_stdev(vals.as_slice());
    println!("average hp = {}, std = {}", average, std);

    let vals = pokemon
        .iter()
        .map(|p| p.base_stats.attack as f32)
        .collect::<Vec<_>>();
    let (average, std) = average_and_stdev(vals.as_slice());
    println!("average attack = {}, std = {}", average, std);

    let vals = pokemon
        .iter()
        .map(|p| p.base_stats.defense as f32)
        .collect::<Vec<_>>();
    let (average, std) = average_and_stdev(vals.as_slice());
    println!("average defense = {}, std = {}", average, std);

    let vals = pokemon
        .iter()
        .map(|p| p.base_stats.sp_attack as f32)
        .collect::<Vec<_>>();
    let (average, std) = average_and_stdev(vals.as_slice());
    println!("average special attack = {}, std = {}", average, std);

    let vals = pokemon
        .iter()
        .map(|p| p.base_stats.sp_defense as f32)
        .collect::<Vec<_>>();
    let (average, std) = average_and_stdev(vals.as_slice());
    println!("average special defense = {}, std = {}", average, std);

    let vals = pokemon
        .iter()
        .map(|p| p.base_stats.speed as f32)
        .collect::<Vec<_>>();
    let (average, std) = average_and_stdev(vals.as_slice());
    println!("average speed = {}, std = {}", average, std);

    let vals = pokemon
        .iter()
        .map(|p| p.base_stats.total() as f32)
        .collect::<Vec<_>>();
    let (average, std) = average_and_stdev(vals.as_slice());
    println!("average total = {}, std = {}", average, std);
}

fn average_and_stdev<T>(v: &[T]) -> (T, T)
    where T: num::traits::Float {
    let average = statistical::mean(v);
    let std = statistical::standard_deviation(v, Some(average));
    (average, std)
}
