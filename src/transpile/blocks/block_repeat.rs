extern crate rand;
use rand::Rng;
use rand::distributions::Alphanumeric;

use super::*;

pub fn parse_repeat(tree :&Mem, pivot :usize)->String {
    let s = format!("q{}", rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .collect::<String>());

    format!("for (int {0}=0;{0}<{1};{0}++) {{\n{scope}}}\n", s,
        &value_parse(&String::from(&s[7..]), 1),
        scope = &transpile(&tree, pivot)
    )
}

#[inline]
pub fn parse_while(tree :&Mem, pivot :usize)->String {
    format!("while ({}) {{\n{scope}}}\n",
        &value_parse(&String::from(&tree[pivot].code[6..]), 1),
        scope = &transpile(&tree, pivot)
    )
}

pub fn parse_for(tree :&Mem, pivot :usize)->String {
    let splited = &split(&tree[pivot].code);
    let mut where_of = 0;
    
    for elem in splited {
        if elem.to_ascii_lowercase() == "of" { break; }
        else { where_of += 1; }
    }
    let mut to_assign;
    let dec = arguments_parse(&splited[1..where_of].to_vec());

    if dec.len() == 1 {
        to_assign = dec[0].to_string();
    }
    else {
        to_assign = String::from("auto [");
        for elem in &dec {
            to_assign += &elem.name[..];
            to_assign += ", ";
        }
        to_assign.pop();
        to_assign.pop();
        to_assign += "]";
    }
    
    format!("for ({} : {}) {{\n{scope}}}\n",
        to_assign,
        &value_parse(&splited[where_of+1..].to_vec().join(" "), 1),
        scope = &transpile(&tree, pivot)
    )
}