use std::collections::HashMap;
use pg::*;
use strategies::Strategy;

// slide 22
fn prog(game: &Game, v: &Node, w: &Node) -> MeasureT {
    let d = 1 + game.max_prio() as usize;
    let mut res = vec![0; d];
    if v.prio % 2 == 0 {
        
    } else {
        
    }

    return MeasureT::Measure(Measure(res));
}

// slide 26
fn lift(game: &Game, strategy: &Strategy, progress: &Progress) -> Progress {
    // grab random vertex
    let v = strategy.vertex();
    let mut progress_val = progress.0.clone();

    let edges = v.succ.iter().map(|w| prog(game, v, game.0.get(w).unwrap()));
    let val = if v.owner == Owner::Even {
            edges.min().unwrap()
        } else {
            edges.max().unwrap()
        };
    if &val > progress_val.get(&v.id).unwrap() {
        progress_val.insert(v.id, val);
    }
    
    Progress(progress_val)
}

pub fn small_progress_measures(game: &Game, strategy: &Strategy) -> Progress {
    let d = 1 + game.max_prio() as usize;
    let mut m = HashMap::new();

    for (id, node) in game.0.iter() {
        let measure = Measure(vec![0; d]);
        m.insert(node.id, MeasureT::Measure(measure));
    }
    let mut progress = Progress(m);
    loop {
        let l = lift(game, strategy, &progress);
        if l >= progress {
            break;
        }
        progress = l;
    }
    return progress;
}