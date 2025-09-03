use crate::instructions::{Instruction, ParamType};
use super::parser::InstructionInstance;

pub fn encode_instruction(inst: &InstructionInstance) -> Vec<u8> {
    let mut bytes = Vec::new();

    // 1️⃣ Opcode
    bytes.push(inst.instr.opcode as u8);

    // 2️⃣ Parameter coding byte (pcode)
    if inst.instr.has_pcode {
        let mut pcode: u8 = 0;

        for (i, param) in inst.params.iter().enumerate() {
            let code = match param.param_type {
                ParamType::Register => 0b01,
                ParamType::Direct   => 0b10,
                ParamType::Indirect => 0b11,
            };

            // Pcode: first param occupies bits 6-7, second 4-5, third 2-3
            let shift = 6 - i * 2;
            pcode |= code << shift;
        }
        bytes.push(pcode);
    }

    // 3️⃣ Parameter values
    for param in &inst.params {
        match param.param_type {
            ParamType::Register => bytes.push(param.value as u8),
            ParamType::Direct => {
                let val = param.value as i16; // normally 2 bytes, could be 4 for live/ld
                bytes.extend_from_slice(&val.to_be_bytes());
            }
            ParamType::Indirect => {
                let val = param.value as i16;
                bytes.extend_from_slice(&val.to_be_bytes());
            }
        }
    }

    bytes
}

// Encode a full program
pub fn encode_program(insts: &[InstructionInstance]) -> Vec<u8> {
    let mut program = Vec::new();
    for inst in insts {
        program.extend_from_slice(&encode_instruction(inst));
    }
    program
}
