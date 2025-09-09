use std::fs::File;
use std::io::Read;
use anyhow::{Result, anyhow};
use shared::utils::*;
use shared::instructions::{INSTRUCTIONS, Instruction, ParamType};

pub struct CorHeader {
    pub name: String,
    pub comment: String,
}

pub struct InstructionInstance {
    pub instr: &'static Instruction,
    pub params: Vec<i32>,
    pub param_types: Vec<ParamType>, // <- useful to keep
}

pub struct Disassembler {
    pub header: CorHeader,
    pub instructions: Vec<InstructionInstance>,
}

impl Disassembler {
    pub fn read_cor_file(path: &str) -> Result<Disassembler> {
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        // --- Header sanity checks ---
        if bytes.len() < 2192 {
            return Err(anyhow!("File too short to contain full Core header (need >= 2192 bytes)"));
        }

        let magic = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if magic != 0x00EA83F3 {
            return Err(anyhow!("Invalid magic number: {:08x}", magic));
        }

        let name = String::from_utf8_lossy(&bytes[4..132]).trim_end_matches('\0').to_string();
        let comment = String::from_utf8_lossy(&bytes[140..2188]).trim_end_matches('\0').to_string();

        // read program size from header (bytes 136..140)
        let prog_size = u32::from_be_bytes([bytes[136], bytes[137], bytes[138], bytes[139]]) as usize;
        let code_start = 2192usize;
        let code_end = code_start.checked_add(prog_size)
            .ok_or_else(|| anyhow!("program size overflow"))?;
        if code_end > bytes.len() {
            return Err(anyhow!("program size {} extends past file length {}", prog_size, bytes.len()));
        }

        let mut disassembler = Disassembler {
            header: CorHeader { name, comment },
            instructions: Vec::new(),
        };

        let mut cursor = code_start;

        while cursor < code_end {
            let opcode = read_u8(&bytes, &mut cursor)?;
            let instr = INSTRUCTIONS.iter()
                .find(|i| i.opcode as u8 == opcode)
                .ok_or_else(|| anyhow!("Unknown opcode {} at offset {}", opcode, cursor - 1))?;

            // read pcode if the instruction uses one
            let pcode = if instr.has_pcode { Some(read_u8(&bytes, &mut cursor)?) } else { None };

            // parse params
            let mut params = Vec::with_capacity(instr.params.len());
            let mut param_types = Vec::with_capacity(instr.params.len());

            for (i, allowed) in instr.params.iter().enumerate() {
                // resolve param type
                let param_type = if let Some(pc) = pcode {
                    match (pc >> (6 - i * 2)) & 0b11 {
                        0b01 => ParamType::Register,
                        0b10 => ParamType::Direct,
                        0b11 => ParamType::Indirect,
                        0b00 => return Err(anyhow!("Coding byte indicates no param for slot {} (pc={:08b})", i, pc)),
                        _ => unreachable!(),
                    }
                } else {
                    // no pcode: choose the default allowed type (must be present)
                    if allowed.is_empty() {
                        return Err(anyhow!("No default param type available for instruction {:?} param {}", instr.opcode, i));
                    }
                    allowed[0] // <-- assumption: allowed[0] is the canonical type when no pcode
                };

                // validate that the decoded type is allowed by this instruction
                if !allowed.contains(&param_type) {
                    return Err(anyhow!(
                        "Param {} type {:?} not allowed for opcode {:?} at offset {}",
                        i, param_type, instr.opcode, cursor
                    ));
                }

                // read value according to type
                let value = match param_type {
                    ParamType::Register => {
                        read_u8(&bytes, &mut cursor)? as i32
                    }
                    ParamType::Indirect => {
                        read_i16_be(&bytes, &mut cursor)? as i32
                    }
                    ParamType::Direct => {
                        if instr.has_idx {
                            read_i16_be(&bytes, &mut cursor)? as i32
                        } else {
                            read_i32_be(&bytes, &mut cursor)?
                        }
                    }
                };

                params.push(value);
                param_types.push(param_type);
            }

            disassembler.instructions.push(InstructionInstance {
                instr,
                params,
                param_types,
            });
        }

        Ok(disassembler)
    }
}
