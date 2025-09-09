use crate::process::Process;
use crate::*;
use shared::instructions::*;

use crate::gamestate::GameState;
use crate::memory::{Arena, ExecutableInstruction};
use crate::warrior::Warrior;

pub struct Executer {
    pub arena: memory::Arena,
    pub warriors: Vec<warrior::Warrior>,
    pub game_state: GameState,
    pub dump_cycles: Option<i32>,
}

impl Executer {
    pub fn new(
        arena: memory::Arena,
        warriors: Vec<warrior::Warrior>,
        dump_cycles: Option<i32>,
    ) -> Self {
        Self {
            arena,
            warriors,
            game_state: GameState::new(),
            dump_cycles,
        }
    }

    pub fn run(&mut self) {
        self.print_intro();

        while self.has_active_processes() && !self.should_stop() {
            self.execute_cycle();

            if self.game_state.should_check_lives() {
                // self.check_lives();
            }

            self.game_state.cycle += 1;
        }

        self.print_winner()
    }

    pub fn execute_cycle(&mut self) {
        for warrior in &mut self.warriors {
            if !warrior.alive {
                continue;
            }

            for i in 0..warrior.processes.len() {
                            // 1) Fetch instruction at warrior.pc
            if let Some(instr) = &self.arena.memory[warrior.processes[i].pc % 4096] {
                // 2) Execute instruction
                // execute(&mut self.arena, instr);

                // 3) Advance PC
                warrior.processes[i].pc = (warrior.processes[i].pc + instr.size_bytes) % 4096;
            } else {
                // No instruction? Just advance
                warrior.processes[i].pc = (warrior.processes[i].pc + 1) % 4096;
            }
            }
        }
    }

    fn execute_opcode(
        &mut self,
        warrior_id: usize,
        instr: &ExecutableInstruction,
    ) -> Option<Process> {
        match shared::instructions::Opcode::from(instr.instruction.opcode) {
            Opcode::Live => live(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
            Opcode::Ld => ld(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
            Opcode::St => st(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
            Opcode::Add => add(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
            Opcode::Sub => sub(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
            Opcode::And => and(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
            Opcode::Or => or(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
            Opcode::Xor => xor(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
            Opcode::Zjmp => zjmp(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
            Opcode::Ldi => ldi(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
            Opcode::Sti => sti(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
            Opcode::Fork => {
                return fork(&mut self.arena, &mut self.warriors[warrior_id - 1], instr);
            }
            Opcode::Lld => lld(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
            Opcode::Lldi => lldi(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
            Opcode::Lfork => lfork(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
            Opcode::Nop => nop(&mut self.arena, &mut self.warriors[warrior_id - 1], instr),
        };
        None
    }
    fn has_active_processes(&self) -> bool {
        self.warriors.iter().any(|warrior| warrior.is_alive())
    }

    fn should_stop(&self) -> bool {
        self.game_state.cycles_to_die <= 0 || self.warriors.len() <= 1
    }

    fn print_intro(&self) {
        println!("For this match the players will be:");
        for warrior in &self.warriors {
            println!("Player {} ( bytes): {} ({})", 
                warrior.id, 
                // warrior.processes[0].registers.len() * 4, // Approximate size
                warrior.name, 
                warrior.comment
            );
        }
    }

    fn print_winner(&self)  {
        if self.warriors.len() == 1 {
            let winner = &self.warriors[0];
            println!("cycle {}: The winner is player {}: {}!", 
                self.game_state.cycle, winner.id, winner.name);
        } else {
            println!("cycle {}: Nobody wins!", self.game_state.cycle);
        }
    }

}

pub fn live(_arena: &mut Arena, warrior: &mut Warrior, _instr: &ExecutableInstruction) {
    warrior.alive = true;
    println!("Warrior {} says LIVE", warrior.name);
}

pub fn ld(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    todo!()
}

pub fn st(arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    todo!()
}

pub fn add(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    todo!()
}

pub fn sub(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    todo!()
}

pub fn and(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    todo!()
}

pub fn or(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    todo!()
}

pub fn xor(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    todo!()
}

pub fn zjmp(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    todo!()
}

pub fn ldi(arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    // Load value from arena indirectly
    todo!()
}

pub fn sti(arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    todo!()
}

pub fn fork(
    _arena: &mut Arena,
    _warrior: &mut Warrior,
    _instr: &ExecutableInstruction,
) -> Option<Process> {
    // TODO: spawn new process
    None
}

pub fn lld(_arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    todo!()
}

pub fn lldi(arena: &mut Arena, warrior: &mut Warrior, instr: &ExecutableInstruction) {
    todo!()
}

pub fn lfork(_arena: &mut Arena, _warrior: &mut Warrior, _instr: &ExecutableInstruction) {
    // TODO: spawn new process far away
}

pub fn nop(_arena: &mut Arena, _warrior: &mut Warrior, _instr: &ExecutableInstruction) {
    // Do nothing
}
