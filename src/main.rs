mod util;
mod tree;

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
    tree::CodeTree::treeify(&String::from(k));
}