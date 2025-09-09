use crate::{config::*, process::Process};

#[derive(Clone)]
pub struct Warrior {
    pub id: u8,
    pub name: String,
    pub comment: String,
    pub processes: Vec<Process>,
    pub cycles: u8,
    pub alive: bool,
}

impl Warrior {
    pub fn new(name: String, comment: String) -> Warrior {
        Warrior {
            id: 0,
            name,
            comment,
            cycles: 0,
            processes: Vec::new(),
            alive: true,
        }
    }
    pub fn is_alive(&self) -> bool {
        self.processes.iter().any(|p| p.alive)
    }

    pub fn reset_live_flags(&mut self) {
        for process in &mut self.processes {
            process.executed_live = false;
        }
    }
}
