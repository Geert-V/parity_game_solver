mod data;
mod parser;

fn main() {
    let pg = String::from("parity 4;\n1 3 0 1,3,4 \"Europe\";\n0 6 1 4,2;\n4 5 1 0 \"Antarctica\";\n1 8 1 2,4,3 \"America\";\n3 6 0 4,2 \"Australia\";\n2 7 0 3,1,0,4 \"Asia\";");
    let game = parser::parse(pg);

    println!("Game: {:?}", game);
}
