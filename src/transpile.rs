pub mod blocks;

pub use blocks::parser::*;
pub use blocks::tree;
pub use blocks::Mem;

pub fn transpile(tree :&Mem, pivot :usize)->String {
    let mut ret = String::new();
    let mut iter = 0;
    let len = tree[pivot].children.len();
    loop {
        let parent = tree[pivot].children[iter];
        let elem = &tree[parent];
        let code = &elem.code;
        let code_splited = split(&code);
        let keyword = keyword(&code);
        
        ret.push_str(&loop {
            if regi(&keyword, "(unless|if|else|while|for|repeat)") {
                break blocks::parse_if(&tree, &mut iter, pivot);
            }
            else if regi(&keyword, "(make|ha(ve|s)|let)") {
                
            }
            else if regi(&keyword, "when") {
            
            }
            else if regi(&keyword, "(include|lib(rary)?|using|import)") {
                break blocks::parse_import(&tree, iter);
            }
            else if regi(&keyword, "return") {
            
            }
            else if regi(&keyword, "name(space)?") {
            
            }
            else if regi(&keyword, "break|continue") {
            
            }
            else if regi(&keyword, "public|private|protected") {
            
            }
            else if regi(&keyword, "set") {
            
            }
            else if regi(&keyword, "class") {
            
            }
            else if regi(&keyword, "use") {
            
            }
            else if regi(&keyword, "about") {
            
            }
            else if first_phrase(&code_splited) == split(&code).len() - 1 {
                
            }
            else {

            }
        });
        iter += 1;
        if iter == len {
            break;
        }
    }
    String::new()
}