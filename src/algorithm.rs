use std::collections::HashSet;
use std::collections::HashMap;
use pg::*;
use strategies::Strategy;
use std::cmp;
// slide 22
fn prog<'game>(v: &Node, w: &Node) -> Measure {
    return Measure(vec!(0));
}

// slide 26
fn lift<'game>(game: &Game, strategy: &'game Strategy, progress: &'game mut Progress) -> &'game mut Progress {
    // grab random vertex
    let v = strategy.vertex();

    let edges = v.succ.iter().map(|w| prog(v, game.0.get(w).unwrap()));
    let val = if v.owner == Owner::Even {
            edges.min().unwrap()
        } else {
            edges.max().unwrap()
        };
    if &val > progress.0.get(&v.id).unwrap() {
        progress.0.insert(v.id, val);
    }
    
    return progress;
}

pub fn small_progress_measures<'game>(game: &Game, strategy: &'game Strategy) -> &'game Progress {
    let d = 1 + game.max_prio() as usize;
    let mut m = HashMap::new();

    for (id, node) in game.0.iter() {
        m.insert(*id, Measure(vec![0; d]));
    }
    let progress = &mut Progress(m);
    loop {
        let l = lift(game, strategy, progress);
        if l >= progress {
            break;
        }
        progress = l;
    }
    return progress;
}