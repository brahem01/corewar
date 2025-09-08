use std::fs::{File, self};
use std::env;
use std::path::PathBuf;
use std::io::{BufReader, Read};
use anyhow::{anyhow, Result};
use crate::vm::config::{DESCRIPTION_LENGTH, MAX_PLAYERS, MAX_PROGRAM_SIZE};
use crate::vm::warrior::Warrior;

pub fn parse_folder() -> Result<Vec<(Warrior, Vec<u8>)>> {
      let dir = env::var("BINARIES")?; 
      let files = list_files(&dir);
      if files.len() > MAX_PLAYERS || files.len() < 2 {
        return Err(anyhow!("the warriors number should be between 2 and 4"));
      }
      let mut warriors_data = Vec::new();
      for file in files {
        if let Ok(warrior) = parse_file(&file) {
            warriors_data.push(warrior);
        } 
      }
      Ok(warriors_data)
}

pub fn parse_file(path: &str) -> Result<(Warrior, Vec<u8>)> {
      let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;

        // parse header
        let magic = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if magic != 0x00EA83F3 {
            return Err(anyhow!("Invalid magic number"));
        }
        let mut cursor = 4;
        let name = String::from_utf8_lossy(&bytes[cursor..cursor+MAX_PROGRAM_SIZE]).trim_end_matches('\0').to_string();
        cursor+=8;
        let comment = String::from_utf8_lossy(&bytes[cursor..cursor+DESCRIPTION_LENGTH]).trim_end_matches('\0').to_string();
        let cursor = 2192;
        Ok((Warrior::new(name, comment), bytes[cursor..].to_vec()))
}

fn list_files(path: &str) -> Vec<String> {
      let mut files = Vec::new();
      if let Ok(entries) = fs::read_dir(path) {
          for entry in entries.flatten() {
              let path: PathBuf = entry.path();
              if path.is_file() {
                  if let Some(s) = path.to_str() {
                      files.push(s.to_string());
                  }
              }
          }
      }
  
      files
  }