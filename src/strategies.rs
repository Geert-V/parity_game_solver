extern crate rand;
use self::rand::Rng;
use std::collections::HashSet;
use std::collections::HashMap;
use pg::*;
use std::borrow::BorrowMut;
use std::iter::Iterator;

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
pub struct DistanceStrategy<'game> (Vec<&'game Node>);
impl<'game> DistanceStrategy<'game> {
    pub fn new(game: &'game Game) -> DistanceStrategy<'game> {
        let mut nodes = game.nodes();

        fn lowest_element<'game>(nodes: &HashSet<&'game Node>) -> &'game Node {
            nodes.iter().fold(None, |min, x| match min {
                None => Some(x),
                Some(y) => Some(if x.prio < y.prio { x } else { y }),
            }).unwrap()
        };

        let lowest = lowest_element(&nodes);

        fn Dijkstra<'game>(game: &'game Game, nodes:&'game Vec<Node>, source: &'game Node ) -> (HashMap<&'game Node, u32>, HashMap<&'game Node, Option<&'game Node>>) {
            let mut dist = HashMap::new();
            let mut prev = HashMap::new();
            let mut queue = HashSet::new();
            for v in nodes {
                dist.insert(v, u32::max_value());
                prev.insert(v, None);
                queue.insert(v);
            };
            dist.insert(source, 0);
            while queue.len() > 0 {
                let u = lowest_element(&queue);
                queue.remove(u);
                for v_id in &u.succ {
                    let v = game.node(v_id);
                    let alt = (dist.get(u).unwrap() + 1).clone();
                    let old = dist.get(v).unwrap().clone();
                    if alt < old {
                        prev.insert(v, Some(u));
                        dist.insert(v, alt);
                    };
                };
            }
            return (dist, prev);
        }
        return DistanceStrategy(nodes.into_iter().collect());
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

