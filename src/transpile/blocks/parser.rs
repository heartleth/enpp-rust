pub mod phrase;
pub mod types;
pub mod value;
pub mod util;
pub mod clause;

pub use phrase::*;
pub use value::*;
pub use types::*;
pub use util::*;

pub fn verb_parse(s :&String)->String {
    #[derive(PartialEq)]
    enum Mode {
        Of, In, By, _None
    }

    let mut parsing_mode:Mode = Mode::_None;
    let splited = split_token(s, "-");
    let name = &trim_tail(&splited[0]);
    let mut collected :Vec<String> = Vec::new();
    let mut ret = String::from(name);
    
    for elem in &splited[1..] {
        if parsing_mode == Mode::_None {
            match &elem.to_lowercase()[..] {
                "by" => { parsing_mode = Mode::By; },
                "in" => { parsing_mode = Mode::In; },
                "of" => { parsing_mode = Mode::Of; },
                _ => { parsing_mode = Mode::_None; }
            }
        }
        else if parsing_mode == Mode::Of {
            if elem == "type" {
                ret += &format!("<{}>", &types::type_parse(&collected))[..];
                parsing_mode = Mode::_None
            }
            else {
                collected.push(String::from(elem));
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