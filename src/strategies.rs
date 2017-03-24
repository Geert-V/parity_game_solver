extern crate rand;
use self::rand::Rng;
use pg::*;
use std::borrow::BorrowMut;

pub trait Strategy {
    fn vertex(&self) -> Vec<&Node>;
}

pub struct InputStrategy<'game>(Vec<&'game Node>);
impl<'game> InputStrategy<'game> {
    pub fn new(game: &'game Game) -> InputStrategy<'game> {
        InputStrategy(game.nodes().into_iter().collect())
    }
}

impl<'game> Strategy for InputStrategy<'game> {
    fn vertex(&self) -> Vec<&Node> {
        self.0.to_vec()
    }
}


pub struct RandomStrategy<'game> (Vec<&'game Node>);
impl<'game> RandomStrategy<'game> {
    pub fn new(game: &'game Game) -> RandomStrategy<'game> {
        return RandomStrategy(game.nodes().into_iter().collect());
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
