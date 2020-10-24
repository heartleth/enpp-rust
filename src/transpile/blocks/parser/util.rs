use regex;

pub fn split(s:&String)->Vec<String> {
    let re = regex::Regex::new(r"[\s\t\r\n]+").unwrap();
    let mut ret :Vec<String> = Vec::new();
    for part in re.split(&s) {
        ret.push(String::from(part));
    }
    ret
}

pub fn split_token(s:&String, delim:&str)->Vec<String> {
    let re = regex::Regex::new(delim).unwrap();
    let mut ret :Vec<String> = Vec::new();
    for part in re.split(&s) {
        ret.push(String::from(part));
    }
    ret
}

pub fn regi(s:&String, reg:&str)->bool {
    regex::Regex::new(reg).unwrap().is_match(&s.to_lowercase())
}

pub fn keyword(s:&String)->String {
    let k = split(s);
    String::from(&k[0])
}

pub fn is_bracket(s :&String)->bool {
    println!("{}", s);
    let mut in_string = false;
    let mut escaped = false;
    let mut stack :Vec<()> = Vec::new();
    
    for elem in s.chars() {
        match elem {
            '\\' => { escaped = in_string && !escaped },
            '"' => { if !escaped { in_string = !in_string; } escaped=false; },
            '(' => if !in_string { stack.push(()) },
            ')' => if !in_string {
                if stack.is_empty() { panic!("괄호쌍 안맞는다 이기야..."); }
                else { stack.pop(); }
            },
            _ => ()
        }
    }
    let len = s.as_bytes().len() - 1;
    (stack.is_empty())
     && (s.as_bytes()[0] == '(' as u8)
     && (s.as_bytes()[len] == ')' as u8)
}