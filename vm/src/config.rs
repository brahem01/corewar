// Sizes
pub const IND_SIZE: usize = 2;
pub const REG_SIZE: usize = 4;
pub const DIR_SIZE: usize = REG_SIZE;

// Parameter codes
pub const REG_CODE: u8 = 1;
pub const DIR_CODE: u8 = 2;
pub const IND_CODE: u8 = 3;

// Game limits
pub const MAX_PLAYERS: usize = 4;
pub const MEM_SIZE: usize = 4 * 1024;
pub const IDX_MOD: usize = MEM_SIZE / 8;
pub const PLAYER_MAX_SIZE: usize = MEM_SIZE / 6;

// Characters in assembly (needed for parsing instruction parameters)
pub const COMMENT_CHAR: char = '#';
pub const LABEL_CHAR: char = ':';
pub const DIRECT_CHAR: char = '%';
pub const SEPARATOR_CHAR: char = ',';
pub const LABEL_CHARS: &str = "abcdefghijklmnopqrstuvwxyz_0123456789";

// Registers
pub const REG_NUMBER: usize = 16;

// Cycle management
pub const CYCLE_TO_DIE: usize = 1536;
pub const CYCLE_DELTA: usize = 50;
pub const NBR_LIVE: usize = 21;
pub const MAX_CHECKS: usize = 10;

// Program metadata limits
pub const PROG_NAME_LENGTH: usize = 128;
pub const DESCRIPTION_LENGTH: usize = 2048;

// File signature for .cor binaries
pub const COREWAR_EXEC_SIGNATURE: u32 = 0xea83f3;
