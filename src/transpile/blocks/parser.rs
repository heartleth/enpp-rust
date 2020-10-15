mod util;
pub use util::*;

pub fn value_parse(s :&Vec<String>)->String {
    String::new()
}

pub fn first_phrase(s :&Vec<String>)->usize {
    split(s).len()-1
}

pub fn first_clause(s :&Vec<String>)->usize {
    split(s).len()-1
}

pub fn verb_parse(s :&String)->usize {
    let splited = s.split('-');
    for elem in splited {
        println!("{}", elem);
    }
    split(s).len()-1
}