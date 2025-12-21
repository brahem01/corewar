pub mod parser;
pub mod lexer;
pub mod encoder;
pub mod instruction;

use std::path::Path;

pub fn run_file(path: &str) -> Result<Vec<u8>, String> {
    let player = parser::parse_file(Path::new(path))
        .map_err(|e| format!("Error parsing the file: {}\nerr: {}", path, e))?;
    
    let bin_data = encoder::encode(player)
        .map_err(|e| format!("Error encoding the data of: {}\nerr: {}", path, e))?;
    
    Ok(bin_data)
}

