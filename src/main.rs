mod pg;
mod parser;
mod algorithm;
use pg::Game;
fn main() {
    let game = parser::parse_from_file("pg_test.txt");
    println!("{:?}", game);
    let progress = algorithm::small_progress_measures(game);
    println!("{:?}", progress);
}
