mod util;
mod tree;

use tree::CodeTree;

pub fn main() {
    let mut k = CodeTree::new(String::from("root"), 0);
    
    k.insert(CodeTree::new(String::from("a"), 0));
    k.insert(CodeTree::new(String::from("b"), 0));
    k.insert(CodeTree::new(String::from("b"), 0));

    for e in k.children {
        println!("{}", e.code);
    }
}