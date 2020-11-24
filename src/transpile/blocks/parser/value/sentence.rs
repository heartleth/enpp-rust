use super::*;

pub fn parse_sentence(s :&String)->Result<String, &'static str> {
    let ret;
    let spl = split(&s);
    
    if s.len() <= 1 { return Err("wrong clause"); }
    let subject_idx = first_phrase(&spl, true, false)?;
    let mut subject = value_parse(&spl[..=subject_idx].to_vec().join(" "), 1)?;

    if subject.to_ascii_lowercase() == "it" {
        subject = String::new();
    }
    else { subject+="."; }
    
    if spl.len() > subject_idx + 2 && regi(&spl[subject_idx + 2], "^(->|with|about|for|:)$") {
        ret = format!("{}{}({})",
            &subject,
            &spl[subject_idx + 1],
            &value_parse(&spl[subject_idx+3..].to_vec().join(" "), 0)?
        );
    }
    else {
        ret = format!("{}{}()", &subject, &spl[subject_idx + 1]);    
    }
    Ok(ret)
}