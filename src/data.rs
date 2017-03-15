use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::LinkedList;

#[derive(Debug)]
pub enum Owner {
    Even,
    Odd,
}

#[derive(Debug)]
pub struct Node {
    id: u32,
    prio: u32,
    owner: Owner,
    succ: HashSet<u32>,
    name: Option<String>
}

#[derive(Debug)]
pub struct Game {
    nodes: HashMap<u32, Node>
}

#[derive(Debug)]
pub struct Play {
    nodes: LinkedList<u32>
}