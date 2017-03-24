mod pg;
mod parser;
mod algorithm;
mod strategies;

use pg::*;
use strategies::*;

fn main() {
    let game = parser::parse_from_file("pg_test.txt");
    println!("{:?}", game);
    println!("");

    let strat = InputStrategy::new(&game);
    let progress = algorithm::small_progress_measures(&game, &strat);
    println!("{:?}", progress);
    println!("");
    println!("Won even: {:?}", progress.winning_set(Owner::Even));
    println!("Won odd : {:?}", progress.winning_set(Owner::Odd));
}
