extern crate rand;
use self::rand::Rng;
use pg::*;

pub trait Strategy {
    fn vertex(&self, usize) -> &Node;
}

pub struct InputStrategy<'game>(Vec<&'game Node>);
impl<'game> InputStrategy<'game> {
    pub fn new(game: &'game Game) -> InputStrategy<'game> {
        InputStrategy(game.nodes().into_iter().collect())
    }
}

impl<'game> Strategy for InputStrategy<'game> {
    fn vertex(&self, i: usize) -> &Node {
        &self.0[i]
    }
}


pub struct RandomStrategy<'game> (Vec<&'game Node>);
impl<'game> RandomStrategy<'game> {
    pub fn new(game: &'game Game) -> RandomStrategy<'game> {
        return RandomStrategy(game.nodes().into_iter().collect());
    }
}
impl<'game> Strategy for RandomStrategy<'game> {
    fn vertex(&self, i: usize) -> &Node {
        let v = rand::thread_rng().choose(&self.0);
        return v.unwrap();
    }
}
