mod data;
mod parser;

fn main() {
    let pg = "pg_test.txt";
    let game = parser::parse_from_file(pg.to_string());

    println!("{:?}", game);
}
