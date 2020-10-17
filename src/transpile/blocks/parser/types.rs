// use super::*;

pub fn type_parse(s :&Vec<&String>)->String {
    let mut ret = String::new();
    for elem in s {
        ret.push_str(elem);
    }
    ret
}