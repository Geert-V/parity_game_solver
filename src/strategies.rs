extern crate rand;
use self::rand::Rng;
use std::collections::HashSet;
use std::collections::HashMap;
use pg::*;
use std::borrow::BorrowMut;
use std::iter::Iterator;
use std::collections::LinkedList;
use std::collections::VecDeque;

pub trait Strategy {
    fn vertex(&self) -> Vec<&Node>;
}

pub struct InputStrategy<'game> (Vec<&'game Node>);
impl<'game> InputStrategy<'game> {
    pub fn new(game: &'game Game) -> InputStrategy<'game> {
        return InputStrategy(game.nodes().into_iter().collect())
    }
}
pub struct RandomStrategy<'game> (Vec<&'game Node>);
impl<'game> RandomStrategy<'game> {
    pub fn new(game: &'game Game) -> RandomStrategy<'game> {
        return RandomStrategy(game.nodes().into_iter().collect());
    }
}
pub struct PriorityStrategy<'game> (Vec<&'game Node>);
impl<'game> PriorityStrategy<'game> {
    pub fn new(game: &'game Game) -> PriorityStrategy<'game> {
        return PriorityStrategy(game.nodes().into_iter().collect());
    }
}
pub struct SuccesorStrategy<'game> (Vec<&'game Node>);
impl<'game> SuccesorStrategy<'game> {
    pub fn new(game: &'game Game) -> SuccesorStrategy<'game> {
        return SuccesorStrategy(game.nodes().into_iter().collect());
    }
}

// fn Dijkstra<'game>(game: &'game Game, nodes: &'game Vec<&'game Node>, reversed: &'game HashMap<&'game u32, LinkedList<u32>>, source: &'game Node )
//              -> (HashMap<&'game Node, u32>, HashMap<&'game Node, Option<&'game Node>>) {
//     let mut dist = HashMap::new();
//     let mut prev = HashMap::new();
//     let mut queue = HashSet::new();
//     for v in *nodes {
//         dist.insert(v, u32::max_value());
//         prev.insert(v, None);
//         queue.insert(v);
//     };
//     dist.insert(source, 0);
//     while queue.len() > 0 {
//         let u = lowest_element(&queue);
//         queue.remove(u);
//         for v_id in &u.succ {
//             let v = game.node(v_id);
//             let alt = (dist.get(u).unwrap() + 1).clone();
//             let old = dist.get(v).unwrap().clone();
//             if alt < old {
//                 prev.insert(v, Some(u));
//                 dist.insert(v, alt);
//             };
//         };
//     }
//     return (dist, prev);
// }

fn lowest_element<'game>(nodes: &HashSet<&'game Node>) -> &'game Node {
    nodes.iter().fold(None, |min, x| match min {
        None => Some(x),
        Some(y) => Some(if x.prio < y.prio { x } else { y }),
    }).unwrap()
}

pub struct SelfLoopStrategy<'game> (HashMap<&'game Node, u32>);
impl<'game> SelfLoopStrategy<'game> {
    pub fn new(game: &'game Game) -> SelfLoopStrategy<'game> {
        let nodes = game.nodes();

        let iter = &nodes.clone();
        let endings = iter.iter().filter(|x| 
            x.succ.contains(&x.id) && (x.succ.len() == 1 || 
                (x.prio % 2 == 0 && x.owner == Owner::Even) ||
                (x.prio % 2 == 1 && x.owner == Owner::Odd)
            )
        );
        let mut reversed: HashMap<&u32, LinkedList<u32>> = HashMap::new();
        for v in iter {
            for e in &v.succ {
                let mut r = reversed.entry(e).or_insert(LinkedList::new());
                r.push_back(v.id);
            }
        }
        let mut queue = &mut VecDeque::new();
        let mut dist = &mut HashMap::new();
        for v in nodes {
            dist.insert(v, u32::max_value());
        }
        for v in endings {
            dist.insert(v, 0);
            queue.push_back(*v);
        }
        
        while queue.len() > 0 {
            let v = queue.pop_front().unwrap();
            let d_v = * dist.get(v).unwrap();
            for w_id in reversed.get(&v.id).unwrap_or(&LinkedList::new()) {
                let w = game.node(w_id);
                let d_w = *dist.get(w).unwrap();
                if d_w > d_v+1 {
                    dist.insert(w, d_v+1);
                    queue.push_back(w);
                }
            }
        }
        // let lowest = lowest_element(&nodes);
        return SelfLoopStrategy(dist.clone());
    }
}

// pub
impl<'game> Strategy for SelfLoopStrategy<'game> {
    fn vertex(&self) -> Vec<&Node> {
        let mut v: Vec<_> = self.0.iter().collect();
        v.sort_by(|x, y| x.1.cmp(&y.1));
        return v.iter().map(|x| *x.0).collect();
    }
}
impl<'game> Strategy for InputStrategy<'game> {
    fn vertex(&self) -> Vec<&Node> {
        let mut v = self.0.clone();
        v.sort_by(|x, y| x.count.cmp(&y.count));
        return v;
    }
}
impl<'game> Strategy for PriorityStrategy<'game> {
    fn vertex(&self) -> Vec<&Node> {
        let mut v = self.0.clone();
        v.sort_by(|x, y| x.prio.cmp(&y.prio));
        return v;
    }
}
impl<'game> Strategy for RandomStrategy<'game> {
    fn vertex(&self) -> Vec<&Node> {
        let mut clone = self.0.clone();
        let mut v = clone.borrow_mut();
        rand::thread_rng().shuffle(v);
        return v.to_vec();
    }
}

impl<'game> Strategy for SuccesorStrategy<'game> {
    fn vertex(&self) -> Vec<&Node> {
        let mut v = self.0.clone();
        v.sort_by(|x, y| x.succ.len().cmp(&y.succ.len()));
        return v;
    }
}

