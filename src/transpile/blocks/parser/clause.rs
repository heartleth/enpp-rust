use super::*;

pub fn first_clause(s :&Vec<String>)->usize {
    if s.len() <= 1 { panic!("wrong clause"); }
    let subject = first_phrase(&s, true, false);

    if s.len() > subject + 2 && regi(&s[subject + 2], "^(->|with|about|for|:)$") {
        subject + first_phrase(&s[subject+3..].to_vec(), true, true) + 3
    }
    else {
        subject + 1
    }
}