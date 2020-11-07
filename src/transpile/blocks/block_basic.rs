use super::*;

#[inline]
pub fn parse_about(s :&String)->Result<String, &str> {
    Ok(format!("template<{}>\n", &args_to_string(&arguments_parse(&split(&s)[1..].to_vec())?)))
}

#[inline]
pub fn parse_return(s :&String)->Result<String, &str> {
    Ok(format!("return ({});\n", &value_parse(&String::from(&s[7..]), 1)?))
}

#[inline]
pub fn parse_namespace(s :&Mem, pivot :usize)->Result<String, &str> {
    Ok(format!("namespace {} {{ {} }}\n", 
        &verb_parse(&String::from(s[pivot].code[10..].trim())),
        transpile(s, pivot)
    ))
}

#[inline]
pub fn parse_access(s :&Mem, pivot :usize)->Result<String, &str> {
    Ok(format!("{}:\n{}\n",
        &s[pivot].code,
        transpile(s, pivot)
    ))
}

#[inline]
pub fn parse_set(s :&String)->Result<String, &str> {
    Ok(format!("{};", value_parse(&String::from(&s[4..]), 3)?))
}