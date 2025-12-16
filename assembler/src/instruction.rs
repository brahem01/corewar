use shared::instructions::{ Instruction, ParamType };
use std::collections::HashMap;

#[derive(Debug)]
pub struct InstructionInstance {
    pub label: Option<String>,
    pub instr: Option<&'static Instruction>,
    pub params: Vec<Param>,
}

#[derive(Debug)]
pub struct Param {
    pub param_type: ParamType,
    pub value: ValueType,
}

#[derive(Debug)]
pub enum ValueType {
    value(i32),
    label(String),
}

impl InstructionInstance {
    pub fn new() -> Self {
        return InstructionInstance{
            params: Vec::new(),
            instr: None,
            label: None
        }
    }

    pub fn label(&self) -> Option<String> {
        for param in &self.params {
            if ParamType::Direct == param.param_type && let ValueType::label(l) = &param.value {
                return Some(l.to_string());
            }
        }
        None
    }

    pub fn calculate_instruction_size(&self) -> usize {
        return self.compute_instruction_size() as usize;
    }

    pub fn encode(&self, current_position: usize, labels: &HashMap<String, usize>) -> Vec<u8> {
        let mut buffer = Vec::new();
        let instr;
        if let Some(i) = self.instr {
            instr = i;
        } else {
            return buffer;
        }
        
        buffer.push(instr.opcode as u8); // opcode
        if instr.has_pcode {
            let pcode = compute_pcode(&self.params);
            buffer.push(pcode);
        }


        for param in &self.params {
            let value = match &param.value {
                ValueType::value(v) => *v,
                ValueType::label(label_name) => {
                    // Resolve label to offset
                    let target_pos = labels
                        .get(label_name)
                        .expect(&format!("Undefined label: {}", label_name));
                    (*target_pos as i32) - (current_position as i32)
                }
            };

            match param.param_type {
                ParamType::Register => {
                    buffer.push(value as u8);
                }
                ParamType::Direct => {
                    if instr.has_idx {
                        buffer.extend(&(value as i16).to_be_bytes());
                    } else {
                        buffer.extend(&value.to_be_bytes());
                    }
                }
                ParamType::Indirect => {
                    buffer.extend(&(value as i16).to_be_bytes());
                }
            }
        }

        buffer
    }

    pub fn compute_instruction_size(&self) -> u32 {
        let mut size = 1; // opcode
        let instr;
        if let Some(i) = self.instr {
            instr = i;
        } else {
            return 0;
        }
        if instr.has_pcode {
            size += 1;
        }

        for param in &self.params {
            size += match param.param_type {
                ParamType::Register => 1,
                ParamType::Indirect => 2,
                ParamType::Direct => {
                    if instr.has_idx { 2 } else { 4 }
                }
            };
        }

        size
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

// _______________ pcode ______________________
// 10 01 11                                   |
// r  %   _                                   | this "_" represent the space
// 10 00 00 00 << 00 00 00 10                 | 1st shift by 6
// 00 01 00 00 << 00 00 00 01                 | 2nd shift by 4
// 00 00 11 00 << 00 00 00 11                 | 3rd shift by 2
// then when we apply or the result will be   |
// 10 01 11 00 that's the result of OR(|)     |
// ___________________________________________|
