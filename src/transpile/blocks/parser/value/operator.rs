use super::*;

pub static OPERATORS :&str = r"^(,|and|or|plus|minus|=|is(not)?|as|[+\-*/%]|<<|>>|[|&]|[><]|[a-zA-Z_][a-zA-Z0-9\-_]*[=!])$";

pub fn left_operator<T>(
    do_pass :&mut bool,
    (units, list, reg) :(&Vec<String>, &Vec<String>, &str),
    functor :&mut T) where T : FnMut(usize)->() {

    let mut cnt = 0;
    for _ in 0..units.len() {
        let elem = &units[cnt];
        
        if regi(&elem, reg) {
            *do_pass = false;
            
            let lport = first_phrase(&list[..cnt].to_vec(), true) + 1;
            if lport != cnt {
                panic!("SyntaxError: phrase left of the operator is too short.");
            }
            functor(cnt);
        }
        cnt += 1;
    }
}