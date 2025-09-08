use crate::vm::*;

pub struct Executer {
      pub arena: memory::Arena, 
      pub warriors: Vec<warrior::Warrior>,
      pub cpu: cpu::Cpu,
}

impl Executer {
    pub fn new(arena: memory::Arena, warriors: Vec<warrior::Warrior>, cpu: cpu::Cpu) -> Self{
      Self { arena, warriors, cpu }
    }
}