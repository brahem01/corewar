//use vm::{blue, red};
use crate::arena::Arena;
use crate::config::{ CYCLE_DELTA, CYCLE_TO_DIE, MAX_CHECKS, NBR_LIVE };
use crate::instructions::VmAction;
use crate::player::Player;
use crate::process::Process;
use crate::*;
use std::collections::HashSet;
use std::sync::mpsc::Sender;
//use std::process as os;
/*
[X] create
[ ] destroy
[ ] wait
[ ] miscellaneous
[ ] control (suspend)
[ ] status
 */
pub struct VmSnapshot {
    pub cycle: usize,
    pub arena: Arena,
    pub processes: Vec<Process>,
    pub winners: HashSet<i32>,
    pub game_over: bool,
}

// vm.rs
pub struct VirtualMachine {
    pub arena: Arena,
    pub processes: Vec<Process>,
    pub winners: HashSet<i32>,
    pub cycle_count: usize,
    pub cycles_to_die: usize,
    pub players: Vec<Player>,
    nbr_checks: usize,
    cycles_since_check: usize,
    cycles_to_stop: Option<usize>,
    verbos: bool,
    sender: Option<Sender<VmSnapshot>>,
}

impl VirtualMachine {
    pub fn create(
        arena: Arena,
        processes: Vec<Process>,
        players: Vec<Player>,
        cycles_to_stop: Option<usize>,
        verbos: bool
    ) -> Self {
        Self {
            arena,
            processes,
            cycle_count: 1, // fix this
            cycles_to_die: CYCLE_TO_DIE,
            nbr_checks: 0,
            cycles_since_check: 0,
            winners: HashSet::new(),
            players: players,
            cycles_to_stop: cycles_to_stop,
            verbos,
            sender: None,
        }
    }
    pub fn set_sender(&mut self, sender: Sender<VmSnapshot>) {
        self.sender = Some(sender);
    }

    fn emit(&self) {
        let game_over =
            !self.processes_alive() ||
            (self.cycles_to_stop.is_some() && self.cycle_count >= self.cycles_to_stop.unwrap());
        if let Some(sender) = &self.sender {
            let snapshot = VmSnapshot {
                cycle: self.cycle_count,
                arena: self.arena.clone(),
                processes: self.processes.clone(),
                winners: self.winners.clone(),
                game_over,
            };
            let _ = sender.send(snapshot);
        }
    }

    pub fn load_player(&mut self, player: Player, i: usize) {
        self.arena.write(i, &player.code);
    }

    pub fn dump_arena(&self) {
        println!("\n{}", green("=== MEMORY DUMP ==="));

        let mut j = 0;
        for (i, chunk) in self.arena.memory.chunks(32).enumerate() {
            if j > 1 {
                break;
            }
            j += 1;
            // Address in hex (8 digits)
            print!("{:08x}  ", i * 32);

            // Hex bytes (32 per line)
            for byte in chunk {
                print!("{:02x} ", byte);
            }

            println!();
        }

        println!("{}", green("=== END DUMP ==="));
    }

    pub fn print_match_intro(&self, players: &[Player]) {
        println!("{}", green(&"=".repeat(80)));
        println!("{}", green("                           COREWAR MATCH"));
        println!("{}", green(&"=".repeat(80)));
        println!();
        println!("For this match the players will be:");

        for (i, player) in players.iter().enumerate() {
            println!(
                "Player {} ({} bytes): {} ({})",
                i + 1,
                player.code.len(),
                player.name,
                player.comment
            );
        }

        println!();
        println!("{}", green(&"=".repeat(80)));
        println!();
    }

