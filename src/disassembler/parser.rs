use std::fs::File;
use std::io::{BufReader, Read};
use anyhow::{Result, anyhow};
use crate::instructions::{INSTRUCTIONS, Instruction, ParamType};

pub struct CorHeader {
    pub name: String,
    pub comment: String,
}

pub struct InstructionInstance {
    pub instr: &'static Instruction,
    pub params: Vec<i32>,
}

pub struct Disassembler {
    pub header: CorHeader,
    pub instructions: Vec<InstructionInstance>,
}

impl Disassembler {
    pub fn read_cor_file(path: &str) -> Result<Disassembler> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;

        // parse header
        let magic = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if magic != 0x00EA83F3 {
            return Err(anyhow!("Invalid magic number"));
        }

        let name = String::from_utf8_lossy(&bytes[4..132]).trim_end_matches('\0').to_string();
        let comment = String::from_utf8_lossy(&bytes[140..2188]).trim_end_matches('\0').to_string();

        let mut disassembler = Disassembler {
            header: CorHeader { name, comment },
            instructions: Vec::new(),
        };

        let mut cursor = 2192; // after header (magic + name + padding1 + prog_size + comment + padding2)

        while cursor < bytes.len() {
            let opcode = bytes[cursor];
            cursor += 1;

            let instr = INSTRUCTIONS.iter()
                .find(|i| i.opcode as u8 == opcode)
                .ok_or_else(|| anyhow!("Unknown opcode {}", opcode))?;

            let pcode = if instr.has_pcode { Some(bytes[cursor]) } else { None };
            if instr.has_pcode { cursor += 1; }

            // parse parameters
            let mut params = Vec::new();
            for (i, allowed) in instr.params.iter().enumerate() {
                let param_type = if let Some(pc) = pcode {
                    match (pc >> (6 - i*2)) & 0b11 {
                        0b01 => ParamType::Register,
                        0b10 => ParamType::Direct,
                        0b11 => ParamType::Indirect,
                        _ => return Err(anyhow!("Invalid param type")),
                    }
                } else {
                    allowed[0]
                };

                let value = match param_type {
                    ParamType::Register => {
                        let v = bytes[cursor] as i32;
                        cursor += 1;
                        v
                    },
                    ParamType::Indirect => {
                        let v = i16::from_be_bytes([bytes[cursor], bytes[cursor+1]]) as i32;
                        cursor += 2;
                        v
                    },
                    ParamType::Direct => {
                        let size = if instr.has_idx { 2 } else { 4 };
                        let v = match size {
                            2 => i16::from_be_bytes([bytes[cursor], bytes[cursor+1]]) as i32,
                            4 => i32::from_be_bytes([bytes[cursor], bytes[cursor+1], bytes[cursor+2], bytes[cursor+3]]),
                            _ => unreachable!(),
                        };
                        cursor += size;
                        v
                    },
                };
                params.push(value);
            }

            disassembler.instructions.push(InstructionInstance {
                instr,
                params,
            });
        }

        Ok(disassembler)
    }
}
