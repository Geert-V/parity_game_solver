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

#[derive(Debug, Eq, Ord)]
pub struct Measure(pub Vec<u32>);

impl Measure {

    /// Returns the value on the specified index wrapped in a `Some` or `None` if the index lies outside the range of values.
    fn get_value(&self, i: usize) -> Option<u32> {
        if i < 0 {
            None
        } else if self.0.len() <= i {
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

        for x in 0..i {
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
        for x in 0..i {
            let self_v = self.get_value_or_zero(x);
            let other_v = other.get_value_or_zero(x);

            if self_v == other_v { continue; }

            return self_v > other_v;
        }

        false
    }

    /// Returns `true` if this measure is greater than or equal to the provided measure up to and including the specified index.
    /// Otherwise `false` is returned.
    pub fn ge(&self, other: &Measure, i: usize) -> bool {
        self.gt(other, i) || self.eq(other, i)
    }

    /// Returns `true` if this measure is less than the provided measure up to and including the specified index.
    /// Otherwise `false` is returned.
    pub fn lt(&self, other: &Measure, i: usize) -> bool {
        !self.ge(other, i)
    }

    /// Returns `true` if this measure is less than or equal to the provided measure up to and including the specified index.
    /// Otherwise `false` is returned.
    pub fn le(&self, other: &Measure, i: usize) -> bool {
        !self.gt(other, i)
    }
}

impl PartialOrd for Measure {
    fn partial_cmp(&self, other: &Measure) -> Option<Ordering> {
        let max_l = cmp::max(self.0.len(), other.0.len());

        if self.eq(other, max_l) {
            Some(Ordering::Equal)
        }
        else if self.gt(other, max_l) {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}
impl PartialEq for Measure {
    fn eq(&self, other: &Measure) -> bool {
        let max_l = cmp::max(self.0.len(), other.0.len());
        self.eq(other, max_l)
    }
}

/// A measure that can also be the special value `Top`.
#[derive(Debug, Eq, Ord)]
pub enum MeasureT {

    /// The value `Top` that is greater than any `Measure`.
    Top,

    /// A measure with a value.
    Measure(Measure)
}

impl MeasureT {

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
