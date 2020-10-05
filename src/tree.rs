#[derive(Debug)]
pub struct CodeTree {
    pub code:String,
    pub line:usize,
    pub children:Vec<CodeTree>
}

impl CodeTree {
    pub fn new(s:String, l:usize)->CodeTree {
        CodeTree {
            code: s,
            line: l,
            children: Vec::new()
        }
    }
    
    pub fn insert(&mut self, s:CodeTree) {
        self.children.push(s);
    }
}

mod filter {
    pub fn is_empty(s:&str)->bool {
        String::from(s).trim().len() == 0
    }
    
    pub fn get_indents(s:&String)->u8 {
        1
    }
}

impl CodeTree {
    /*pub fn treeify(s:&String)->CodeTree {
        
    }*/
}