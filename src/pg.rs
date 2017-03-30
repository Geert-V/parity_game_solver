use std::cmp;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::str;
use std::hash::{Hash, Hasher, SipHasher};

use std::cmp::Ordering;


#[derive(Debug, Eq, PartialEq)]
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
    pub count: usize,
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
pub struct Game {
    id_to_node: HashMap<u32, Node>,
    max_prio: u32,
    max_measure: Measure
}

impl Game {

    pub fn new(id_to_node: HashMap<u32, Node>) -> Game {
        let max_prio = Game::_max_prio(&id_to_node);
        let max_measure = Game::_max_measure(&id_to_node, max_prio);

        Game {
            id_to_node: id_to_node,
            max_prio: max_prio,
            max_measure: max_measure
        }
    }

    pub fn node(&self, id: &u32) -> &Node {
        &self.id_to_node[id]
    }

    pub fn nodes(&self) -> HashSet<&Node> {
        self.id_to_node
            .values()
            .collect::<HashSet<&Node>>()
    }

    pub fn new_measure(&self) -> MeasureT {
        let d = self.max_prio() as usize + 1;
        let m = Measure(vec![0; d]);

        MeasureT::Measure(m)
    }

    pub fn new_progress(&self) -> Progress {
        let mut m = HashMap::new();

        for node in self.nodes() {
            m.insert(node.id, self.new_measure());
        }

        Progress(m)
    }

    /// Returns the maximal priority of any node in the game.
    ///
    /// Returns the maximal priority of any node in the game, or 0 if there are no nodes defined.
    pub fn max_prio(&self) -> u32 {
        self.max_prio
    }

    pub fn max_measure(&self) -> &Measure {
        &self.max_measure
    }

    fn _nodes_with_prio(id_to_node: &HashMap<u32, Node>, prio: u32) -> HashSet<&Node> {
        id_to_node
            .values()
            .filter(|&n| n.prio == prio)
            .collect::<HashSet<&Node>>()
    }
    
    fn _max_prio(id_to_node: &HashMap<u32, Node>) -> u32 {
        id_to_node
            .values()
            .map(|n| n.prio)
            .max()
            .unwrap_or(0)
    }

    fn _max_measure(id_to_node: &HashMap<u32, Node>, max_prio: u32) -> Measure {
        let prio = max_prio.clone();
        let mut measure = vec![0; prio as usize + 1];

        // Get the last odd index of the measure.
        let mut i = prio as i32;

        if i % 2 == 0 {
            i -= 1;
        }

        while i >= 1 {
            let nr_of_nodes_with_prio = Game::_nodes_with_prio(id_to_node, i as u32).len(); 
            measure[i as usize] = nr_of_nodes_with_prio as u32;

            i -= 2;
        }

        Measure(measure)
    }
}

#[derive(Debug, Eq, Clone, Ord)]
pub struct Measure(pub Vec<u32>);

impl Measure {

    pub fn length(&self) -> usize {
        self.0.len()
    }

    /// Returns the value on the specified index wrapped in a `Some` or `None` if the index lies outside the range of values.
    fn get_value(&self, i: usize) -> Option<u32> {
        if i < self.length() {
            Some(self.0[i])
        } else {
            None
        }
    }

    /// Returns the value on the specified index or 0 if the index lies outside the range of values.
    fn get_value_or_zero(&self, i: usize) -> u32 {
        let value = self.get_value(i);

        match value {
            Some(v) => v,
            None    => 0,
        }
    }

    /// Returns `true` if this measure is equal to the provided measure up to and including the specified index.
    /// Otherwise `false` is returned.
    pub fn eq(&self, other: &Measure, i: usize) -> bool {
        for x in 0..(i + 1) {
            let self_v = self.get_value_or_zero(x);
            let other_v = other.get_value_or_zero(x);

            if self_v == other_v {
                continue;
            }

            return false;
        }
        
        true
    }

    /// Returns `true` if this measure is greater than the provided measure up to and including the specified index.
    /// Otherwise `false` is returned.
    pub fn gt(&self, other: &Measure, i: usize) -> bool {
        for x in 0..(i+1) {
            let self_v = self.get_value_or_zero(x);
            let other_v = other.get_value_or_zero(x);

            if self_v == other_v { continue; }

            return self_v > other_v;
        }

        false
    }

    // /// Returns `true` if this measure is greater than or equal to the provided measure up to and including the specified index.
    // /// Otherwise `false` is returned.
    // pub fn ge(&self, other: &Measure, i: usize) -> bool {
    //     self.gt(other, i) || self.eq(other, i)
    // }

    // /// Returns `true` if this measure is less than the provided measure up to and including the specified index.
    // /// Otherwise `false` is returned.
    // pub fn lt(&self, other: &Measure, i: usize) -> bool {
    //     !self.ge(other, i)
    // }

    // /// Returns `true` if this measure is less than or equal to the provided measure up to and including the specified index.
    // /// Otherwise `false` is returned.
    // pub fn le(&self, other: &Measure, i: usize) -> bool {
    //     !self.gt(other, i)
    // }
}

