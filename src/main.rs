mod pg;
mod parser;
mod algorithm;
mod strategies;

use pg::*;
use strategies::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let game = parser::parse_from_file("pg_test.txt");
    println!("{:?}", game);
    println!("");

    let strat_input = &args[1];
    
    match strat_input.as_str() {
        "random" => {
            run(&game, &RandomStrategy::new(&game).vertex());
        },
        "input" => {
            run(&game, &InputStrategy::new(&game).vertex());
        },
        _ => {
            panic!("No strategy specified! Try: input, random")
        }
    };


}

fn run(game: &Game, strat: &Vec<&Node>) {
    let progress = algorithm::small_progress_measures(&game, &strat);
    println!("{:?}", progress);
    println!("");
    println!("Won even: {:?}", progress.winning_set(Owner::Even));
    println!("Won odd : {:?}", progress.winning_set(Owner::Odd));
}