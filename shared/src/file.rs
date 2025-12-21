use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Write;

pub fn write_binary_to_file(binaries: Vec<u8>, path: &str) -> Result<(), String> {
    let file_path = Path::new(path);

    // Ensure parent directory exists
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)
                    .map_err(|e| format!("error creating the parent folders for the cor file\nerror: {e}"))?;
    }

    // Create (or truncate) the file and write bytes
    let mut file = File::create(file_path)
                    .map_err(|e| format!("error creating the cor file\nerror: {e}"))?;
    file.write_all(&binaries)
                    .map_err(|e| format!("error writing into the file: {e}"))?;

    Ok(())
}

pub fn validate_assembly_file(s: &str) -> Result<String, String> {
    let path = Path::new(s);

    // Check extension
    if path.extension().and_then(|ext| ext.to_str()) != Some("s") {
        return Err(format!("Invalid file extension for {s}, expected .s"));
    }

    // Extract filename (without extension)
    let file_stem = path.file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| format!("Invalid file name"))?;

    // Build new path: players/bin/<filename>.cor
    let mut new_path = PathBuf::from("players/bin");
    new_path.push(format!("{file_stem}.cor"));

    Ok(new_path.to_string_lossy().into_owned())
}


pub fn validate_core_file(s: &str) -> Result<String, String> {
    let path = Path::new(s);

    // Check extension
    if path.extension().and_then(|ext| ext.to_str()) != Some("cor") {
        return Err(format!("Invalid file extension, expected .s"));
    }

    // Extract filename (without extension)
    let file_stem = path.file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| format!("Invalid file name"))?;

    // Build new path: players/bin/<filename>.cor
    let mut new_path = PathBuf::from("players/bin");
    new_path.push(format!("{file_stem}.s"));

    Ok(new_path.to_string_lossy().into_owned())
}