impl PartialOrd for Measure {
    fn partial_cmp(&self, other: &Measure) -> Option<Ordering> {
        let max_l = cmp::max(self.length(), other.length());

        if self.eq(other, max_l - 1) {
            Some(Ordering::Equal)
        }
        else if self.gt(other, max_l - 1) {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}
impl PartialEq for Measure {
    fn eq(&self, other: &Measure) -> bool {
        let max_l = cmp::max(self.length(), other.length());

        self.eq(other, max_l - 1)
    }
}

/// A measure that can also be the special value `Top`.
#[derive(Debug, Eq, Ord, Clone)]
pub enum MeasureT {
    /// The value `Top` that is greater than any `Measure`.
    Top,
    /// A measure with a value.
    Measure(Measure)
}

impl MeasureT {

    pub fn inc(&self, game: &Game) -> MeasureT {

        fn inc_measure(cur: &Measure, max: &Measure) -> MeasureT {
            if cur == max {
                // It is the maximal value, return `Top`.
                MeasureT::Top
            }
            else {
                // Clone the current measure and get its last index.
                let mut new = cur.clone();
                let mut i = cur.length() as i32 - 1;

                // We can only increase the odd numbers, so if even go the the closest odd number.
                if i % 2 == 0 {
                    i -= 1;
                }

                // Move backwards through the vector and increase the value where possible.
                while i >= 1 {
                    let max_v = max.0[i as usize];
                    let cur_v = cur.0[i as usize];

                    if cur_v < max_v {
                        // We can increase this value, we are finished.
                        new.0[i as usize] += 1;
                        break;
                    } else {
                        // Set this value to 0 as it had the maximum value, a higher value will be increased.
                        new.0[i as usize] = 0;
                    }

                    // Only the odd numbers can change.
                    i -= 2;
                }

                MeasureT::Measure(new)
            }
        }

        match self {
            &MeasureT::Top        => MeasureT::Top,
            &MeasureT::Measure(ref m) => inc_measure(m, &game.max_measure())
        }
    }

    /// Returns `true` if this measure is equal to the provided measure up to and including the specified index.
    /// Otherwise `false` is returned.
    pub fn eq(&self, other: &MeasureT, i: usize) -> bool {
        match (self, other) {
            (&MeasureT::Top, &MeasureT::Top)                                  => true,
            (&MeasureT::Top, _)                                               => false,
            (_, &MeasureT::Top)                                               => false,
            (&MeasureT::Measure(ref self_m), &MeasureT::Measure(ref other_m)) => self_m.eq(other_m, i),
        }
    }

    /// Returns `true` if this measure is greater than the provided measure up to and including the specified index.
    /// Otherwise `false` is returned.
    pub fn gt(&self, other: &MeasureT, i: usize) -> bool {
        match (self, other) {
            (&MeasureT::Top, &MeasureT::Top)                                  => false,
            (&MeasureT::Top, _)                                               => true,
            (_, &MeasureT::Top)                                               => false,
            (&MeasureT::Measure(ref self_m), &MeasureT::Measure(ref other_m)) => self_m.gt(other_m, i),
        }
    }

    /// Returns `true` if this measure is greater than or equal to the provided measure up to and including the specified index.
    /// Otherwise `false` is returned.
    pub fn ge(&self, other: &MeasureT, i: usize) -> bool {
        self.gt(other, i) || self.eq(other, i)
    }

    /// Returns `true` if this measure is less than the provided measure up to and including the specified index.
    /// Otherwise `false` is returned.
    pub fn lt(&self, other: &MeasureT, i: usize) -> bool {
        !self.ge(other, i)
    }

    /// Returns `true` if this measure is less than or equal to the provided measure up to and including the specified index.
    /// Otherwise `false` is returned.
    pub fn le(&self, other: &MeasureT, i: usize) -> bool {
        !self.gt(other, i)
    }
}

impl PartialOrd for MeasureT {
    fn partial_cmp(&self, other: &MeasureT) -> Option<Ordering> {
        match (self, other) {
            (&MeasureT::Top, &MeasureT::Top)                                  => Some(Ordering::Equal),
            (&MeasureT::Top, _)                                               => Some(Ordering::Greater),
            (_, &MeasureT::Top)                                               => Some(Ordering::Less),
            (&MeasureT::Measure(ref self_m), &MeasureT::Measure(ref other_m)) => self_m.partial_cmp(other_m),
        }
    }
}
impl PartialEq for MeasureT {
    fn eq(&self, other: &MeasureT) -> bool {
        match (self, other) {
            (&MeasureT::Top, &MeasureT::Top)                                  => true,
            (&MeasureT::Top, _)                                               => false,
            (_, &MeasureT::Top)                                               => false,
            (&MeasureT::Measure(ref self_m), &MeasureT::Measure(ref other_m)) => self_m == other_m
        }
    }
}

#[derive(Debug, Clone)]
pub struct Progress(pub HashMap<u32, MeasureT>);

impl Progress {
    pub fn nodes(&self) -> HashSet<&u32> {
        self.0
            .keys()
            .collect::<HashSet<&u32>>()
    }

    pub fn measure(&self, node_id: &u32) -> &MeasureT {
        &self.0[node_id]
    }

    pub fn winning_set(&self, owner: Owner) -> HashSet<&u32> {
        let mut all = self.0.iter();

        match owner {
            Owner::Even => all
                .filter(|&(_, m)| m != &MeasureT::Top)
                .map(|(id, _)| id)
                .collect(),
            Owner::Odd => all
                .filter(|&(_, m)| m == &MeasureT::Top)
                .map(|(id, _)| id)
                .collect(),
        }
    }
}
impl PartialEq for Progress {
    fn eq(&self, other: &Progress) -> bool {
        let self_v = self.nodes();
        let other_v = self.nodes();

        if self_v != other_v {
            return false;
        }

        for node_id in self_v {
            let self_m = self.measure(node_id);
            let other_m = other.measure(node_id);

            if self_m != other_m {
                return false;
            }
        }

        true
    }
}
