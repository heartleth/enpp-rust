pub mod blocks;
pub use blocks::{
    parse_sentence,
    Mem,
    keyword,
    split,
    regi,
    value_parse,
    first_phrase,
    first_clause,
    tree
};

use std::panic;

pub fn transpile(tree :&Mem, pivot :usize)->String {
    let mut ret = String::new();
    let mut iter = 0;
    let len = tree[pivot].children.len();
    loop {
        if iter == len {
            break;
        }
        let parent = tree[pivot].children[iter];
        let elem = &tree[parent];
        let code = &elem.code;
        let code_splited = split(&code);
        let keyword = keyword(&code);
        
        ret = format!("{}{}", ret, &{
            if regi(&keyword, "^(unless|if|else)$") {
                blocks::parse_if(&tree, &mut iter, pivot)
            }
            else if regi(&keyword, "^(repeat)$") {
                blocks::parse_repeat(&tree, parent)
            }
            else if regi(&keyword, "^(while)$") {
                blocks::parse_while(&tree, parent)
            }
            else if regi(&keyword, "^(for)$") {
                blocks::parse_for(&tree, parent)
            }
            else if regi(&keyword, "^(make|ha(ve|s)|let)$") {
                blocks::parse_new(code)
            }
            else if regi(&keyword, "^when$") {
                blocks::parse_when(&tree, parent)
            }
            else if regi(&keyword, "^(include|lib(rary)?|using|import)$") {
                blocks::parse_import(&tree, parent)
            }
            else if regi(&keyword, "^return$") {
                blocks::parse_return(&code)
            }
            else if regi(&code, r"^name\s?space\s.+$") {
                blocks::parse_namespace(&tree, parent)
            }
            else if regi(&keyword, "^break|continue$") {
                format!("{};", code)
            }
            else if regi(&keyword, "^public|private|protected$") {
                blocks::parse_access(&tree, parent)
            }
            else if regi(&keyword, "^set$") {
                blocks::parse_set(&code)
            }
            else if regi(&keyword, "^class$") {
                blocks::parse_class(&tree, parent)
            }
            else if regi(&keyword, "^use$") {
                blocks::parse_use(&tree, parent)
            }
            else if regi(&keyword, "^about$") {
                blocks::parse_about(&code)
            }
            else if first_phrase(&code_splited, true, false) == code_splited.len() - 1 {
                value_parse(&code, 1) + ";"
            }
            else if first_clause(&code_splited) == code_splited.len() - 1 {
                parse_sentence(&code) + ";"
            }
            else {
                panic!("Invalid sentence")
            }
        });
        iter += 1;
    }
    ret
}