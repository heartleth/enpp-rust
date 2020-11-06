use super::*;

#[inline]
pub fn parse_use(s :&Mem, pivot :usize)->String {
    format!("{}([&](){{\n{}\n}});\n\n",
        &verb_parse(&split(&s[pivot].code)[1]),
        transpile(s, pivot)
    )
}