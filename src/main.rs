mod arguments;
mod pg;
mod parser;
mod algorithm;
mod strategies;

use arguments::*;
use pg::*;
use strategies::*;

fn main() {
    let args = arguments::get();
    
    let game = parser::parse_from_file(&args.pg_file);
    println!("{:?}", game);
    println!("");
    
    match args.strategy {
        StrategySort::Random => run(&game, &RandomStrategy::new(&game)),
        StrategySort::Input => run(&game, &InputStrategy::new(&game)),
        StrategySort::Priority => run(&game, &PriorityStrategy::new(&game)),
        StrategySort::Succesor => run(&game, &SuccesorStrategy::new(&game)),
        // StrategySort::Distance => run(&game, &DistanceStrategy::new(&game)),
    }
}

fn run(game: &Game, strat: &Strategy) {
    let progress = algorithm::small_progress_measures(&game, strat);
    println!("{:?}", progress);
    println!("");
    println!("Won even: {:?}", progress.winning_set(Owner::Even));
    println!("Won odd : {:?}", progress.winning_set(Owner::Odd));
}