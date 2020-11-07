use super::util::*;
pub use clause::*;
use super::*;

pub fn first_phrase(s :&Vec<String>, is_first :bool, allow_multi :bool)->Result<usize, &'static str> {
    let operators = if allow_multi {
        r"^(,|and|or|plus|minus|=|is(not)?|as|[+\-*/%]|<<|>>|[|&]|[><]|[a-zA-Z_][a-zA-Z0-9\-_]*[=!]|having|been|do|in)$"
    } else {
        r"^(and|or|plus|minus|=|is(not)?|as|[+\-*/%]|<<|>>|[|&]|[><]|[a-zA-Z_][a-zA-Z0-9\-_]*[=!]|having|been|do|in)$"
    };
    
    let mut ret = 0;
    let len = s.len();
    if len == 1 {
        return Ok(0);
    }
    else {
        let first_low = s[0].to_ascii_lowercase();
        if is_bracket(&s.join(" "), ('(', ')'))? || is_bracket(&s.join(" "), ('{', '}'))? {
            if first_phrase(&s[1..len - 1].to_vec(), true, true)? == len - 3 {
                return Ok(len - 1);
            }
            else {
                return Err("wrong phrase");
            }
        }
        else if is_string(&s.join(" ")) {
            return Ok(len - 1);
        }
        else if first_low == "|" {
            for elem in 1 .. s.len() - 1 {
                if s[elem] == "|" {
                    ret = elem + 1;
                    return Ok(ret + first_phrase(&s[ret..].to_vec(), true, allow_multi)?);
                }
            }
        }
        else if first_low == "^if$" {
            let mut expression = first_phrase(&s[1..].to_vec(), true, allow_multi)?;
            expression += 1;
            expression += first_phrase(&s[expression+2..].to_vec(), true, allow_multi)?;
            expression += 2;
            ret = expression;
            ret += first_phrase(&s[expression+2..].to_vec(), true, allow_multi)?;
            ret += 2;
        }
        else if first_low == "^(result)$" {
            ret = 2;
            ret += first_clause(&s[2..].to_vec())?;
        }
        else if regi(&first_low, r"^(\$|[tw]hat)$") {
            ret = 1;
            ret += first_clause(&s[1..].to_vec())?;
        }
        else if first_low == "^(make)$" {
            let to_give = [vec![String::from("a ")], s[1..].to_vec()].concat();
            ret = first_clause(&to_give)?;
        }
        else if regi(&first_low, "^(value|addr(ess)?|ptr|pointer)$") {
            ret = 2;
            ret += first_phrase(&s[2..].to_vec(), false, false)?;
        }
        else if regi(&first_low, r"^[a-zA-Z_]\w*:$") {
            ret = 1;
            ret += first_phrase(&s[1..].to_vec(), false, true)?;
        }
        else if regi(&s[is_first as usize], operators) {
            ret = is_first as usize + 1;
            ret += first_phrase(&s[ret..].to_vec(), false, allow_multi)?;
        }
        else {
            let ignores = existing_keys(&s)?;
            let mut breaker = 0;
            let mut is_breaked = false;
            for elem in &ignores {
                if regi(&elem, operators) {
                    is_breaked = true;
                    break;
                }
                else { breaker += 1; }
            }

            if !is_breaked || breaker == s.len()-1 {
                return Ok(0);
            }
            let lport = first_phrase(&s[..breaker].to_vec(), true, allow_multi)?;
            if lport != breaker - 1 {
                return Ok(lport);
            }
            ret = breaker;
            ret += first_phrase(&s[breaker+1..].to_vec(), true, allow_multi)?;
            ret += 1;
        }
    }
    Ok(ret)
}