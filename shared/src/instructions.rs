

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamType {
    Register,
    Direct,
    Indirect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub name: &'static str,
    pub nb_params: u8,
    pub cycles: u32,
    pub has_pcode: bool,
    pub has_idx: bool,
    pub params: &'static [ &'static [ParamType] ],
}

pub fn valid_instruction(s: &str) -> bool {
    INSTRUCTIONS.iter().any(|instr| instr.name == s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Live  = 1,
    Ld   ,
    St   ,
    Add  ,
    Sub  ,
    And  ,
    Or   ,
    Xor  ,
    Zjmp ,
    Ldi  ,
    Sti  ,
    Fork ,
    Lld  ,
    Lldi ,
    Lfork,
    Nop  ,
}


pub const INSTRUCTIONS: [Instruction; 16] = [
    Instruction {
        opcode: Opcode::Live,
        name: "live",
        nb_params: 1,
        cycles: 10,
        has_pcode: false,
        has_idx: false,
        params: &[ &[ParamType::Direct] ],
    },
    Instruction {
        opcode: Opcode::Ld,
        name: "ld",
        nb_params: 2,
        cycles: 5,
        has_pcode: true,
        has_idx: false,
        params: &[
            &[ParamType::Indirect, ParamType::Direct],
            &[ParamType::Register],
        ],
    },
    Instruction {
        opcode: Opcode::St,
        name: "st",
        nb_params: 2,
        cycles: 5,
        has_pcode: true,
        has_idx: false,
        params: &[
            &[ParamType::Register],
            &[ParamType::Register, ParamType::Indirect],
        ],
    },
    Instruction {
        opcode: Opcode::Add,
        name: "add",
        nb_params: 3,
        cycles: 10,
        has_pcode: true,
        has_idx: false,
        params: &[
            &[ParamType::Register],
            &[ParamType::Register],
            &[ParamType::Register],
        ],
    },
    Instruction {
        opcode: Opcode::Sub,
        name: "sub",
        nb_params: 3,
        cycles: 10,
        has_pcode: true,
        has_idx: false,
        params: &[
            &[ParamType::Register],
            &[ParamType::Register],
            &[ParamType::Register],
        ],
    },
    Instruction {
        opcode: Opcode::And,
        name: "and",
        nb_params: 3,
        cycles: 6,
        has_pcode: true,
        has_idx: false,
        params: &[
            &[ParamType::Register, ParamType::Indirect, ParamType::Direct],
            &[ParamType::Register, ParamType::Indirect, ParamType::Direct],
            &[ParamType::Register],
        ],
    },
    Instruction {
        opcode: Opcode::Or,
        name: "or",
        nb_params: 3,
        cycles: 6,
        has_pcode: true,
        has_idx: false,
        params: &[
            &[ParamType::Register, ParamType::Indirect, ParamType::Direct],
            &[ParamType::Register, ParamType::Indirect, ParamType::Direct],
            &[ParamType::Register],
        ],
    },
    Instruction {
        opcode: Opcode::Xor,
        name: "xor",
        nb_params: 3,
        cycles: 6,
        has_pcode: true,
        has_idx: false,
        params: &[
            &[ParamType::Register, ParamType::Indirect, ParamType::Direct],
            &[ParamType::Register, ParamType::Indirect, ParamType::Direct],
            &[ParamType::Register],
        ],
    },
    Instruction {
        opcode: Opcode::Zjmp,
        name: "zjmp",
        nb_params: 1,
        cycles: 20,
        has_pcode: false,
        has_idx: true,
        params: &[ &[ParamType::Direct] ],
    },
    Instruction {
        opcode: Opcode::Ldi,
        name: "ldi",
        nb_params: 3,
        cycles: 25,
        has_pcode: true,
        has_idx: true,
        params: &[
            &[ParamType::Register, ParamType::Indirect, ParamType::Direct],
            &[ParamType::Register, ParamType::Direct],
            &[ParamType::Register],
        ],
    },
    Instruction {
        opcode: Opcode::Sti,
        name: "sti",
        nb_params: 3,
        cycles: 25,
        has_pcode: true,
        has_idx: true,
        params: &[
            &[ParamType::Register],
            &[ParamType::Register, ParamType::Indirect, ParamType::Direct],
            &[ParamType::Register, ParamType::Direct],
        ],
    },
    Instruction {
        opcode: Opcode::Fork,
        name: "fork",
        nb_params: 1,
        cycles: 800,
        has_pcode: false,
        has_idx: true,
        params: &[ &[ParamType::Direct] ],
    },
    Instruction {
        opcode: Opcode::Lld,
        name: "lld",
        nb_params: 2,
        cycles: 10,
        has_pcode: true,
        has_idx: false,
        params: &[
            &[ParamType::Indirect, ParamType::Direct],
            &[ParamType::Register],
        ],
    },
    Instruction {
        opcode: Opcode::Lldi,
        name: "lldi",
        nb_params: 3,
        cycles: 50,
        has_pcode: true,
        has_idx: true,
        params: &[
            &[ParamType::Register, ParamType::Indirect, ParamType::Direct],
            &[ParamType::Register, ParamType::Direct],
            &[ParamType::Register],
        ],
    },
    Instruction {
        opcode: Opcode::Lfork,
        name: "lfork",
        nb_params: 1,
        cycles: 1000,
        has_pcode: false,
        has_idx: true,
        params: &[ &[ParamType::Direct] ],
    },
    Instruction {
        opcode: Opcode::Nop,
        name: "nop",
        nb_params: 1,
        cycles: 2,
        has_pcode: true,
        has_idx: false,
        params: &[ &[ParamType::Register] ],
    },
];
