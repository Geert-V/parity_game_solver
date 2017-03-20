use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::str;

#[derive(Debug)]
pub enum Owner {
    Even,
    Odd,
}

impl str::FromStr for Owner {
    type Err = String;

    /// Parse a string into an Owner.
    fn from_str(s: &str) -> Result<Owner, Self::Err> {
        if s == "0" {
            Ok(Owner::Even)
        }
        else if s == "1" {
            Ok(Owner::Odd)
        }
        else {
            Err(format!("The string '{}' cannot be parsed as the type Owner.", s))
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub id: u32,
    pub prio: u32,
    pub owner: Owner,
    pub succ: HashSet<u32>,
    pub name: Option<String>
}

#[derive(Debug)]
pub struct Game(pub HashMap<u32, Node>);

impl Game {
    
    /// Returns the maximal priority of any node in the game.
    ///
    /// Returns the maximal priority of any node in the game, or 0 if there are no nodes defined.
    fn max_prio(&self) -> u32 {
        self.0
            .values()
            .map(|n| n.prio)
            .max()
            .unwrap_or(0)
    }
}

#[derive(Debug)]
pub struct Play(pub LinkedList<u32>);

#[derive(Debug)]
pub struct Measure(pub Vec<u32>);

#[derive(Debug)]
pub struct Measures(pub HashMap<Node, HashSet<Measure>>);