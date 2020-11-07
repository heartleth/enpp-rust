use super::*;
pub static mut IS_DYNAMIC :bool = false;

pub fn parse_when(tree :&Mem, pivot :usize)->Result<String, &'static str> {
    let code = &tree[pivot].code;
    let splited = split(&code);

    if splited[1].to_ascii_lowercase() == "it" && splited[2].to_ascii_lowercase() == "starts" {
        Ok(format!("int main(){{{}}}", transpile(&tree, pivot)))
    }
    else {
        match &splited[2].to_ascii_lowercase()[..] {
            "created" => Ok(format!("{}({}){{{}}}",
                &splited[1],
                args_to_string(&arguments_parse(&splited[3..].to_vec())?),
                transpile(&tree, pivot)
            )),
            _ => {
                let func_name = &splited[2];
                let return_type :String;
                let mut func_is_dynamic = false;
                let mut func_is_const = false;
                let mut where_return = 0;
                let mut where_is = 0;

                for elem in &splited {
                    if regi(&elem, "returns?") {
                        break;
                    }
                    else {
                        where_return += 1;
                    }
                }
                for elem in &splited {
                    if regi(&elem, "is") {
                        break;
                    }
                    else {
                        where_is += 1;
                    }
                }
                if where_return == splited.len() {
                    return_type = String::from("void");
                }
                else {
                    return_type = type_parse(&splited[where_return+1..where_is].to_vec());
                }
                
                let where_args_end = if where_return > where_is { where_is } else { where_return };
                let args = &arguments_parse(&splited[3..where_args_end].to_vec())?;

                unsafe { IS_DYNAMIC = func_is_dynamic }

                for elem in &splited[where_is..] {
                    if regi(&elem, "dynamic") {
                        func_is_dynamic = true
                    }
                    if regi(&elem, "const") {
                        func_is_const = true
                    }
                }

                if func_is_dynamic {
                    
                    Ok(format!("{retType} {funcName}({args}) {cst} {{
static std::map<std::tuple<{argTypes}>, {retType}> __MEMOI;
auto __MEMOI_ELEMENT=std::tuple<{argTypes}>({argNames});
if(__MEMOI.count(__MEMOI_ELEMENT)){{
return __MEMOI[__MEMOI_ELEMENT];}}
{scope}
                    }}",
                        argTypes = args_types_to_string(args),
                        argNames = args_names_to_string(args),
                        args = args_to_string(args),
                        retType = return_type,
                        scope = transpile(&tree, pivot),
                        funcName = func_name,
                        cst = (if func_is_const {"const"} else {""})
                    ))
                }
                else {
                    Ok(format!("{} {}({}) {} {{{}}}",
                        return_type,
                        &splited[2],
                        args_to_string(&arguments_parse(&splited[3..where_args_end].to_vec())?),
                        (if func_is_const {"const"} else {""}),
                        transpile(&tree, pivot)
                    ))
                }
            }
        }
    }
}