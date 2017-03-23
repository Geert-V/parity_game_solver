mod pg;
mod parser;
mod algorithm;
mod strategies;
use strategies::*;

fn main() {
    let game = parser::parse_from_file("pg_test.txt");
    println!("{:?}", game);

    let strat = InputStrategy::new(&game);
    let progress = algorithm::small_progress_measures(&game, &strat);
    println!("{:?}", progress);
}
