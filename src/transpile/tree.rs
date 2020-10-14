#[derive(Debug)]
pub struct CodeTree {
    pub code:String,
    pub line:usize,
    pub children:Vec<usize>
}

impl CodeTree {
    pub fn new(s:&str, l:usize)->CodeTree {
        CodeTree {
            code: String::from(s),
            line: l,
            children: Vec::new()
        }
    }
}

pub mod filter {
    fn is_empty(s:&str)->bool {
        String::from(s).trim().len() == 0
    }
    
    static mut INDEDT_TYPE:char = '\t';
    static mut INDENT_SIZE:usize = 0;
    static mut IS_FIRST:bool = true;

    pub fn get_indents(s:&str)->i32 {
        if is_empty(s) {
            return -1;
        }
        
        if &s[0..1] != " " && &s[0..1] != "\t" {
            return 0;
        }
        let mut ret:i32 = 0;
        unsafe {
            if IS_FIRST && (&s[0..1] == " " || &s[0..1] == "\t") {
                for elem in s.as_bytes() {
                    let c = *elem as char;
                    if c != ' ' && c != '\t' {
                        break;
                    }
                    INDENT_SIZE += 1;
                }
                INDEDT_TYPE = s.as_bytes()[0] as char;
                IS_FIRST = false;
                
                return 1;
            }
            else {
                for elem in s.as_bytes() {
                    let c = *elem as char;
                    if c != INDEDT_TYPE {
                        break;
                    }
                    ret += 1;
                }
                ret /= INDENT_SIZE as i32;
            }
        }
        return ret;
    }
}

impl CodeTree {
    pub fn treeify(s :&String)->Vec<CodeTree> {
        let mut mem :Vec<CodeTree> = vec!(CodeTree::new("root", 0));
        let mut stack :Vec<usize> = vec!(0);

        let mut before_indents:usize = 0;
        let mut line:usize = 0;

        for elem in s.split('\n') {
            line += 1;
            let indents = filter::get_indents(&elem);
            if indents >= 0 {
                let indents = indents as usize;
                let top = stack.len() - 1;
                
                let diff = indents as i32 - before_indents as i32;
                if diff > 0 {
                    let last = mem[stack[top]].children.len() - 1;
                    stack.push(mem[stack[top]].children[last]);
                }
                else if diff < 0 {
                    for _ in indents .. before_indents {
                        stack.pop();
                    }
                }
               
                let top = stack.len() - 1;
                mem.push(CodeTree::new(elem.trim(), line));
                let back_memory = mem.len() - 1;

                mem[stack[top]].children.push(back_memory);
                before_indents = indents;
            }
        }
        mem
    }
}

pub fn info(data: &Vec<CodeTree>, pivot:usize, indents:usize) {
    println!("{}{}", "  ".repeat(indents), data[pivot].code);
    for elem in &data[pivot].children {
        info(data, *elem, indents + 1);
    }
}