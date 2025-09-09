use super::parser::{InstructionInstance, Param};
use crate::parser::Player;
use shared::instructions::{Instruction, ParamType, INSTRUCTIONS};
use anyhow::Result;

struct CorHeader {
    magic: u32,
    name: [u8; 128],
    padding1: [u8; 4],
    prog_size: u32,
    comment: [u8; 2048],
    padding2: [u8; 4],
}

impl CorHeader {
    fn new(name_str: &str, comment_str: &str, prog_size: u32) -> Self {
        let mut name = [0u8; 128];
        let mut comment = [0u8; 2048];
        name[..name_str.len()].copy_from_slice(name_str.as_bytes());
        comment[..comment_str.len()].copy_from_slice(comment_str.as_bytes());
        Self {
            magic: 0x00EA83F3,
            name,
            padding1: [0u8; 4],
            prog_size,
            comment,
            padding2: [0u8; 4],
        }
    }
}

fn compute_pcode(params: &[Param]) -> u8 {
    let mut pcode = 0u8;
    for (i, param) in params.iter().enumerate() {
        let bits = match param.param_type {
            ParamType::Register => 0b01,
            ParamType::Direct => 0b10,
            ParamType::Indirect => 0b11,
        };
        pcode |= bits << (6 - 2 * i);
    }
    pcode
}

fn compute_program_size(instructions: &[InstructionInstance]) -> u32 {
    instructions.iter().map(|inst| {
        let mut size = 1; // opcode
        if inst.instr.has_pcode { size += 1; }
        for param in &inst.params {
            size += match param.param_type {
                ParamType::Register => 1,
                ParamType::Indirect => 2,
                ParamType::Direct => if inst.instr.has_idx { 2 } else { 4 },
            };
        }
        size
    }).sum()
}



pub fn encode(player: Player) -> Result<Vec<u8>> {
    let prog_size = compute_program_size(&player.instructions);
    let head = CorHeader::new(&player.name, &player.comment, prog_size);

    let mut buffer = Vec::new();

    // Serialize header
    buffer.extend(&head.magic.to_be_bytes()); 
    buffer.extend(&head.name);                
    buffer.extend(&head.padding1);           
    buffer.extend(&head.prog_size.to_be_bytes()); 
    buffer.extend(&head.comment);            
    buffer.extend(&head.padding2);           

    // Encode each instruction
    for inst in &player.instructions {
        //if its a label register the adress < labelDef>
        buffer.push(inst.instr.opcode as u8); // opcode

        if inst.instr.has_pcode {
            let pcode = compute_pcode(&inst.params);
            buffer.push(pcode);
        }

        for param in &inst.params {
            match param.param_type {
                ParamType::Register => buffer.push(param.value as u8),
                ParamType::Direct => {
                    if inst.instr.has_idx {
                        buffer.extend(&(param.value as i16).to_be_bytes());
                    } else {
                        buffer.extend(&param.value.to_be_bytes());
                    }
                }
                ParamType::Indirect => {
                    buffer.extend(&(param.value as i16).to_be_bytes());
                }
            }
        }
    }

    Ok(buffer)
}
