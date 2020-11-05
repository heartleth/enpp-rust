pub mod blocks;

pub use blocks::*;

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
            if regi(&keyword, "^(unless|if|else|while|for|repeat)$") {
                blocks::parse_if(&tree, &mut iter, pivot)
            }
            else if regi(&keyword, "^(make|ha(ve|s)|let)$") {
                String::new()
            }
            else if regi(&keyword, "^when$") {
                String::new()
            }
            else if regi(&keyword, "^(include|lib(rary)?|using|import)$") {
                blocks::parse_import(&tree, parent)
            }
            else if regi(&keyword, "^return$") {
                String::new()
            }
            else if regi(&keyword, "^name(space)?$") {
                String::new()
            }
            else if regi(&keyword, "^break|continue$") {
                String::new()
            }
            else if regi(&keyword, "^public|private|protected$") {
                String::new()
            }
            else if regi(&keyword, "^set$") {
                String::new()
            }
            else if regi(&keyword, "^class$") {
                String::new()
            }
            else if regi(&keyword, "^use$") {
                String::new()
            }
            else if regi(&keyword, "^about$") {
                String::new()
            }
            else if first_phrase(&code_splited, true) == split(&code).len() - 1 {
                value_parse(&code, 1)
            }
            else {
                String::new()
            }
        });
        iter += 1;
    }
    ret
}