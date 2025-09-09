use shared::{instructions::*, utils::*};
use anyhow::{anyhow, Ok, Result};
use crate::{config::*, warrior::Warrior};

#[derive(Debug, Clone)]
pub struct ExecutableInstruction {
    pub instruction: &'static Instruction,
    pub params: Vec<i32>,
    pub param_types: Vec<ParamType>, // Actual parameter types from pcode
    pub size_bytes: usize,
}

#[derive(Debug)]
pub struct Arena {
      pub memory: [Option<ExecutableInstruction>; ARENA_SIZE],
}

impl Arena {
      pub fn new() -> Self {
            Self{
                  memory: std::array::from_fn(|_| None)
            }
      }

      pub fn setup_warriors(&mut self, w_data: Vec<(Warrior, Vec<u8>)>) -> Result<Vec<Warrior>> {
            let mut warriors = Vec::new();
            let warriors_number = w_data.len();
            let spacing = MEM_SIZE/warriors_number;

            for (i, (mut warrior, bytecode)) in w_data.into_iter().enumerate() {
                  let start_pos = (i*spacing)%MEM_SIZE;
                  self.load_program(start_pos, &bytecode, i)?;

                  // i didn't understand exactly why the first register should contains the negative player id yet: 
                  warrior.registers[0] = -(i as i32 + 1);
                  warrior.pc = start_pos;
                  warrior.id = i as u8 +1;
                  warriors.push(warrior.clone());
            }
            Ok(warriors)     
      }

      fn load_program(&mut self, mut start_pos: usize, bytecode: &[u8], warrior_id: usize) -> Result<()> {
            if bytecode.is_empty() {
                return Err(anyhow!("Empty program for warrior {}", warrior_id));
            }
    
            let mut cursor = 0;
            while cursor < bytecode.len() {
                let (exec_instr, consumed) = decode_instruction(bytecode, cursor)?;
                self.write_instruction(start_pos, exec_instr)?;
                start_pos = (start_pos + 1) % MEM_SIZE;
                cursor += consumed;
            }
    
            Ok(())
      }



          /// Write executable instruction to memory
      pub fn write_instruction(&mut self, address: usize, instruction: ExecutableInstruction) -> Result<()> {
            let normalized_addr = address % MEM_SIZE;
            self.memory[normalized_addr] = Some(instruction);
            Ok(())
      }
}

pub fn decode_instruction(bytes: &[u8], cursor: usize) -> Result<(ExecutableInstruction, usize)> {
    if cursor >= bytes.len() {
        return Err(anyhow!("Unexpected EOF: cursor {cursor} >= file length"));
    }

    let mut offset = cursor;

    // 1) Read opcode
    let opcode = *bytes.get(offset).ok_or_else(|| anyhow!("Missing opcode at {offset}"))?;
    offset += 1;

    let instr = INSTRUCTIONS
        .iter()
        .find(|i| i.opcode as u8 == opcode)
        .ok_or_else(|| anyhow!("Unknown opcode {opcode} at {cursor}"))?;

    // 2) Read coding byte (if present)
    let pcode = if instr.has_pcode {
        let val = *bytes.get(offset).ok_or_else(|| anyhow!("Missing pcode at {offset}"))?;
        offset += 1;
        Some(val)
    } else {
        None
    };

    // 3) Decode params
    let mut params = Vec::with_capacity(instr.params.len());
    let mut param_types = Vec::with_capacity(instr.params.len());

    for (i, allowed) in instr.params.iter().enumerate() {
        let param_type = if let Some(pc) = pcode {
            match (pc >> (6 - i * 2)) & 0b11 {
                0b01 => ParamType::Register,
                0b10 => ParamType::Direct,
                0b11 => ParamType::Indirect,
                _ => return Err(anyhow!("Invalid param type in pcode at param {i}")),
            }
        } else {
            allowed[0] // convention: default type if no pcode
        };

        if !allowed.contains(&param_type) {
            return Err(anyhow!("Param type {:?} not allowed for opcode {opcode}", param_type));
        }

        // 4) Read value depending on type
        let value = match param_type {
            ParamType::Register => {
                let v = *bytes.get(offset).ok_or_else(|| anyhow!("Missing register byte at {offset}"))?;
                offset += 1;
                v as i32
            }
            ParamType::Indirect => {
                let slice = bytes.get(offset..offset + 2).ok_or_else(|| anyhow!("Missing indirect bytes at {offset}"))?;
                offset += 2;
                i16::from_be_bytes([slice[0], slice[1]]) as i32
            }
            ParamType::Direct => {
                let size = if instr.has_idx { 2 } else { 4 };
                let slice = bytes.get(offset..offset + size).ok_or_else(|| anyhow!("Missing direct bytes at {offset}"))?;
                offset += size;
                match size {
                    2 => i16::from_be_bytes([slice[0], slice[1]]) as i32,
                    4 => i32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]),
                    _ => unreachable!(),
                }
            }
        };

        params.push(value);
        param_types.push(param_type);
    }

    let size_bytes = offset - cursor;

    Ok((
        ExecutableInstruction {
            instruction: instr,
            params,
            param_types,
            size_bytes,
        },
        size_bytes,
    ))
}
