use super::transpile::*;
use rand::distributions::Alphanumeric;
use std::io::Write;
use std::fs::File;
use std::io::Read;
use rand::Rng;

pub static mut CURRENT_FILE :String = String::new();

pub fn convert_to_cpp(path :&String, ext :&str)->std::io::Result<()> {
    unsafe { CURRENT_FILE = String::from(&path[..]) }
    
    let mut target = File::create(format!("{}.{}", path, ext))?;
    let mut source_file = File::open(&format!("{}.epp", path))?;
    let mut source = String::new();
    source_file.read_to_string(&mut source)?;
    let converted :String = transpile(&tree::CodeTree::treeify(&source), 0);
    if ext == "hpp" {
        target.write_all(format!("#ifndef __{token}_H__\n#define __{token}_H__\n{contents}\n#endif",
            token = rand::thread_rng().sample_iter(&Alphanumeric).take(20).collect::<String>(),
            contents = converted
        ).as_bytes())?;
    }
    else {
        target.write_all(converted.as_bytes())?;
    }
    Ok(())
}