pub mod parser;
pub mod lexer;
pub mod encoder;
pub mod errors;

use anyhow::Result;
use std::path::Path;

pub fn run_file(path: &str) -> Result<()> {
    parser::parse_file(Path::new(path))
}
