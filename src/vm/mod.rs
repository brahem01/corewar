mod cpu;
mod executer;
mod memory;
mod parser;
mod warrior;
use anyhow::{Result};



pub fn vm() -> Result<()> {
      let warriors_data = parser::parse_folder()?;
      let mut arena = memory::Arena::new();
      let warriors = arena.setup_warriors(warriors_data)?;
      let cpu = cpu::Cpu::new();
      let mut executer = executer::Executer::new(arena, warriors, cpu);
      for warrior in executer.warriors {
      }
      Ok(())
}