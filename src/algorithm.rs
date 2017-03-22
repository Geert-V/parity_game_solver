extern crate rand;
use self::rand::Rng;
use std::collections::HashSet;
use std::collections::HashMap;
use pg::*;
use strategies::Strategy;

// slide 22
fn prog(progress: &Progress, v: Node, w: Node) -> Progress {
    let m = HashMap::new();
    return Progress(m);
}

// slide 26
fn lift(game: &Game, strategy: &Strategy, progress: &Progress) -> Progress {
    let m = HashMap::new();
    // grab random vertex
    let v = strategy.vertex();
    if v.owner == Owner::Even {
        // min
        
    } else if v.owner == Owner::Odd {
        // max
    }

    return Progress(m);
} 



pub fn small_progress_measures(game: &Game, strategy: &Strategy) -> Progress {
    let d = 1 + game.max_prio() as usize;
    let mut m = HashMap::new();

    for node in game.0.iter() {
        let measure = Measure(vec![0; d]);
        m.insert(node.id, MeasureT::Measure(measure));
    }
    let mut progress = Progress(m);
    loop {
        let l = lift(&game, strategy, &progress);
        if l >= progress {
            break;
        }
        progress = l;
    }
    return progress;
}