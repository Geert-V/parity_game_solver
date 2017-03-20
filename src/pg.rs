use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::str;
use std::hash::{Hash, Hasher, SipHasher};

use std::cmp::Ordering;


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
impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id
    }
}
impl Eq for Node {}
impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}


#[derive(Debug)]
pub struct Game(pub HashSet<Node>);

impl Game {
    
    /// Returns the maximal priority of any node in the game.
    ///
    /// Returns the maximal priority of any node in the game, or 0 if there are no nodes defined.
    pub fn max_prio(&self) -> u32 {
        self.0
            .iter()
            .map(|n| n.prio)
            .max()
            .unwrap_or(0)
    }
}

#[derive(Debug)]
pub struct Play(pub LinkedList<u32>);

#[derive(Debug, Eq)]
pub struct Measure(pub Vec<u32>);

impl Ord for Measure {
    fn cmp(&self, other: &Measure) -> Ordering {
        Ordering::Greater// TODO: implement
    }
}

impl PartialOrd for Measure {
    fn partial_cmp(&self, other: &Measure) -> Option<Ordering> {
        Some(self.cmp(other)) // TODO: implement
    }
}
impl PartialEq for Measure {
    fn eq(&self, other: &Measure) -> bool {
        return true;// TODO: implement
    }
}



#[derive(Debug)]
pub struct Progress(pub HashMap<u32, Measure>);

impl PartialOrd for Progress {
    fn partial_cmp(&self, other: &Progress) -> Option<Ordering> {
        Some(Ordering::Greater)
    }
}
impl PartialEq for Progress {
    fn eq(&self, other: &Progress) -> bool {
        return true;
    }
}