    pub fn get_player(&self, id: i32) -> Option<String> {
        for player in &self.players {
            if player.id == id {
                return Some(player.name.clone());
            }
        }
        return None;
    }
    pub fn run(&mut self) {
        while self.processes_alive() {
            if let Some(cycles_to_stop) = self.cycles_to_stop && self.cycle_count >= cycles_to_stop {
                self.dump_arena();
                break;
            }
            for process in &mut self.processes {
                if process.state() == process::State::NoInstruction {
                    process.fetch_decode(&mut self.arena, self.cycle_count);
                }
            }
            //self.simple_debug();
            self.cycle();
            // this is for convinience to look exactly like the reference vm giving.
            // otherwise it is not important to do the printing before the cycle or after!
            // debugging lines goew here
            if self.verbos {
                self.debug1();
                self.debug2();
            }
            let before = self.cycles_to_die;
            let decreased = self.cycle_logic();
            if decreased {
                println!(
                    "cycle {}: Cycles to die decreased: {} -> {}",
                    self.cycle_count,
                    before,
                    self.cycles_to_die
                );
            }
            self.cycle_count += 1;
            if self.verbos {
                self.emit();
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        }

        if self.winners.len() != 1 {
            println!("cycle {}: Nobody wins!", self.cycle_count);
        } else {
            let winner = *self.winners.iter().next().unwrap();

            let name = match self.get_player(winner) {
                Some(name) => name,
                None => "___".into(),
            };
            println!(
                "cycle {}: The winner is player ({}): {}!",
                //winner.live_status.last_live_cycle,
                self.cycle_count,
                winner * -1,
                name
            );
        }
        if self.verbos {
            self.emit();
        }
    }
    // fn simple_debug(&self, process: &mut Process, current_cyle: usize) {
    // }
    pub fn cycle(&mut self) {
        let mut new_processes = Vec::new();
        for process in &mut self.processes {
            let action = process.execute_cycle(&mut self.arena, self.cycle_count);
            match action {
                VmAction::Fork { new_pc, use_idx } => {
                    let mut new_process = process::Process::new(
                        process.player_id,
                        process.id,
                        process.pc.get()
                    );
                    new_process.pc.set(new_pc as usize, use_idx);
                    new_process.current_instruction = None;
                    new_processes.push(new_process);
                }
                VmAction::Live(id) => {
                    match get_playername(self.players.clone(), id) {
                        Some(name) => {
                            process.live_status.executed = true;
                            process.live_status.nbr_live += 1;
                            process.live_status.last_live_cycle = self.cycle_count;
                            process.live_status.player_id = id;
                            println!(
                                "cycle {}: Player {} {} is alive",
                                self.cycle_count,
                                id * -1,
                                name
                            );
                            process.current_instruction = None;
                        }
                        None => {
                            println!("cycle {}: live: Invalid argument: {}", self.cycle_count, id);
                        }
                    };
                }
                _ => {}
            }
        }
        // Append all new processes at once after the loop
        self.processes.extend(new_processes);
    }
    pub fn cycle_logic(&mut self) -> bool {
        let mut decreased = false;

        self.cycles_since_check += 1;

        if self.cycles_since_check == self.cycles_to_die {
            self.check_lives();
        }
        if self.cycles_since_check > self.cycles_to_die {
            self.cycles_since_check = 0;

            let nbr_lives = self.read_nbr_lives();

            // MAX_CHECKS logic
            if nbr_lives < NBR_LIVE {
                self.nbr_checks += 1;
            }

            // CTD decrease conditions
            if nbr_lives >= NBR_LIVE || self.nbr_checks > MAX_CHECKS {
                self.cycles_to_die = self.cycles_to_die.saturating_sub(CYCLE_DELTA);
                self.nbr_checks = 0;
                decreased = true;
            }

            self.rest_nbr_lives();
        }

        if self.cycles_to_die == 0 {
            self.check_lives();
        }
        decreased
    }

    fn debug1(&self) {
        println!(
            "{} ",
            green(
                "------------------------------------------------------------------------------------"
            )
        );
        println!(
            "Cycle {} || Cycles before life check: {} || Cycles between checks: {}",
            self.cycle_count,
            self.cycles_to_die - self.cycles_since_check,
            self.cycles_to_die
        );

        println!("Processes:");
        println!("Id |Player Id |Pc   |Carry |Instr  |Wait |Registers");
        for p in self.processes.iter() {
            let current_instruction_name: String = if p.state() == process::State::NoInstruction {
                "___".to_string()
            } else {
                p.current_instruction_name.clone()
            };
            print!(
                "{:>2} |{:>9} |{:>4} |{:5} |{:<6} |{:>4} | ",
                p.id,
                &p.player_id.to_string(),
                &p.instction_pc.to_string(),
                &p.carry.to_string(),
                current_instruction_name,
                &p.remaining_cycles.to_string()
            );

            // Registers //print
            for (i, reg) in p.registers.iter().enumerate() {
                print!("{}:{:x}  ", i + 1, reg);
            }
            println!();
        }
    }
    fn debug2(&self) {
        println!("Players:");
        println!("Id |Last Live |Nb Live since last check");
        for pl in self.processes.iter() {
            println!(
                "{:>2} |{:>9} |{:>3}",
                pl.live_status.player_id,
                pl.live_status.last_live_cycle,
                pl.live_status.nbr_live
            );
        }

        println!("Arena:");
        let mut count = 0;
        for (i, byte) in self.arena.memory.iter().enumerate() {
            if i % 32 == 0 {
                print!("{:08x}  ", i);
            }
            print!("{:02x} ", byte);
            if i % 32 == 31 {
                println!("");
            }

            if count == 31 {
                break;
            }
            count += 1;
        }
        println!();
    }
    fn read_nbr_lives(&mut self) -> usize {
        let mut count = 0;
        for process in &mut self.processes {
            count += process.live_status.nbr_live;
        }
        count
    }

    fn rest_nbr_lives(&mut self) {
        for process in &mut self.processes {
            process.live_status.nbr_live = 0;
        }
    }
    pub fn processes_alive(&self) -> bool {
        self.processes.len() > 0
    }

    fn check_lives(&mut self) {
        let mut winners = HashSet::new();
        for process in &self.processes {
            if self.get_player(process.live_status.player_id).is_some() {
                winners.insert(process.live_status.player_id);
            }
        }
        self.winners = winners;
        self.processes.retain(|process| process.live_status.executed);
        for process in &mut self.processes {
            process.live_status.executed = false;
        }
    }
}

fn get_playername(players: Vec<Player>, id: i32) -> Option<String> {
    for player in players {
        if player.id == id {
            return Some(player.name.clone());
        }
    }
    return None;
}
