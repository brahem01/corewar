mod executer;
mod memory;
mod parser;
mod warrior;
use anyhow::{Result};



pub fn vm() -> Result<()> {
      let warriors_data: Vec<(warrior::Warrior, Vec<u8>)> = parser::parse_folder()?;
      let mut arena = memory::Arena::new();
      let warriors = arena.setup_warriors(warriors_data)?;
      let mut executer = executer::Executer::new(arena, warriors);
      for warrior in executer.warriors {
            
      }
      Ok(())
}