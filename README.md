# Pokemon
A list of all the Gen 8 pokemon and moves. Use `pokemon::load_pokemon()` and
`moves::load_moves()` to generate a new vector of the pokemon and moves,
respectively.

There are a few basic functions available, but not many. For example, you can do
the following.

* Check if a pokemon can learn a move.
* Check the type effectiveness of an attack against a pokemon.

# Status
This is provided as is. Expect there to be a few mistakes. There are at
least a few cases of missing information. For example, Freeze Dry is super
effective against Water type pokemon despite being an Ice type move. This
currently isn't captured in this data set

# Examples
[move\_search.rs](pokemon_stats/examples/move_search.rs) shows an example of how
to search for all pokemon that know the moves Reflect and Light Screen, sorted
by base stat total.

[population\_stats](pokemon_stats/examples/population_stats.rs) shows how to get
the average and standard deviation of each of the base stats for a list of
pokemon.
