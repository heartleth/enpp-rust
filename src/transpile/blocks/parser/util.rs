use regex;

pub fn split(s:&String)->Vec<String> {
    let re = regex::Regex::new(r"[\s\t\r\n]+").unwrap();
    let mut ret = Vec::new();
    for part in re.split(&s) {
        ret.push(String::from(part));
    }
    ret
}

pub fn regi(s:&String, reg:&str)->bool {
    regex::Regex::new(reg).unwrap().is_match(&s.to_lowercase())
}

pub fn keyword(s:&String)->String {
    let k = split(s);
    String::from(&k[0])
}

pub fn to_str(s:&String)->&str {
    &s[..]
}