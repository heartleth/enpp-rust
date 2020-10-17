mod types;
mod util;
pub use util::*;

pub fn value_parse(s :&Vec<String>)->String {
    String::new()
}

pub fn first_phrase(s :&Vec<String>)->usize {
    s.len()-1
}

pub fn first_clause(s :&Vec<String>)->usize {
    s.len()-1
}

pub fn verb_parse(s :&String)->String {
    #[derive(PartialEq)]
    enum Mode {
        Of, In, By, _None
    }

    let mut parsing_mode:Mode = Mode::_None;
    let splited = split_token(s, "-");
    let name = &splited[0];
    let mut collected :Vec<&String> = Vec::new();
    let mut ret = String::from(name);

    for elem in &splited[1..] {
        if parsing_mode == Mode::_None {
            match &elem.to_lowercase()[..] {
                "by" => { parsing_mode = Mode::By; },
                "in" => { parsing_mode = Mode::In; },
                "of" => { parsing_mode = Mode::Of; },
                _ => panic!("SyntaxError: VerbError")
            }
        }
        else if parsing_mode == Mode::Of {
            if elem == "type" {
                ret += &format!("<{}>", &types::type_parse(&collected))[..];
                parsing_mode = Mode::_None
            }
            else {
                collected.push(elem);
            }
        }
        else if parsing_mode == Mode::In {
            ret = format!("{}::{}", elem, ret);
            parsing_mode = Mode::_None
        }
        else if parsing_mode == Mode::By {
            ret = format!("{}.{}", elem, ret);
            parsing_mode = Mode::_None
        }
    }
    
    ret
}