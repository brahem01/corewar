use anyhow::Result;
use std::fs::File;
use std::io::Write;


pub fn write_cor_file(path: &str, bytes: Vec<u8>) -> Result<()> {
      let mut file = File::create(path)?;
      file.write_all(&bytes)?;
      Ok(())
}