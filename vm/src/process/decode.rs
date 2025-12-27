use super::Process;
use crate::arena::*;
use crate::helper;
use crate::instructions;
use crate::instructions::*;
use crate::process::ParamType;
use crate::process::instruction_info::*; //INSTRUCTION_TABLE};

impl Process {
    pub fn decode(&mut self, opcode: u8, arena: &mut Arena) -> Option<Instruction> {
        let inst_index = (opcode - 1) as usize;
        let inst_info = INSTRUCTION_TABLE[inst_index]; // instructions table is 1-indexed
        self.remaining_cycles = inst_info.nb_cycles - 1; //.saturating_sub);

        let opcode_addr = self.pc.get() - 1;

        if inst_info.has_pcode {
            let pcode = arena.read(self.pc.get(), 1)[0];
            self.pc.inc();

            let type_params = decode_pcode(pcode, inst_info.nb_params);
            if !is_valid_params(opcode, &type_params) {
                //println!("Invalid parameter {:?}", type_params);
                return None;
            }
            let params = self.build_params(type_params, inst_info, arena);
            // decode parameters
            Some(Instruction::new(opcode, params, opcode_addr))
            // verify integraty
        } else {
            let size = if inst_info.has_idx {
                ////println!("we are going to fetch just 2");
                2
            } else {
                ////println!("we are going to fetch 4");
                4
            };
            let bytes = arena.read(self.pc.get(), size);
            ////println!("the value we fetched bytes is {:?}", bytes);
            self.pc.add(size);
            let value = helper::bytes_to_i32(&bytes); // sign-extend 2-byte or 4-byte to i32

            ////println!("the value we fetched is {}", value);
            return Some(Instruction::new(
                opcode,
                vec![Parameter::Direct(value)],
                opcode_addr,
            ));
        }
    }

    fn build_params(
        &mut self,
        type_params: [ParamType; 3],
        inst_info: InstructionInfo,
        arena: &mut Arena,
    ) -> Vec<instructions::Parameter> {
        let mut params = Vec::new();
        for param_type in type_params.iter() {
            let param = match param_type {
                ParamType::Direct => {
                    let size = if inst_info.has_idx { 2 } else { 4 };
                    let bytes = arena.read(self.pc.get(), size);
                    self.pc.add(size);
                    Parameter::Direct(helper::bytes_to_i32(&bytes))
                }
                ParamType::Indirect => {
                    let bytes = arena.read(self.pc.get(), 2);
                    self.pc.add(2);
                    let offset = helper::bytes_to_i16(&bytes);
                    Parameter::Indirect(offset as i32)
                }
                ParamType::Register => {
                    let reg = arena.read(self.pc.get(), 1)[0] as usize;
                    self.pc.inc();
                    if reg < 1 || reg > 16 {
                        // Invalid register number, return None or handle error appropriately
                        // For now, returning None will cause the instruction to be skipped
                        Parameter::None
                    } else {
                        Parameter::Register(reg)
                    }
                }
                _ => Parameter::None,
            };
            params.push(param);
        }
        return params.clone();
    }
}

fn decode_pcode(pcode: u8, num_args: usize) -> [ParamType; 3] {
    let mut result = [ParamType::None; 3];

    for i in 0..num_args {
        let shift = 6 - (i * 2);
        let bits = (pcode >> shift) & 0b11;
        result[i] = match bits {
            0b01 => ParamType::Register,
            0b10 => ParamType::Direct,
            0b11 => ParamType::Indirect,
            _ => ParamType::None, // 0b00 means unused/invalid
        };
    }

    result
}

