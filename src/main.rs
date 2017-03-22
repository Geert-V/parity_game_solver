mod pg;
mod parser;
mod algorithm;
use pg::Game;
use algorithm::RandomStrategy;

fn main() {
    let game = parser::parse_from_file("pg_test.txt");
    println!("{:?}", game);

    let strat = RandomStrategy::new(&game);
    let progress = algorithm::small_progress_measures(&game, &strat);
    println!("{:?}", progress);
}
