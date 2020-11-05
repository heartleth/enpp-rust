use super::*;

pub fn parse_new(s :&String)->String {
    #[derive(Debug)]
    enum InitType {
        Constructor, RefType, CopyType, _None
    }
    #[derive(Debug)]
    enum DeclareType {
        Make,
        Have,
        Let
    }
    use DeclareType::*;
    use InitType::*;

    let keyword = &keyword(&s).to_ascii_lowercase()[..];
    let splited = &split(&s);
    let init_type :InitType;
    let make_type :DeclareType = match &keyword[..] {
        "have" => Have,
        "let" => Let,
        "make" => Make,
        _ => Let
    };

    let mut where_as = 1;
    for elem in &splited[1..] {
        if regi(&elem, "^(as|is|:|->|for|with|about)$") { break; }
        else { where_as += 1; }
    }

    if where_as == splited.len() {
        init_type = InitType::_None;
    }
    else {
        init_type = match &splited[where_as].to_ascii_lowercase()[..] {
            ":" | "->" |
            "for" | "about" |
            "with" => InitType::Constructor,
            "is" => InitType::RefType,
            "as" => InitType::CopyType,
            _ => InitType::_None
        };
    }

    let var = &declarition_parse(&splited[1..where_as].to_vec());

    let ret = match init_type {
        Constructor => match make_type {
            Have => format!("constexpr {}({});\n", var.to_string(), &value_parse(&splited[where_as+1..].to_vec().join(" "), 0)),
            Let => format!("{}({});\n", var.to_string(), &value_parse(&splited[where_as+1..].to_vec().join(" "), 0)),
            Make => format!(
                "{type}* {name} = new {type}({args});\n",
                type = var.typename,
                name = var.name,
                args = &value_parse(&splited[where_as+1..].to_vec().join(" "), 0)
            )
        },
        CopyType => match make_type {
            Have => format!("constexpr {} = {};\n", var.to_string(), &value_parse(&splited[where_as+1..].to_vec().join(" "), 1)),
            Let => format!("{} = {};\n", var.to_string(), &value_parse(&splited[where_as+1..].to_vec().join(" "), 1)),
            Make => format!(
                "{type}* {name} = new {type}({to_copy});\n",
                type = var.typename,
                name = var.name,
                to_copy = &value_parse(&splited[where_as+1..].to_vec().join(" "), 1)
            )
        },
        RefType => match make_type {
            Have => panic!("우흥~ 하고 울어요. 우흥~ 하고 우는데..."),
            Let => format!("{}&{} = {};\n", var.typename, var.name, &value_parse(&splited[where_as+1..].to_vec().join(" "), 1)),
            Make => format!(
                "{type} {name} = {to_copy};\n",
                type = var.typename,
                name = var.name,
                to_copy = &value_parse(&splited[where_as+1..].to_vec().join(" "), 1)
            )
        },
        _None => var.to_string()
    };
    ret
}