fn is_valid_params(opcode: u8, type_params: &[ParamType; 3]) -> bool {
    match opcode {
        // -------------------------------------------------------------------------
        // live %<direct>
        // -------------------------------------------------------------------------
        1 => matches!(type_params.get(0), Some(ParamType::Direct)),

        // -------------------------------------------------------------------------
        // ld <direct|indirect>, <register>
        // -------------------------------------------------------------------------
        2 => {
            let first_ok = matches!(
                type_params.get(0),
                Some(ParamType::Direct) | Some(ParamType::Indirect)
            );
            let second_ok = matches!(type_params.get(1), Some(ParamType::Register));
            first_ok && second_ok
        }

        // -------------------------------------------------------------------------
        // st <register>, <register|indirect>
        // -------------------------------------------------------------------------
        3 => {
            let first_ok = matches!(type_params.get(0), Some(ParamType::Register));
            let second_ok = matches!(
                type_params.get(1),
                Some(ParamType::Register) | Some(ParamType::Indirect)
            );
            first_ok && second_ok
        }

        // -------------------------------------------------------------------------
        // add / sub <register>, <register>, <register>
        // -------------------------------------------------------------------------
        4 | 5 => type_params
            .iter()
            .take(3)
            .all(|p| matches!(p, ParamType::Register)),

        // -------------------------------------------------------------------------
        // and / or / xor <reg|ind|dir>, <reg|ind|dir>, <register>
        // -------------------------------------------------------------------------
        6 | 7 | 8 => {
            let first_ok = matches!(
                type_params.get(0),
                Some(ParamType::Register) | Some(ParamType::Direct) | Some(ParamType::Indirect)
            );
            let second_ok = matches!(
                type_params.get(1),
                Some(ParamType::Register) | Some(ParamType::Direct) | Some(ParamType::Indirect)
            );
            let third_ok = matches!(type_params.get(2), Some(ParamType::Register));
            first_ok && second_ok && third_ok
        }

        // -------------------------------------------------------------------------
        // zjmp %<direct>
        // -------------------------------------------------------------------------
        9 => matches!(type_params.get(0), Some(ParamType::Direct)),

        // -------------------------------------------------------------------------
        // ldi / sti / lldi <various>
        // -------------------------------------------------------------------------
        10 => {
            let first_ok = matches!(
                type_params.get(0),
                Some(ParamType::Register) | Some(ParamType::Direct) | Some(ParamType::Indirect)
            );
            let second_ok = matches!(
                type_params.get(1),
                Some(ParamType::Register) | Some(ParamType::Direct)
            );
            let third_ok = matches!(type_params.get(2), Some(ParamType::Register));
            first_ok && second_ok && third_ok
        }

        11 => {
            let first_ok = matches!(type_params.get(0), Some(ParamType::Register));
            let second_ok = matches!(
                type_params.get(1),
                Some(ParamType::Register) | Some(ParamType::Direct) | Some(ParamType::Indirect)
            );
            let third_ok = matches!(
                type_params.get(2),
                Some(ParamType::Register) | Some(ParamType::Direct)
            );
            first_ok && second_ok && third_ok
        }

        12 | 15 => matches!(type_params.get(0), Some(ParamType::Direct)), // fork / lfork

        13 => {
            let first_ok = matches!(
                type_params.get(0),
                Some(ParamType::Direct) | Some(ParamType::Indirect)
            );
            let second_ok = matches!(type_params.get(1), Some(ParamType::Register));
            first_ok && second_ok
        }

        14 => {
            let first_ok = matches!(
                type_params.get(0),
                Some(ParamType::Register) | Some(ParamType::Direct) | Some(ParamType::Indirect)
            );
            let second_ok = matches!(
                type_params.get(1),
                Some(ParamType::Register) | Some(ParamType::Direct)
            );
            let third_ok = matches!(type_params.get(2), Some(ParamType::Register));
            first_ok && second_ok && third_ok
        }

        // -------------------------------------------------------------------------
        // nop <register>
        // -------------------------------------------------------------------------
        16 => matches!(type_params.get(0), Some(ParamType::Register)),

        // -------------------------------------------------------------------------
        _ => {
            //println!("Unknown opcode {}", opcode);
            false
        }
    }
}
