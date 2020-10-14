pub mod tree;
pub mod util;

pub fn transpile(tree :&Vec<tree::CodeTree>)->String {
    let mut ret = String::new();
    for elem in tree {
        let code = &elem.code;
        let splited_code = util::split(code);
        let keyword = util::keyword(code);
        
        

    }
    String::new()
}