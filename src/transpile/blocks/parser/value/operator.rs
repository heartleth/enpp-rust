use super::*;

pub fn left_operator<T>(
    do_pass :&mut bool,
    (units, list, reg) :(&Vec<String>, &Vec<String>, &str),
    functor :&mut T)->Result<(), &'static str>
    where T : FnMut(usize)->Result<(), &'static str> {

    let mut cnt = 0;
    for _ in 0..units.len() {
        let elem = &units[cnt];
        
        if regi(&elem, reg) {
            *do_pass = false;
            
            let lport = first_phrase(&list[..cnt].to_vec(), true, false)? + 1;
            if lport != cnt {
                return Err("SyntaxError: phrase left of the operator is too short.");
            }
            functor(cnt)?;
        }
        cnt += 1;
    }
    Ok(())
}