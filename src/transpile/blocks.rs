pub mod parser;
pub mod stdlib;
pub mod tree;

use crate::transpile::transpile;
use tree::*;
pub use parser::*;

pub type Mem = Vec<CodeTree>;

pub mod block_if;
pub use block_if::*;

pub mod block_new;
pub use block_new::*;

pub mod block_repeat;
pub use block_repeat::*;