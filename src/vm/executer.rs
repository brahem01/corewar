use crate::vm::*;
use crate::instructions::*;

use crate::vm::memory::{Arena, ExecutableInstruction};
use crate::vm::warrior::Warrior;

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
    fn execute_opcode(&mut self, warrior_id: usize, instr: &ExecutableInstruction) {
      match crate::instructions::Opcode::from(instr.instruction.opcode) {
          Opcode::Live => live(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::Ld   => ld(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::St   => st(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::Add  => add(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::Sub  => sub(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::And  => and(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::Or   => or(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::Xor  => xor(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::Zjmp => zjmp(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::Ldi  => ldi(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::Sti  => sti(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::Fork => fork(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::Lld  => lld(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::Lldi => lldi(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::Lfork=> lfork(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
          Opcode::Nop  => nop(&mut self.arena, &mut self.warriors[warrior_id-1], instr),
      }
  }
  
   
}



pub fn live(_arena: &mut Arena, warrior: &mut Warrior, _instr: &ExecutableInstruction) {
    warrior.alive = true;
    println!("Warrior {} says LIVE", warrior.name);
}

pub fn ld(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    let val = instr.params[0];
    let reg_idx = instr.params[1] as usize;
    warrior.registers[reg_idx] = val;
    warrior.carry = val == 0;
}

pub fn st(arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    let val = warrior.registers[instr.params[0] as usize];
    let addr = (warrior.pc as i32 + instr.params[1]) as usize % 4096;
    arena.memory[addr] = Some(ExecutableInstruction {
        instruction: instr.instruction,
        params: vec![val],
        param_types: vec![crate::instructions::ParamType::Direct],
        size_bytes: 1,
    });
}

pub fn add(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    let r1 = instr.params[0] as usize;
    let r2 = instr.params[1] as usize;
    let r3 = instr.params[2] as usize;
    let sum = warrior.registers[r1] + warrior.registers[r2];
    warrior.registers[r3] = sum;
    warrior.carry = sum == 0;
}

pub fn sub(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    let r1 = instr.params[0] as usize;
    let r2 = instr.params[1] as usize;
    let r3 = instr.params[2] as usize;
    let diff = warrior.registers[r1] - warrior.registers[r2];
    warrior.registers[r3] = diff;
    warrior.carry = diff == 0;
}

pub fn and(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    let r1 = instr.params[0] as usize;
    let r2 = instr.params[1] as usize;
    let r3 = instr.params[2] as usize;
    let val = warrior.registers[r1] & warrior.registers[r2];
    warrior.registers[r3] = val;
    warrior.carry = val == 0;
}

pub fn or(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    let r1 = instr.params[0] as usize;
    let r2 = instr.params[1] as usize;
    let r3 = instr.params[2] as usize;
    let val = warrior.registers[r1] | warrior.registers[r2];
    warrior.registers[r3] = val;
    warrior.carry = val == 0;
}

pub fn xor(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    let r1 = instr.params[0] as usize;
    let r2 = instr.params[1] as usize;
    let r3 = instr.params[2] as usize;
    let val = warrior.registers[r1] ^ warrior.registers[r2];
    warrior.registers[r3] = val;
    warrior.carry = val == 0;
}

pub fn zjmp(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    if warrior.carry {
        warrior.pc = ((warrior.pc as i32 + instr.params[0]) % 4096) as usize;
    }
}

pub fn ldi(arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    // Load value from arena indirectly
    let addr = ((warrior.pc as i32 + instr.params[0] + instr.params[1]) % 4096) as usize;
    if let Some(cell) = &arena.memory[addr] {
        let reg_idx = instr.params[2] as usize;
        warrior.registers[reg_idx] = cell.params[0];
        warrior.carry = warrior.registers[reg_idx] == 0;
    }
}

pub fn sti(arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    let val = warrior.registers[instr.params[0] as usize];
    let addr = ((warrior.pc as i32 + instr.params[1] + instr.params[2]) % 4096) as usize;
    arena.memory[addr] = Some(ExecutableInstruction {
        instruction: instr.instruction,
        params: vec![val],
        param_types: vec![crate::instructions::ParamType::Direct],
        size_bytes: 1,
    });
}

pub fn fork(_arena: &mut Arena, _warrior: &mut Warrior, _instr: &ExecutableInstruction) {
    // TODO: spawn new process
}

pub fn lld(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    let val = instr.params[0];
    let reg_idx = instr.params[1] as usize;
    warrior.registers[reg_idx] = val;
    warrior.carry = val == 0;
}

pub fn lldi(arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    let addr = ((warrior.pc as i32 + instr.params[0] + instr.params[1]) % 4096) as usize;
    if let Some(cell) = &arena.memory[addr] {
        let reg_idx = instr.params[2] as usize;
        warrior.registers[reg_idx] = cell.params[0];
        warrior.carry = warrior.registers[reg_idx] == 0;
    }
}

pub fn lfork(_arena: &mut Arena, _warrior: &mut Warrior, _instr: &ExecutableInstruction) {
    // TODO: spawn new process far away
}

pub fn nop(_arena: &mut Arena, _warrior: &mut Warrior, _instr: &ExecutableInstruction) {
    // Do nothing
}
