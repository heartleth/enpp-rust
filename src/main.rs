pub extern crate text_io;
pub mod transpile;
use transpile::*;
mod runner;
use runner::*;
use std::env;

fn main()-> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl::repl();
    }
    else {
        filesys::convert_to_cpp(&args[1], "cpp")?;
    }
    Ok(())
}
