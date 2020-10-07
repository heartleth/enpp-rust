#[derive(Debug)]
pub struct CodeTree {
    pub code:String,
    pub line:usize,
    pub children:Vec<*mut CodeTree>
}

impl CodeTree {
    pub fn new(s:&str, l:usize)->CodeTree {
        CodeTree {
            code: String::from(s),
            line: l,
            children: Vec::new()
        }
    }
    
    pub fn insert(&mut self, s:*mut CodeTree) {
        self.children.push(s);
    }
    
}

pub fn info(tree:&CodeTree, indents:u8) {
    print!("{}", "    ".repeat(indents as usize));
    println!("{}", tree.code);
    for elem in tree.children.clone() {
        unsafe {
            info(&*elem, indents + 1);
        }
    }
}

pub mod filter {
    pub fn is_empty(s:&str)->bool {
        String::from(s).trim().len() == 0
    }
    
    static mut indentType:char = '\t';
    static mut indentSize:usize = 0;
    static mut isFirst:bool = true;

    pub fn get_indents(s:&str)->i32 {
        if is_empty(s) {
            return -1;
        }
        
        if &s[0..1] != " " && &s[0..1] != "\t" {
            return 0;
        }
        let mut ret:i32 = 0;
        unsafe {
            if isFirst && (&s[0..1] == " " || &s[0..1] == "\t") {
                for elem in s.as_bytes() {
                    let c = *elem as char;
                    if c != ' ' && c != '\t' {
                        break;
                    }
                    indentSize += 1;
                }
                indentType = s.as_bytes()[0] as char;
                isFirst = false;
                
                return 1;
            }
            else {
                for elem in s.as_bytes() {
                    let c = *elem as char;
                    if c != indentType {
                        break;
                    }
                    ret += 1;
                }
                ret /= indentSize as i32;
            }
        }
        return ret;
    }
}

impl CodeTree {
    pub fn treeify(s:&String)->CodeTree {
        let mut ret = CodeTree::new("root", 0);
        let mut stack:Vec<*mut CodeTree> = vec!(&mut ret);

        let mut before_indents:usize = 0;
        let mut line:usize = 0;

        for elem in s.split('\n') {
            line += 1;
            let indents = filter::get_indents(&elem);
            unsafe {
                if indents >= 0 {
                    let indents = indents as usize;
                    let top = stack.len() - 1;

                    if indents as i32 - before_indents as i32 > 0 {
                        let c = &(*stack[top]).children;
                        stack.push(c[c.len()-1]);
                    }
                    else if (indents as i32 - before_indents as i32) < 0 {
                        for _ in indents .. before_indents {
                            stack.pop();
                        }
                    }
                    println!("{}", stack.len());
                    
                    let top = stack.len() - 1;
                    let c = &mut (*stack[top]).children;
                    c.push(&mut CodeTree::new(elem.trim(), line));

                    before_indents = indents;
                }
            }
        }
        // info(&ret, 0);
        ret
    }
}