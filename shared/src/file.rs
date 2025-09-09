use std::path::{Path, PathBuf};
use anyhow::{Result, anyhow};
use std::fs::{self, File};
use std::io::Write;

pub fn write_binary_to_file(binaries: Vec<u8>, path: &str) -> Result<()> {
    let file_path = Path::new(path);

    // Ensure parent directory exists
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Create (or truncate) the file and write bytes
    let mut file = File::create(file_path)?;
    file.write_all(&binaries)?;

    Ok(())
}

pub fn validate_assembly_file(s: &str) -> Result<String> {
    let path = Path::new(s);

    // Check extension
    if path.extension().and_then(|ext| ext.to_str()) != Some("s") {
        return Err(anyhow!("Invalid file extension, expected .s"));
    }

    // Extract filename (without extension)
    let file_stem = path.file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| anyhow!("Invalid file name"))?;

    // Build new path: players/bin/<filename>.cor
    let mut new_path = PathBuf::from("players/bin");
    new_path.push(format!("{file_stem}.cor"));

    Ok(new_path.to_string_lossy().into_owned())
}


pub fn validate_core_file(s: &str) -> Result<String> {
    let path = Path::new(s);

    // Check extension
    if path.extension().and_then(|ext| ext.to_str()) != Some("cor") {
        return Err(anyhow!("Invalid file extension, expected .s"));
    }

    // Extract filename (without extension)
    let file_stem = path.file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| anyhow!("Invalid file name"))?;

    // Build new path: players/bin/<filename>.cor
    let mut new_path = PathBuf::from("players/bin");
    new_path.push(format!("{file_stem}.s"));

    Ok(new_path.to_string_lossy().into_owned())
}