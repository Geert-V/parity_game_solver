#[derive(Debug)]
enum Owner {
    Even,
    Odd,
}

#[derive(Debug)]
struct Node {
    id: u32,
    prio: u32,
    owner: Owner,
    succ: HashSet<u32>,
    name: Option<String>
}

#[derive(Debug)]
struct Game {
    nodes: HashMap<u32, Node>
}

#[derive(Debug)]
struct Play {
    nodes: LinkedList<u32>
}