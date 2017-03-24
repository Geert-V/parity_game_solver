use std::collections::HashMap;
use pg::*;
use strategies::Strategy;

// slide 22
fn prog(game: &Game, progress: &Progress, v: &Node, w: &Node) -> MeasureT {
    let m_w = progress.measure(&w.id);

    if m_w == &MeasureT::Top {
        return MeasureT::Top;
    }

    let mut m = game.new_measure();
    let v_prio = v.prio as usize;
    let prio_is_even = v_prio % 2 == 0;

    if prio_is_even {
        while m.lt(m_w, v_prio) {
            m = m.inc(game);
        }
    } else {
        while m.le(m_w, v_prio) {
            m = m.inc(game);
        }
    }

    m
}

// slide 26
fn lift(game: &Game, v: &Node, progress: &Progress) -> Progress {
    let mut progress_val = progress.0.clone();

    let edges = v.succ.iter().map(|w| prog(game, progress, v, game.node(w)));
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
    let mut progress = game.new_progress();
    let vertices = strategy.vertex();
    loop {
        let mut any_change = false;

        for v in &vertices {
            loop {
                let progress_new = lift(game, v, &progress);
                
                if progress != progress_new {
                    progress = progress_new;
                    any_change = true;
                } else {
                    break;
                }
            }
        }

        if !any_change {
            break;
        }
    }

    return progress;
}