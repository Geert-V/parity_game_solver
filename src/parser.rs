use std::collections::HashMap;
use std::collections::HashSet;

use data::Game;
use data::Node;
use data::Owner;

extern crate regex;
use self::regex::Regex;
use self::regex::Captures;

fn try_parse_header(header: String) -> Option<u32> {
    let mut header_split = header.split_whitespace();

    if header_split.next() != Some("parity") {
        return None;
    }
    
    let max_id = header_split
        .next()
        .expect("The header has no node ID specified.")
        .parse::<u32>()
        .expect("The node ID is not a valid positive integer.");

    if header_split.next() != None {
        return None;
    }

    Some (max_id)
}

fn get_capture_as_string(caps : &Captures, name: &str) -> String {
    caps.name(name).unwrap().as_str().to_string()
}

fn try_parse_node_spec(node_spec: String) -> Option<Node> {
    let re = Regex::new("^(?P<id>\\d+) (?P<prio>\\d+) (?P<owner>[01]+) (?P<succ>.+?)( \"(?P<name>.+)\")?$").unwrap();
    let cs = re.captures(node_spec.trim());

    fn as_node(caps : Captures) -> Node {
        let id = get_capture_as_string(&caps, "id")
            .parse::<u32>()
            .unwrap();
        let prio = get_capture_as_string(&caps, "prio")
            .parse::<u32>()
            .unwrap();
        let owner = get_capture_as_string(&caps, "owner")
            .parse::<Owner>()
            .unwrap();
        let succ = get_capture_as_string(&caps, "succ")
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let name = caps
            .name("name")
            .map(|n| n.as_str().to_string());

        Node {
            id: id,
            prio: prio,
            owner: owner,
            succ: succ,
            name: name
        }
    }

    cs.map(as_node)
}

pub fn parse(parity_game: String) -> Game {
    let lines = parity_game.split(';');

    let mut max_id = None;
    let mut nodes = HashMap::new();

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let header = try_parse_header(line.to_string());

        if header.is_some() {
            if max_id.is_some() {
                panic!("A header was already parsed. Header found: {}", line);
            }

            max_id = header;
        }
        else
        {
            let node = try_parse_node_spec(line.to_string())
                .expect(&format!("Failed to parse line: '{}'", line));

            nodes.insert(node.id, node);
        }
    }

    Game(nodes)
}