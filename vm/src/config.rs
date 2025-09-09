// Sizes
pub const IND_SIZE: usize = 2;
pub const REG_SIZE: usize = 4;
pub const DIR_SIZE: usize = REG_SIZE;

// Parameter type codes
pub const REG_CODE: u8 = 1;
pub const DIR_CODE: u8 = 2;
pub const IND_CODE: u8 = 3;

// Game limits
pub const MAX_PLAYERS: usize = 4;
pub const MEM_SIZE: usize = 4 * 1024; // 4096
pub const IDX_MOD: i32 = (MEM_SIZE / 8) as i32; // 512
pub const PLAYER_MAX_SIZE: usize = MEM_SIZE / 6; // 682

// Parsing characters
pub const COMMENT_CHAR: char = '#';
pub const LABEL_CHAR: char = ':';
pub const DIRECT_CHAR: char = '%';
pub const SEPARATOR_CHAR: char = ',';
pub const LABEL_CHARS: &str = "abcdefghijklmnopqrstuvwxyz_0123456789";

// Command strings
pub const NAME_CMD_STRING: &str = ".name";
pub const DESCRIPTION_CMD_STRING: &str = ".description";

// Register and process configuration
pub const REG_NUMBER: usize = 16;

// Cycle configuration
pub const CYCLE_TO_DIE: i32 = 1536;
pub const CYCLE_DELTA: i32 = 50;
pub const NBR_LIVE: i32 = 21;
pub const MAX_CHECKS: i32 = 10;

// File format specifications
pub const PROG_NAME_LENGTH: usize = 128;
pub const DESCRIPTION_LENGTH: usize = 2048;
pub const COREWAR_EXEC_SIGNATURE: u32 = 0xea83f3;

// Derived constants
pub const ARENA_SIZE: usize = MEM_SIZE;
pub const MAX_PROGRAM_SIZE: usize = PLAYER_MAX_SIZE;