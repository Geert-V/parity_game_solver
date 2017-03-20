mod pg;
mod parser;

fn main() {
    let game = parser::parse_from_file("pg_test.txt");

    println!("{:?}", game);
}
