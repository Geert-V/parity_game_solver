extern crate rand;
use self::rand::Rng;
use std::collections::HashSet;
use std::collections::HashMap;
use pg::*;

// slide 22
fn prog(progress: &Progress, v: Node, w: Node) -> Progress {
    let m = HashMap::new();
    return Progress(m);
}

// slide 26
fn lift(game: &Game, progress: &Progress) -> Progress {
    let m = HashMap::new();
    // let v = rand::thread_rng().choose(&game.0);

    // if v.owner == Owner::Even {
    //     // min
    // } else if v.owner == Owner::Odd {
    //     // max
    // }

    return Progress(m);
} 

pub fn small_progress_measures(game: Game) -> Progress {
    let d = 1 + game.max_prio() as usize;
    let mut m = HashMap::new();

    for node in game.0.iter() {
        m.insert(node.id, Measure(vec![0; d]));
    }
    let mut progress = Progress(m);
    loop {
        let l = lift(&game, &progress);
        if l >= progress {
            break;
        }
        progress = l;
    }
    return progress;
}