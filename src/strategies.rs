extern crate rand;
use self::rand::Rng;
use std::collections::HashMap;
use pg::*;
use std::borrow::BorrowMut;
use std::iter::Iterator;
use std::collections::LinkedList;
use std::collections::VecDeque;

pub trait Strategy {
    fn vertices(&self) -> Vec<&Node>;
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
pub struct SuccessorStrategy<'game> (Vec<&'game Node>);
impl<'game> SuccessorStrategy<'game> {
    pub fn new(game: &'game Game) -> SuccessorStrategy<'game> {
        return SuccessorStrategy(game.nodes().into_iter().collect());
    }
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
    fn vertices(&self) -> Vec<&Node> {
        let mut v: Vec<_> = self.0.iter().collect();
        v.sort_by(|x, y| x.1.cmp(&y.1));
        return v.iter().map(|x| *x.0).collect();
    }
}
impl<'game> Strategy for InputStrategy<'game> {
    fn vertices(&self) -> Vec<&Node> {
        let mut v = self.0.clone();
        v.sort_by(|x, y| x.count.cmp(&y.count));
        return v;
    }
}
impl<'game> Strategy for PriorityStrategy<'game> {
    fn vertices(&self) -> Vec<&Node> {
        let mut v = self.0.clone();
        v.sort_by(|x, y| x.prio.cmp(&y.prio));
        return v;
    }
}
impl<'game> Strategy for RandomStrategy<'game> {
    fn vertices(&self) -> Vec<&Node> {
        let mut clone = self.0.clone();
        let mut v = clone.borrow_mut();
        rand::thread_rng().shuffle(v);
        return v.to_vec();
    }
}

impl<'game> Strategy for SuccessorStrategy<'game> {
    fn vertices(&self) -> Vec<&Node> {
        let mut v = self.0.clone();
        v.sort_by(|x, y| x.succ.len().cmp(&y.succ.len()));
        return v;
    }
}

