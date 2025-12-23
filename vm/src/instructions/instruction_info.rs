#[derive(Copy, Clone)]
pub struct InstructionInfo {
    pub nb_params: usize,
    pub nb_cycles: i32,
    pub has_pcode: bool,
    pub has_idx: bool,
    pub direct_size: usize, // 2 if IDX, 4 otherwise
}

pub const INSTRUCTION_TABLE: [InstructionInfo; 16] = [
    // 1. live
    InstructionInfo {
        nb_params: 1,
        nb_cycles: 10,
        has_pcode: false,
        has_idx: false,
        direct_size: 4,
    },
    // 2. ld
    InstructionInfo {
        nb_params: 2,
        nb_cycles: 5,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 3. st
    InstructionInfo {
        nb_params: 2,
        nb_cycles: 5,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 4. add
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 10,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 5. sub
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 10,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 6. and
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 6,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 7. or
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 6,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 8. xor
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 6,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 9. zjmp
    InstructionInfo {
        nb_params: 1,
        nb_cycles: 20,
        has_pcode: false,
        has_idx: true,
        direct_size: 2,
    },
    // 10. ldi
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 25,
        has_pcode: true,
        has_idx: true,
        direct_size: 2,
    },
    // 11. sti
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 25,
        has_pcode: true,
        has_idx: true,
        direct_size: 2,
    },
    // 12. fork
    InstructionInfo {
        nb_params: 1,
        nb_cycles: 800,
        has_pcode: false,
        has_idx: true,
        direct_size: 2,
    },
    // 13. lld
    InstructionInfo {
        nb_params: 2,
        nb_cycles: 10,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
    // 14. lldi
    InstructionInfo {
        nb_params: 3,
        nb_cycles: 50,
        has_pcode: true,
        has_idx: true,
        direct_size: 2,
    },
    // 15. lfork
    InstructionInfo {
        nb_params: 1,
        nb_cycles: 1000,
        has_pcode: false,
        has_idx: true,
        direct_size: 2,
    },
    // 16. nop
    InstructionInfo {
        nb_params: 1,
        nb_cycles: 2,
        has_pcode: true,
        has_idx: false,
        direct_size: 4,
    },
];
