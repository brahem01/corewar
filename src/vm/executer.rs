use crate::vm::*;

pub struct Executer {
      pub arena: memory::Arena, 
      pub warriors: Vec<warrior::Warrior>,
}

impl Executer {
    pub fn new(arena: memory::Arena, warriors: Vec<warrior::Warrior>) -> Self{
      Self { arena, warriors }
    }

    pub fn execute_cycle(&mut self) {
      for warrior in &mut self.warriors {
          if !warrior.alive { continue; }

          // 1) Fetch instruction at warrior.pc
          if let Some(instr) = &self.arena.memory[warrior.pc % 4096] {
              // 2) Execute instruction
              // execute(&mut self.arena, instr);

              // 3) Advance PC
              warrior.pc = (warrior.pc + instr.size_bytes) % 4096;
          } else {
              // No instruction? Just advance
              warrior.pc = (warrior.pc + 1) % 4096;
          }
      }
    }

   
}

pub fn execute(arena: &mut memory::Arena, instr: &memory::ExecutableInstruction) {}
