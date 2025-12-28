pub mod decode;
pub mod display;
//use vm::red;
use crate::arena::*;
use crate::config::REG_NUMBER;
use crate::counter::PC;
use crate::instructions::*;
use crate::*;

// process.rs
// https://www.geeksforgeeks.org/operating-systems/process-in-operating-system/
// https://www.geeksforgeeks.org/operating-systems/process-control-block-in-os/
// running, waiting, or ready to execute.
/*
• Running: In the running state, a process is running on a processor.
    This means it is executing instructions.
• Ready: In the ready state, a process is ready to run but for some
   reason the OS has chosen not to run it at this given moment.
• Blocked: In the blocked state, a process has performed some kind
    of operation that makes it not ready to run until some other event
    takes place. A common example: when a process initiates an I/O
    request to a disk, it becomes blocked and thus some other process
    can use the processor.

    // the registers xv6 will save and restore
// to stop and subsequently restart a process
struct context {
int eip;
int esp;
int ebx;
int ecx;
int edx;
int esi;
int edi;
int ebp;
};
// the different states a process can be in
enum proc_state { UNUSED, EMBRYO, SLEEPING,
RUNNABLE, RUNNING, ZOMBIE };
*/
#[derive(PartialEq)]
pub enum State {
    Waiting,
    Ready,
    NoInstruction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamType {
    None,
    Register,
    Direct,
    Indirect,
}

#[derive(Debug, Clone)]
pub struct LiveStatus {
    pub executed: bool,
    pub player_id: i32,  // negative of the player ID as per Core War convention
    pub nbr_live: usize, // used with "Stop process execution"
    pub last_live_cycle: usize,
}
/*
// the information xv6 tracks about each process
// including its register context and state
struct proc {
char *mem;
// Start of process memory
uint sz;
// Size of process memory
char *kstack;
// Bottom of kernel stack
// for this process
enum proc_state state;
// Process state
int pid;
// Process ID
struct proc *parent;
// Parent process
void *chan;
// If non-zero, sleeping on chan
int killed;
// If non-zero, have been killed
struct file *ofile[NOFILE]; // Open files
struct inode *cwd;
// Current directory
struct context context;
// Switch here to run process
struct trapframe *tf;
// Trap frame for the
// current interrupt
};*/
#[derive(Debug, Clone)]
pub struct Process {
    pub name: String,
    pub id: usize,
    pub player_id: i32,
    pub pc: PC, // Program Counter
    pub registers: [i32; REG_NUMBER],
    pub carry: bool,
    pub current_instruction: Option<Instruction>,
    pub instction_pc: usize,
    pub current_instruction_name: String,
    pub remaining_cycles: i32,
    pub live_status: LiveStatus,
}

impl Process {
    pub fn new(player_id: i32, id: usize, pc: usize, name: String) -> Self {
        let mut pro = Self {
            name: name,
            id: id,
            player_id: player_id * -1,
            pc: PC::new(pc),
            registers: [0; REG_NUMBER],
            remaining_cycles: 0,
            current_instruction: None,
            instction_pc: 0,
            current_instruction_name: "None".into(),
            carry: false,
            live_status: LiveStatus {
                executed: false,
                player_id: player_id * -1, // Initialize with the process's actual player_id
                nbr_live: 0,
                last_live_cycle: 0,
            },
        };
        pro.registers[0] = player_id;
        pro
    }

    pub fn state(&self) -> State {
        if self.current_instruction.is_some() && self.remaining_cycles == 0 {
            return State::Ready;
        } else if self.current_instruction.is_some() && self.remaining_cycles != 0 {
            return State::Waiting;
        } else {
            return State::NoInstruction;
        }
    }

    pub fn fetch_decode(&mut self, arena: &mut Arena, cycle: usize) {
        let opcode = arena.read(self.pc.get(), 1)[0];
        self.instction_pc = self.pc.get();
        self.pc.inc();
        if opcode >= 1 && opcode <= 16 {
            ////println!("address {} instruction : {:?}", self.pc.get(), opcode);
            let inst = self.decode(opcode, arena);
            if inst.is_some() {
                self.current_instruction_name = opcode_to_name(opcode);
            } else {
                self.current_instruction_name = "None".to_string();
            }
            self.current_instruction = inst;
        } else {
            eprintln!(
                "cycle {}: Opcode {} is not a valid instruction",
                cycle, opcode
            );
        }
    }
    //Opcode ->
    // https://corewar-docs.readthedocs.io/en/latest/redcode/opcodes/?
    // https://corewar-docs.readthedocs.io/en/latest/redcode/parser/
    // work on decoding an instruction
    // [Opcode] [Pcode?] [Param1] [Param2] [Param3]
    pub fn execute_cycle(
        &mut self,
        arena: &mut Arena,
        current_cycle: usize,
    ) -> instructions::VmAction /*Option<Process>*/ {
        let child: instructions::VmAction = instructions::VmAction::None;
        match self.state() {
            State::Waiting => {
                ////println!("waiting...");
                self.remaining_cycles -= 1;
            }
            State::Ready => {
                ////println!("executing...");
                ////println!("instruction {:?}", self.current_instruction);
                let current_inst = self.current_instruction.clone().take().unwrap();
                return current_inst.execute(self, arena, current_cycle);
            }
            State::NoInstruction => {
                //println!("should not try to execute an process with empty instruction");
                self.current_instruction_name = "None".to_string();
                // //println!("free...");
                // self.fetch_decode(arena);
            }
        }
        //thread::sleep(Duration::from_millis(60));
        return child;
    }
}

pub fn opcode_to_name(op: u8) -> String {
    let name = match op {
        1 => "live",
        2 => "ld",
        3 => "st",
        4 => "add",
        5 => "sub",
        6 => "and",
        7 => "or",
        8 => "xor",
        9 => "zjmp",
        10 => "ldi",
        11 => "sti",
        12 => "fork",
        13 => "lld",
        14 => "lldi",
        15 => "lfork",
        16 => "nop",
        _ => "invalid",
    };

    name.to_string()
}
