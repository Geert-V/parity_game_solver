extern crate rand;
use self::rand::Rng;
use pg::*;

pub trait Strategy {
    fn vertex(&self) -> &Node;
}


pub struct RandomStrategy<'game> (Vec<&'game Node>);
impl<'game> RandomStrategy<'game> {
    pub fn new(game: &'game Game) -> RandomStrategy<'game> {
        return RandomStrategy(game.0.iter().collect());
    }
}
impl<'game> Strategy for RandomStrategy<'game> {
    fn vertex(&self) -> &Node {
        let v = rand::thread_rng().choose(&self.0);
        return v.unwrap();
    }
}
