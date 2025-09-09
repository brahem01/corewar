pub mod parser;
pub mod lexer;
pub mod encoder;
pub mod errors;

use anyhow::{Result, anyhow};
use std::path::Path;

pub fn run_file(path: &str) -> Result<Vec<u8>> {
    let player = parser::parse_file(Path::new(path))
        .map_err(|e| anyhow!("Error parsing the file: {}\nerr: {}", path, e))?;
    
    let bin_data = encoder::encode(player)
        .map_err(|e| anyhow!("Error encoding the data of: {}\nerr: {}", path, e))?;
    
    Ok(bin_data)
}

