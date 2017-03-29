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
    println!("");
    println!("Maximal measure: {:?}", game.max_measure());
    
    match args.strategy {
        StrategySort::Random => run(&game, &RandomStrategy::new(&game)),
        StrategySort::Input => run(&game, &InputStrategy::new(&game)),
        StrategySort::Priority => run(&game, &PriorityStrategy::new(&game)),
        StrategySort::Succesor => run(&game, &SuccesorStrategy::new(&game)),
        StrategySort::SelfLoop => run(&game, &SelfLoopStrategy::new(&game))
        // StrategySort::Distance => run(&game, &DistanceStrategy::new(&game)),
    }
}

fn run(game: &Game, strat: &Strategy) {
    let progress = algorithm::small_progress_measures(&game, strat);
    println!("");
    println!("Won even: {:?}", progress.winning_set(Owner::Even));
    println!("");
    println!("Won odd : {:?}", progress.winning_set(Owner::Odd));
}