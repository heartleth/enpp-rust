mod operator;

pub use operator::*;
use super::phrase::*;
use super::types::*;
use super::util::*;

pub fn value_parse(s :&String, level :usize)->String {
    let s = &String::from(s.trim());
    let mut ret = String::new();
    let mut do_pass = true;
    let list = &split(&s);
    let units = &existing_keys(&split(&s));

    if units.len() == 1 {
        return String::from(s);
    }
    if is_bracket(&s) {
        return format!("({})", &value_parse(&String::from(&s[1..s.len()-1]), 0));
    }

    if level == 0 {
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
            let args = arguments_parse(&list[1..cnt].to_vec());
            for elem in args {
                vars.push_str(&format!("{} {}, ", elem.typename, elem.name));
            }
            if vars.len() > 0 {
                vars.pop();
                vars.pop();
            }
            ret = format!("([&]({}){{return({});}})", vars, &value_parse(&list[cnt+1..].to_vec().join(" "), level));
        }
    }
    else if level == 1 {
        if units[0].to_ascii_lowercase() == "^if$" {
            do_pass = false;
            let first = 1 + first_phrase(&list[1..].to_vec(), true);
            let second = first + 2 + first_phrase(&list[first+2..].to_vec(), true);
            ret = format!("(({}) ? ({}) : ({}))",
                &value_parse(&list[1..first+1].to_vec().join(" "), level),
                &value_parse(&list[first+2..second+1].to_vec().join(" "), level),
                &value_parse(&list[second+2..].to_vec().join(" "), level)
            );
        }
    }
    else if level == 2 {
        let mut cnt = units.len();
        for _ in 0..units.len() {
            cnt -= 1;
            let elem = &units[cnt];

            if regi(&elem, r"^(=|as|[a-zA-Z_]\w*=)$") {
                let lport = first_phrase(&list[..cnt].to_vec(), true) + 1;
                if lport != cnt {
                    panic!("SyntaxError: phrase left of the operator 'as' is too short.");
                }

                do_pass = false;
                if regi(&elem, r"^(as|=)$") {
                    ret = format!("({} = {})", &value_parse(&list[..cnt].to_vec().join(" "), level - 1), &value_parse(&list[cnt+1..].to_vec().join(" "), level));
                }
                else {
                    ret = format!("({to_assign} = {functor}({to_assign}, {argument}))", 
                        to_assign = &value_parse(&list[..cnt].to_vec().join(" "), level - 1),
                        argument = &value_parse(&list[cnt+1..].to_vec().join(" "), level),
                        functor = &elem[..elem.len()-1]
                    );
                }
            }
        }
    }
    else if level == 3 {
        left_operator(&mut do_pass, (units, list, "^or$"), &mut |cnt :usize| {
            ret = format!("({} || {})", &value_parse(&list[..cnt].to_vec().join(" "), level - 1), &value_parse(&list[cnt+1..].to_vec().join(" "), level));
        });
    }
    else if level == 4 {
        left_operator(&mut do_pass, (units, list, "^and$"), &mut |cnt :usize| {
            ret = format!("({} && {})", &value_parse(&list[..cnt].to_vec().join(" "), level - 1), &value_parse(&list[cnt+1..].to_vec().join(" "), level));
        });
    }
    else if level == 5 {
        left_operator(&mut do_pass, (units, list, "^(is(not)?|[<>]=?)$"), &mut |cnt :usize| {
            if regi(&list[cnt], r"is") {
                ret = format!("({} == {})", &value_parse(&list[..cnt].to_vec().join(" "), level - 1), &value_parse(&list[cnt+1..].to_vec().join(" "), level));
            }
            else if regi(&list[cnt], r"not") {
                ret = format!("({} != {})", &value_parse(&list[..cnt].to_vec().join(" "), level - 1), &value_parse(&list[cnt+1..].to_vec().join(" "), level));
            }
            else {
                ret = format!("({} {operator} {})", &value_parse(&list[..cnt].to_vec().join(" "), level - 1), &value_parse(&list[cnt+1..].to_vec().join(" "), level), operator = &units[cnt]);
            }
        });
    }
    else if level == 6 {
        left_operator(&mut do_pass, (units, list, "^([/*%])$"), &mut |cnt :usize| {
            ret = format!("({} {operator} {})", &value_parse(&list[..cnt].to_vec().join(" "), level), &value_parse(&list[cnt+1..].to_vec().join(" "), level), operator = &units[cnt]);
        });
    }
    else if level == 7 {
        left_operator(&mut do_pass, (units, list, "^([+-]|plus|minus)$"), &mut |cnt :usize| {
            if regi(&units[cnt], "^plus$") {
                ret = format!("({} + {})", &value_parse(&list[..cnt].to_vec().join(" "), level - 1), &value_parse(&list[cnt+1..].to_vec().join(" "), level));
            }
            else if regi(&units[cnt], "^minus$") {
                ret = format!("({} + {})", &value_parse(&list[..cnt].to_vec().join(" "), level - 1), &value_parse(&list[cnt+1..].to_vec().join(" "), level));
            }
            else {
                ret = format!("({} {operator} {})", &value_parse(&list[..cnt].to_vec().join(" "), level - 1), &value_parse(&list[cnt+1..].to_vec().join(" "), level), operator = &units[cnt]);
            }
        });
    }
    else if level == 8 {
        if regi(&units[0], r"^[a-zA-Z_]\w*:$") {
            do_pass = false;
            ret = format!("{}({})", &units[0][..&units[0].len()-1], &value_parse(&list[1..].to_vec().join(" "), level));
        }
    }

    if do_pass { return value_parse(&s, level + 1); }
    
    ret
}