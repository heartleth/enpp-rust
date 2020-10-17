pub mod transpile;
use transpile::*;

pub fn main() {
    println!("{}", verb_parse(&String::from("vector-in-std-of-i4-type")));
}