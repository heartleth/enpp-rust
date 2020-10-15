pub mod parser;
pub mod stdlib;
pub mod tree;

use crate::transpile::transpile;
use stdlib::*;
use tree::*;
pub use parser::*;

pub type Mem = Vec<CodeTree>;

pub fn parse_else(tree :&Mem, nth :&mut usize, parent_idx :usize)->String {
    String::new()
}

pub fn parse_if(tree :&Mem, nth :&mut usize, parent_idx :usize)->String {
    let mut ret;
    let parent = &tree[parent_idx];
    let elem = parent.children[*nth];
    let to_parse = &tree[elem];
    let split = &split(&to_parse.code);

    ret = String::from("if");
    ret += &value_parse(&split);
    transpile(&tree, elem);
    ret += "}";
    
    if *nth < parent.children.len() - 1 {
        if regi(&keyword(&tree[parent.children[*nth + 1]].code), "else") {
            *nth += 1;
            parse_else(&tree, nth, parent_idx);
        }
    }

    String::new()
}

pub fn parse_import(s :&Mem, pivot :usize)->String {
    
    String::new()
}