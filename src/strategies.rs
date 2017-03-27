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
pub struct SuccesorStrategy<'game> (Vec<&'game Node>);
impl<'game> SuccesorStrategy<'game> {
    pub fn new(game: &'game Game) -> SuccesorStrategy<'game> {
        return SuccesorStrategy(game.nodes().into_iter().collect());
    }
}

// pub
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

