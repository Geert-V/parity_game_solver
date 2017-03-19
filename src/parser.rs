use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use data::Game;
use data::Node;
use data::Owner;

extern crate regex;
use self::regex::Regex;
use self::regex::Captures;

/// Tries to parse the header data from the provided string.
///
/// If the provided string is a valid header, then the maximum node ID specified in this header is returned.
/// This value is then wrapped in `Some`. Otherwise, if the string is not a valid header, `None` is returned.
/// A valid header has the format: 'parity <identifier>'.
fn try_parse_header(header: &str) -> Option<u32> {
    
    // Explanation:
    // '^'                : the start of the string.
    // 'parity '          : match this exact text.
    // '(?P<max_id> ...)' : the name of a capture group.
    // '\\d+'             : any number of digits.
    // '\\s*?;?'          : a number of optional white spaces (non-greedy) followed by an optional semicolon.
    // '$'                : the end of the string.
    let reg_ex = Regex::new("^parity (?P<max_id>\\d+)\\s*?;?$");

    /// Takes the maximum node ID from the regular expression captures, and returns it.
    ///
    /// Note that this is an internal function and that the panics should never happen if the regular expression is successfully matched.
    fn as_id(caps: Captures) -> u32 {
        caps.name("max_id")
            .expect("No maximal identifier found in the header.")
            .as_str()
            .parse::<u32>()
            .expect("The max node ID is not a valid natural number.")
    }

    reg_ex
        .unwrap()
        .captures(header.trim())
        .map(as_id)
}

/// Retrieves the value of the captured string.
///
/// # Panics
/// No value with the specified name has been captured.
fn get_capture_as_string(caps: &Captures, name: &str) -> String {
    caps.name(name).unwrap().as_str().to_string()
}

/// Tries to parse the node specification from the provided string.
///
/// If the provided string is a valid node specification, then the node is returned.
/// This value is then wrapped in `Some`. Otherwise, if the string is not a valid node specification, `None` is returned.
/// A valid node specification has the format: '<identifier> <priority> <owner> <successor> [<name>] [;]'.
fn try_parse_node_spec(node_spec: &str) -> Option<Node> {

    // Explanation:
    // '^'              : the start of the string.
    // '(?P<name> ...)' : the name of a capture group.
    // '\\d+'           : any number of digits.
    // '[01]'           : either a zero or one.
    // (\\d+,?)+        : one or more occurrences of digits followed by an optional comma. 
    // \"               : matches a double quote.
    // .+               : one or more non-white space character(s).
    // '\\s*?;?'        : a number of optional white spaces (non-greedy) followed by an optional semicolon.
    // '$'              : the end of the string.
    let reg_ex = Regex::new("^(?P<id>\\d+) (?P<prio>\\d+) (?P<owner>[01]) (?P<succ>(\\d+,?)+)( \"(?P<name>.+)\")?\\s*?;?$").unwrap();

    /// Converts from the regular expression capture to a `Node`.
    ///
    /// Note that this is an internal function and that the panics should never happen if the regular expression is successfully matched.
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

    reg_ex
        .captures(node_spec.trim())
        .map(as_node)
}

/// Parses the provided string as node specification in the format: '<identifier> <priority> <owner> <successor> [<name>] [;]'.
///
/// # Panics
/// The provided string is not a valid node specification.
fn parse_node_spec(node_spec: &str) -> Node {
    try_parse_node_spec(node_spec)
        .expect(&format!("Invalid node specification: '{}'.", node_spec))
}

/// Parses the provided string as a parity game.
///
/// # Panics
/// - The string contains multiple headers.
/// - The string contains an invalid header or node specification.
pub fn parse(parity_game: &str) -> Game {
    let mut nodes = HashMap::new();
    let mut lines = parity_game.trim().split(';');

    // Check if the first line is a header.
    let first_line = lines.next();
    let max_id = first_line
        .map(|line| line)
        .and_then(try_parse_header);
    
    // If the line is not a header (but does exist), parse it as a node specification.
    if max_id.is_none() && first_line.is_some() {
        let node = parse_node_spec(first_line.unwrap());
        nodes.insert(node.id, node);
    }

    // Parse the rest of the lines as node specifications.
    for line in lines {

        // Ignore this line if it only consists of white spaces or is empty.
        if line.trim().is_empty() {
            continue;
        }

        let node = parse_node_spec(line);
        nodes.insert(node.id, node);
    }

    Game(nodes)
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