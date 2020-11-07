use super::*;

pub fn parse_else(tree :&Mem, nth :&mut usize, parent_idx :usize)->Result<String, &'static str> {
    let mut ret = String::from("else ");
    let parent = &tree[parent_idx];
    let splited = &split(&tree[parent.children[*nth]].code);
    let code = &tree[parent.children[*nth]].code;

    if splited.len() == 1 {
        ret += "{";
        ret += transpile(&tree, parent.children[*nth]).as_str();
        ret += "}\n";
    }
    else {
        ret += "if (";
        ret += &value_parse(&String::from(&code[8..]), 1)?;
        ret += ") {";
        ret += transpile(&tree, parent.children[*nth]).as_str();
        ret += "}\n"
    }
    if *nth < parent.children.len() - 1 && splited.len() > 1 {
        if regi(&keyword(&tree[parent.children[*nth + 1]].code), "^else$") {
            *nth += 1;
            ret += parse_else(&tree, nth, parent_idx)?.as_str();
        }
    }
    Ok(ret)
}

pub fn parse_if(tree :&Mem, nth :&mut usize, parent_idx :usize)->Result<String, &'static str> {
    let mut ret = String::from("if (");
    let parent = &tree[parent_idx];
    let elem = parent.children[*nth];
    let to_parse = &tree[elem];

    let condition = value_parse(&String::from(&to_parse.code[3..]), 1)?;
    ret += &condition;
    ret += ") {";
    ret += transpile(&tree, elem).as_str();
    ret += "}\n";
    
    if *nth < parent.children.len() - 1 {
        if regi(&keyword(&tree[parent.children[*nth + 1]].code), "^else$") {
            *nth += 1;
            ret += parse_else(&tree, nth, parent_idx)?.as_str();
        }
    }
    Ok(ret)
}

use std::fs::File;
use std::io::prelude::*;
pub fn parse_import(s :&Mem, pivot :usize)->Result<String, &str> {
    let method = keyword(&s[pivot].code);
    let mut ret = String::new();
    match &method.to_ascii_lowercase()[..] {
        "import" => {
            ret = format!("#include\"{}\"\n", &s[pivot].code[7..]);
        },
        "using" => {
            ret = format!("using {};\n", &s[pivot].code[6..]);
        },
        "lib" | "library" => {
            if regi(&split(&s[pivot].code)[1], "^(std|standard)$") {
                ret.push_str("#include \"engppstd.hpp\"\n");
                let file = File::create("engppstd.hpp");
                file.unwrap().write_all(&stdlib::STDLIB[..])
                    .expect("Failed to write standard library file.");
            }
            else {
                ret = format!("#include <{}>\n", &s[pivot].code[7..]);
            }
        },
        _=>{}
    };
    Ok(ret)
}