#![allow(dead_code, unused_imports)]

use vm::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Live,
    Ld,
    St,
    Add,
    Sub,
    And,
    Or,
    Xor,
    Zjmp,
    Ldi,
    Sti,
    Fork,
    Lld,
    Lldi,
    Lfork,
    Nop,
}
impl Instruction {
    pub fn cycles(self) -> usize {
        match self {
            Instruction::Live => 10,
            Instruction::Ld => 5,
            Instruction::St => 5,
            Instruction::Add => 10,
            Instruction::Sub => 10,
            Instruction::And => 6,
            Instruction::Or => 6,
            Instruction::Xor => 6,
            Instruction::Zjmp => 20,
            Instruction::Ldi => 25,
            Instruction::Sti => 25,
            Instruction::Fork => 800,
            Instruction::Lld => 10,
            Instruction::Lldi => 50,
            Instruction::Lfork => 1000,
            Instruction::Nop => 2,
        }
    }
}

#[track_caller]
pub fn run_inst(vm: &mut VirtualMachine, inst: Instruction) {
    // zjmp 20
    let n_cycles = inst.cycles();
    for _ in 0..n_cycles {
        for process in &mut vm.processes {
            if process.state() == State::NoInstruction {
                process.fetch_decode(&mut vm.arena, vm.cycle_count);
            }
        }
        vm.cycle();
    }
}
#[track_caller]
pub fn build_vm(file_name: &str) -> VirtualMachine {
    let path = format!("playground/players_src/{file_name}.cor");

    let args = vec!["vm".to_string(), path];

    let (players, _, _) = parse_arguments(args).expect("parse failed");
    let player = players[0].clone();

    let arena = Arena::new();
    let process = Process::new(player.id, 0, 0);

    //println!("{player}");
    ////println!("{}", process);
    //////println!("{}", arena);

    let mut vm = VirtualMachine::create(arena.clone(), vec![process], players.clone(), None, false);

    vm.load_player(player, 0);
    vm
}
#[track_caller]
pub fn build_vm_more(file_names: Vec<&str>) -> VirtualMachine {
    // Prepare arguments for all files
    let args: Vec<String> = std::iter::once("vm".to_string())
        .chain(
            file_names
                .iter()
                .map(|file_name| format!("playground/players_src/{file_name}.cor")),
        )
        .collect();

    // Parse players from arguments
    let (players, _, _) = parse_arguments(args).expect("parse failed");
    let players_count = players.len();
    // Create arena
    let arena = Arena::new();

    // Create a process for each player, keeping their index
    let processes: Vec<Process> = players
        .iter()
        .enumerate()
        .map(|(i, player)| {
            //println!("Loading player {} at index {}", player.name, i);
            Process::new(player.id, i, MEM_SIZE / players_count * i)
        })
        .collect();

    // Create the VM with arena and processes
    let mut vm = VirtualMachine::create(arena.clone(), processes, players.clone(), None, false);

    // Load each player into the arena
    for (i, player) in players.iter().enumerate() {
        vm.load_player(player.clone(), MEM_SIZE / players_count * i);
    }

    vm
}

#[track_caller]
pub fn does_reg(vm: &VirtualMachine, n: usize, has: i32) {
    assert_eq!(vm.processes[0].registers[n - 1], has);
}
