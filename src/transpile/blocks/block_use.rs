use super::*;

#[inline]
pub fn parse_use(s :&Mem, pivot :usize)->Result<String, &'static str> {
    Ok(format!("{}([&](){{\n{}\n}});\n\n",
        &verb_parse(&split(&s[pivot].code)[1]),
        transpile(s, pivot)
    ))
}