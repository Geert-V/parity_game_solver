

#[test]
fn parsing() {
    let input = "parity 4;\n1 3 0 1,3,4 \"Europe\";\n0 6 1 4,2;\n4 5 1 0 \"Antarctica\";\n1 8 1 2,4,3 \"America\";\n3 6 0 4,2 \"Australia\";\n2 7 0 3,1,0,4 \"Asia\";";
    
    let parsed = parser::parse(pg);;
    let result = Game({
        Node { id: 0, prio: 6, owner: Odd, succ: {2, 4}, name: Some("Africa") }, 
        Node { id: 3, prio: 6, owner: Even, succ: {2, 4}, name: Some("Australia") }, 
        Node { id: 1, prio: 3, owner: Even, succ: {1, 3, 4}, name: Some("Europe") }, 
        Node { id: 4, prio: 5, owner: Odd, succ: {0}, name: Some("Antarctica") }, 
        Node { id: 2, prio: 7, owner: Even, succ: {1, 0, 4, 3}, name: Some("Asia") }
    });
    assert_eq!(parsed, result);

    let result = Game({});
    assert_ne!(parsed, result);

    let result = Game({
        Node { id: 0, prio: 5, owner: Odd, succ: {2, 4}, name: Some("Africa") }, 
        Node { id: 3, prio: 6, owner: Even, succ: {2, 4}, name: Some("Australia") }, 
        Node { id: 1, prio: 2, owner: Even, succ: {1, 3, 4}, name: Some("Europe") }, 
        Node { id: 4, prio: 3, owner: Odd, succ: {0}, name: Some("Antarctica") }, 
        Node { id: 2, prio: 7, owner: Even, succ: {1, 0, 4, 3}, name: Some("Asia") }
    });
    assert_ne!(parsed, result);
}