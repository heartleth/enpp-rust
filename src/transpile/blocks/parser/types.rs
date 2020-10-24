#[derive(Debug)]
pub struct Variable {
    pub typename :String,
    pub name :String
}

impl Variable {
    pub fn to_string(&self)->String {
        format!("{} {}", self.typename, self.name)
    }
}

pub fn type_parse(s :&Vec<&String>)->String {
    let mut ret = String::new();
    for elem in s {
        ret.push_str(elem);
    }
    ret
}

pub fn declarition_parse(s :&Vec<String>)->Variable {
    let last = s.len()-1;
    if s.len() == 0 { panic!("니 변수이름 어디로 운지했노?"); }
    let mut ret = Variable{
        typename: String::new(),
        name: String::from(&s[last])
    };
    
    for elem in 0 .. s.len()-1 {
        ret.typename += &format!("{} ", match &s[elem].to_ascii_lowercase()[..] {
            "integer" => "int",
            "constant" => "const",
            _ => &s[elem][..]
        })[..];
    }
    if last == 0 { ret.typename=String::from("auto "); }
    ret
}