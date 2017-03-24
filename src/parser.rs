use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use pg::Game;
use pg::Node;
use pg::Owner;

/// Tries to parse the header data from the provided string.
///
/// If the provided string is a valid header, then the maximum node ID specified in this header is returned.
/// This value is then wrapped in `Some`. Otherwise, if the string is not a valid header, `None` is returned.
/// A valid header has the format: 'parity <identifier>'.
///
/// #Panics
/// If the provided string is a header, but is in an invalid format.
fn try_parse_header(header: &str) -> Option<u32> {
    let mut split = header.split_whitespace();

    let def = split.next();
    if def.is_some() && def.unwrap() == "parity" {
        let max_id = split
            .next()
            .expect(&format!("The header '{}' does not contain a max identifier.", header))
            .parse::<u32>()
            .expect(&format!("The max identifier of the header '{}' is not a valid natural number.", header));
        
        Some(max_id)
    } else {
        None
    }
}

/// Parse the node specification from the provided string.
///
/// #Panics
/// If the provided string is not a valid node specification.
/// A valid node specification has the format: '<identifier> <priority> <owner> <successor> [<name>] [;]'.
fn parse_node_spec(i: usize, node_spec: &str) -> Node {
    let mut split = node_spec.split_whitespace();

    let id = split
        .next()
        .expect(&format!("No ID defined for node spec: '{}'", node_spec))
        .parse::<u32>()
        .expect(&format!("The ID defined for node spec: '{}' is not a natural number.", node_spec));
    
    let prio = split
        .next()
        .expect(&format!("No prio defined for node spec: '{}'", node_spec))
        .parse::<u32>()
        .expect(&format!("The prio defined for node spec '{}' is not a natural number.", node_spec));

    let owner = split
        .next()
        .expect(&format!("No owner defined for node spec: '{}'", node_spec))
        .parse::<Owner>()
        .expect(&format!("The owner defined for node spec '{}' is invalid. Must be in rage [0-1].", node_spec));
    
    let succ = split
        .next()
        .expect(&format!("No successors defined for node spec: '{}'", node_spec))
        .split(',')
        .map(|s| s.parse::<u32>().expect(&format!("Invalid successor '{}' in node spec: '{}'.", s, node_spec)))
        .collect::<HashSet<u32>>();

    let name = split
        .next()
        .map(|n| { n
            .split('"')
            .filter(|part| !part.is_empty())
            .next()
            .expect(&format!("No name is defined, but is expected to in the string: '{}' of node spec: '{}'.", n, node_spec))
            .to_string()
        });

    Node {
        id: id,
        count: i,
        prio: prio,
        owner: owner,
        succ: succ,
        name: name
    }
}

/// Parses the provided string as a parity game.
///
/// # Panics
/// - The string contains multiple headers.
/// - The string contains an invalid header or node specification.
pub fn parse(parity_game: &str) -> Game {
    let mut nodes = HashMap::new();
    let mut lines = parity_game
        .trim()
        .split(';')
        .peekable();

    // Check if the first line is a header.
    let max_id = lines
        .peek()
        .and_then(|header| try_parse_header(header));
    
    // If the first line was indeed a header, skip it.
    if max_id.is_some() {
        lines.next();
    }

    // Parse the rest of the lines as node specifications.
    for (i, line) in lines.enumerate() {
        // Ignore this line if it only consists of white spaces or is empty.
        if line.trim().is_empty() {
            continue;
        }

        let node = parse_node_spec(i, line);
        nodes.insert(node.id, node);
    }

    Game::new(nodes)
}

/// Parses a parity game from the specified file.
///
/// # Panics
/// - The file does not exist.
/// - The string contains multiple headers.
/// - The string contains an invalid header or node specification.
pub fn parse_from_file(file_path: &str) -> Game {
    let file = File::open(file_path)
        .expect(&format!("Failed to open the file: '{}'.", file_path));
        
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect(&format!("Failed to read the file: '{}'.", file_path));

    parse(&contents)
}