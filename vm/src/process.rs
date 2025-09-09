#[derive(Clone)]
pub struct Process {
    pub pc: usize,
    pub carry: bool,
    pub registers: [i32; 16],
    pub alive: bool,
    pub executed_live: bool,
}

//later we should add the processes to the warrior type
impl Process {
    pub fn new(start_pc: usize, player_id: u8) -> Self {
        let mut registers = [0; 16];
        registers[0] = -(player_id as i32);
        
        Process {
            pc: start_pc,
            carry: false,
            registers,
            alive: true,
            executed_live: false,
        }
    }
}