mod operator;
mod sentence;

pub use operator::*;
pub use sentence::*;
use super::phrase::*;
use super::types::*;
use super::util::*;
use super::*;

pub fn value_parse(s :&String, level :usize)->Result<String, &'static str> {
    let s = &String::from(s.trim());
    let mut ret = String::new();
    let mut do_pass = true;
    let list = &split(&s);
    let units = &existing_keys(&split(&s))?;

    if units.len() == 1 {
        if is_string(&s) {
            return Ok(format!("{}", s));
        }
        if units[0].as_bytes()[0] == '[' as u8 {
            let valid = String::from(&s[1..units[0].len() - 1]);
            let range :Vec<String> = split_token(&valid, "\\.");
            let step = range.len() - 1;
            let begin :&String = &range[0];
            let end :&String = &range[step];
            if begin.chars().next().unwrap() == '\'' {
                let begin = begin.chars().nth(1).unwrap() as u32;
                let end = end.chars().nth(1).unwrap() as u32;
                ret += "std::vector<int>({";
                for elem in (begin..=end).step_by(step-1) {
                    ret += elem.to_string().as_str();
                    ret += ", ";
                }
                ret += "})";
            }
            else {
                let begin = begin.parse::<i32>().unwrap();
                let end = end.parse::<i32>().unwrap();
                ret += "std::vector<int>({";
                for elem in (begin..=end).step_by(step-1) {
                    ret += elem.to_string().as_str();
                    ret += ", ";
                }
                ret += "})";
            }
            return Ok(ret);
        }
        return Ok(String::from(s));
    }
    
    if is_bracket(&s, ('(', ')'))? {
        return Ok(format!("({})", &value_parse(&String::from(&s[1..s.len()-1]), 0)?));
    }
    else if is_bracket(&s, ('{', '}'))? {
        return Ok(format!("{{{}}}", &value_parse(&String::from(&s[1..s.len()-1]), 0)?));
    }

    if level == 0 {
        left_operator(&mut do_pass, (units, list, "^(,)$"), &mut |cnt :usize| {
            ret = format!("{}, {}", &value_parse(&list[..cnt].to_vec().join(" "), 0)?, &value_parse(&list[cnt+1..].to_vec().join(" "), 0)?);
            Ok(())
        })?;
    }
    else if level == 1 {
        if units[0] == "|" {
            do_pass = false;
            let mut cnt = 0;
            for elem in &units[1..].to_vec() {
                cnt += 1;
                if elem == "|" {
                    break;
                }
            }
            let mut vars = String::new();
            vars += &args_to_string(&arguments_parse(&list[1..cnt].to_vec())?);
            
            ret = format!("([&]({}){{return({});}})", vars, &value_parse(&list[cnt+1..].to_vec().join(" "), level)?);
        }
    }
    else if level == 2 {
        if regi(&units[0], r"^(make)$") {
            do_pass = false;
            ret = parse_sentence(&format!("it {}", &s[5..]))?;
        }
        else if regi(&units[0], r"^(result)$") {
            do_pass = false;
            ret = parse_sentence(&String::from(&s[10..]))?;
        }
        else if regi(&units[0], r"^([wt]hat|\$)$") {
            do_pass = false;
            ret = parse_sentence(&String::from(&s[5..]))?;
        }
        else if regi(&units[0], r"^(to)$") {
            do_pass = false;
            ret = format!("[&](){{return({});}}", parse_sentence(&format!("it {}", &s[3..]))?);
        }
    }
    else if level == 3 {
        if regi(&units[0].to_ascii_lowercase(), "^if$") {
            do_pass = false;
            let first = 1 + first_phrase(&list[1..].to_vec(), true, false)?;
            let second = first + 2 + first_phrase(&list[first+2..].to_vec(), true, false)?;
            ret = format!("(({}) ? ({}) : ({}))",
                &value_parse(&list[1..first+1].to_vec().join(" "), 1)?,
                &value_parse(&list[first+2..second+1].to_vec().join(" "), 1)?,
                &value_parse(&list[second+2..].to_vec().join(" "), 1)?
            );
        }
    }
    else if level == 4 {
        let mut cnt = units.len();
        for _ in 0..units.len() {
            cnt -= 1;
            let elem = &units[cnt];

            if regi(&elem, r"^(=|as|[a-zA-Z_]\w*=)$") {
                let lport = first_phrase(&list[..cnt].to_vec(), true, false)? + 1;
                if lport != cnt {
                    return Err("SyntaxError: phrase left of the operator 'as' is too short.");
                }

                do_pass = false;
                if regi(&elem, r"^(=|as)$") {
                    ret = format!("({} = {})", &value_parse(&list[..cnt].to_vec().join(" "), 1)?, &value_parse(&list[cnt+1..].to_vec().join(" "), 1)?);
                }
                else {
                    ret = format!("({to_assign} = {functor}({to_assign}, {argument}))", 
                        to_assign = &value_parse(&list[..cnt].to_vec().join(" "), 1)?,
                        argument = &value_parse(&list[cnt+1..].to_vec().join(" "), 1)?,
                        functor = &elem[..elem.len()-1]
                    );
                }
            }
        }
    }
    else if level == 5 {
        left_operator(&mut do_pass, (units, list, "^(or)?or$"), &mut |cnt :usize| {
            if regi(&list[cnt], "^oror$") {
                ret = format!("({} || {})", &value_parse(&list[..cnt].to_vec().join(" "), 1)?, &value_parse(&list[cnt+1..].to_vec().join(" "), 1)?);
            }
            else {
                ret = format!("({} | {})", &value_parse(&list[..cnt].to_vec().join(" "), 1)?, &value_parse(&list[cnt+1..].to_vec().join(" "), 1)?);
            }
            Ok(())
        })?;
    }
    else if level == 6 {
        left_operator(&mut do_pass, (units, list, "^(and)?and$"), &mut |cnt :usize| {
            if regi(&list[cnt], "^andand$") {
                ret = format!("({} && {})", &value_parse(&list[..cnt].to_vec().join(" "), 1)?, &value_parse(&list[cnt+1..].to_vec().join(" "), 1)?);
            }
            else {
                ret = format!("({} & {})", &value_parse(&list[..cnt].to_vec().join(" "), 1)?, &value_parse(&list[cnt+1..].to_vec().join(" "), 1)?);
            }
            Ok(())
        })?;
    }
    else if level == 7 {
        left_operator(&mut do_pass, (units, list, "^(is(not)?|[<>]=?)$"), &mut |cnt :usize| {
            if regi(&list[cnt], r"is") {
                ret = format!("({} == {})", &value_parse(&list[..cnt].to_vec().join(" "), 1)?, &value_parse(&list[cnt+1..].to_vec().join(" "), 1)?);
            }
            else if regi(&list[cnt], r"not") {
                ret = format!("({} != {})", &value_parse(&list[..cnt].to_vec().join(" "), 1)?, &value_parse(&list[cnt+1..].to_vec().join(" "), 1)?);
            }
            else {
                ret = format!("({} {operator} {})", &value_parse(&list[..cnt].to_vec().join(" "), 1)?, &value_parse(&list[cnt+1..].to_vec().join(" "), 1)?, operator = &units[cnt]);
            }
            Ok(())
        })?;
    }
    else if level == 8 {
        left_operator(&mut do_pass, (units, list, "^([+-]|plus|minus)$"), &mut |cnt :usize| {
            if regi(&units[cnt], "^plus$") {
                ret = format!("({} + {})", &value_parse(&list[..cnt].to_vec().join(" "), 1)?, &value_parse(&list[cnt+1..].to_vec().join(" "), 1)?);
            }
            else if regi(&units[cnt], "^minus$") {
                ret = format!("({} + {})", &value_parse(&list[..cnt].to_vec().join(" "), 1)?, &value_parse(&list[cnt+1..].to_vec().join(" "), 1)?);
            }
            else {
                ret = format!("({} {operator} {})", &value_parse(&list[..cnt].to_vec().join(" "), 1)?, &value_parse(&list[cnt+1..].to_vec().join(" "), 1)?, operator = &units[cnt]);
            }
            Ok(())
        })?;
    }
    else if level == 9 {
        left_operator(&mut do_pass, (units, list, "^([/*%])$"), &mut |cnt :usize| {
            ret = format!("({} {operator} {})", &value_parse(&list[..cnt].to_vec().join(" "), level)?, &value_parse(&list[cnt+1..].to_vec().join(" "), level)?, operator = &units[cnt]);
            Ok(())
        })?;
    }
    else if level == 10 {
        if regi(&units[0], r"^[a-zA-Z_]\w*:$") {
            do_pass = false;
            let func_name = &verb_parse(&String::from(&units[0][..&units[0].len()-1]));
            let to_be_evaluated = &value_parse(&list[1..].to_vec().join(" "), 0)?;
            if is_bracket(&to_be_evaluated, ('(', ')'))? {
                ret = format!("{}{}", func_name, to_be_evaluated);
            }
            else {
                ret = format!("{}({})", func_name, to_be_evaluated);

            }
        }
        else {
            left_operator(&mut do_pass, (units, list, r"^[a-zA-Z_]\w*!$"), &mut |cnt :usize| {
                ret = format!("({}({}, {}))", 
                    &verb_parse(&String::from(&units[cnt][..&units[cnt].len()-1])),
                    &value_parse(&list[..cnt].to_vec().join(" "), 1)?,
                    &value_parse(&list[cnt+1..].to_vec().join(" "), 1)?
                );
                Ok(())
            })?;
        }
    }
    else if level == 11 {
        if regi(&units[units.len()-2], r"^(been)$") {
            do_pass = false;
            ret = format!("{}({})", &verb_parse(&units[units.len()-1]), &value_parse(&list[0..units.len()-2].to_vec().join(" "), 1)?);
        }
        else if regi(&units[units.len()-2], r"^do$") {
            do_pass = false;
            ret = format!("{1}.{0}()", &verb_parse(&units[units.len()-1]), &value_parse(&list[0..units.len()-2].to_vec().join(" "), 1)?);
        }
    }
    else if level == 12 {
        if regi(&units[units.len()-2], r"^(in)$") {
            do_pass = false;
            ret = format!("{}::{}", &units[units.len()-1], &value_parse(&list[0..units.len()-2].to_vec().join(" "), 1)?);
        }
    }
    else if level == 13 {
        if regi(&units[units.len()-2], r"^(having)$") {
            do_pass = false;
            ret = format!("{}.{}", &value_parse(&list[0..units.len()-2].to_vec().join(" "), 1)?, &units[units.len()-1]);
        }
    }
    else if level == 14 {
        if regi(&units[0], r"^(value|addr(ess)?|pointer|ptr)$") {
            do_pass = false;
            if regi(&units[0], r"^(value)$") {
                ret = format!("(*{})", &value_parse(&list[2..units.len()-2].to_vec().join(" "), 1)?);
            }
            else {
                ret = format!("(&{})", &value_parse(&list[2..units.len()-2].to_vec().join(" "), 1)?);
            }
        }
    }
    else if level >= 15 {
        if is_string(&s) {
            return Ok(format!("{}", s));
        }
        else {
            return Err("SyntaxError: invalid phrase");
        }
    }

    if do_pass { return value_parse(&s, level + 1); }

    Ok(ret)
}