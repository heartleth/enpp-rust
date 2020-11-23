use std::process::Command;
use super::transpile::*;
use crate::text_io::*;
use std::io::Write;
use std::fs::File;
use std::str;

pub fn repl() {
    let mut leave = String::from("#include \"engppstd.hpp\"\n");
    let mut lines :Vec<String> = Vec::new();
    let mut stack :Vec<char> = Vec::new();
    let mut leave_main = String::new();
    let mut header = String::new();
    let mut in_string = false;
    let mut escaped = false;
    let mut indents = 0;

    loop {
        if stack.is_empty() && lines.is_empty() {
            print!("\n >> ");
            std::io::stdout().flush().expect("Error to flush stdout");
        }
        else {
            print!("{} .. ", "    ".repeat(indents));
            std::io::stdout().flush().expect("Error to flush stdout");
        }
        let mut buff :String = read!("{}\n");

        for elem in buff.chars() {
            match elem {
                '\\' => { escaped = in_string && !escaped },
                '"' => { if !escaped { in_string = !in_string; } escaped=false; },
                '(' => if !in_string { stack.push('(') },
                ')' => if !in_string {
                    if stack.is_empty() { eprintln!("괄호쌍 안맞는다 이기야..."); continue; }
                    else if *stack.last().unwrap() == '(' { stack.pop(); }
                    else { eprintln!("괄호쌍 안맞는다 이기야..."); continue; }
                },
                '{' => if !in_string { stack.push('{') },
                '}' => if !in_string {
                    if stack.is_empty() { eprintln!("괄호쌍 안맞는다 이기야..."); continue; }
                    else if *stack.last().unwrap() == '{' { stack.pop(); }
                    else { eprintln!("괄호쌍 안맞는다 이기야..."); continue; }
                },
                _ => { escaped=false; }
            };
        }
        let mut will_evaluated = false;

        if !stack.is_empty() {
            lines.push(String::from(&buff));
        }
        else if buff.trim().is_empty() {
            if indents > 0 {
                indents -= 1;
            }
            else {
                will_evaluated = true;
            }
        }
        else if regi(&buff, r"^([\s\t]*([Ii]f|else|[Ww]hile|[Ff]or|[Rr]epeat|[Uu]nless|[Ww]hen|[Uu]se|public|protected|[Cc]lass|private).*)$") {
            if header.is_empty() {
                header = keyword(&buff);
            }
            buff = format!("{}{}", "    ".repeat(indents), buff);
            lines.push(String::from(&buff));
            indents += 1;
        }
        else if indents > 0 {
            if header.is_empty() {
                header = keyword(&buff);
            }
            buff = format!("{}{}", "    ".repeat(indents), buff);
            lines.push(String::from(&buff));
        }
        else {
            lines.push(String::from(&buff));
            if header.is_empty() {
                header = keyword(&buff);
            }
            will_evaluated = true;
        }

        if will_evaluated {
            if regi(&header, "^(class|when|make|have)$") {
                leave += &transpile(&tree::CodeTree::treeify(&lines.join("\n")), 0);
            }
            else if regi(&header, "^(let|set)$") {
                leave_main += &transpile(&tree::CodeTree::treeify(&lines.join("\n")), 0);
            }
            else {
                let file = File::create("engppstd.hpp");
                file.unwrap().write_all(&blocks::stdlib::STDLIB[..])
                    .expect("Failed to write standard library file.");

                let mut file = File::create("enpprtpl.cpp").unwrap();
                file.write_all(format!("{}int main(){{{}{}}}",
                    leave,
                    leave_main,
                    &transpile(&tree::CodeTree::treeify(&lines.join("\n")), 0)
                ).as_bytes()).expect("Failed to make output");
                drop(file);
                
                if cfg!(target_os = "windows") {
                    Command::new("cmd").args(&["/C", "g++ -o out enpprtpl.cpp -std=c++17"]).spawn()
                        .expect("failed to execute process").wait()
                        .expect("failed to wait");
                    Command::new("cmd").args(&["/C", ".\\out"]).spawn()
                        .expect("failed to execute process").wait()
                        .expect("failed to wait");
                }
                else {
                        Command::new("sh").args(&["-c", "g++ -o out enpprtpl.cpp -std=c++17"]).spawn()
                            .expect("failed to execute process").wait()
                            .expect("failed to wait");
                        Command::new("sh").args(&["-c", "./out"]).spawn()
                            .expect("failed to execute process").wait()
                            .expect("failed to wait");
                }
            }
            indents = 0;
            stack.clear();
            lines.clear();
            header.clear();
        }
    };
}