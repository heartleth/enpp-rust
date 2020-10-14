pub mod transpile;
use transpile::*;

pub fn main() {
    let k = "
a
b
    c
        d
e
    f
        g
    h
    ";
    let v = tree::CodeTree::treeify(&String::from(k));
    transpile(&v);
}