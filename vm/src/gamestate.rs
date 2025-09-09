use super::config::*;

pub struct GameState {
    pub cycle: i32,
    pub cycles_to_die: i32,
    pub checks_since_last_decrement: i32,
    pub live_count_since_last_check: i32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            cycle: 0,
            cycles_to_die: CYCLE_TO_DIE,
            checks_since_last_decrement: 0,
            live_count_since_last_check: 0,
        }
    }

    pub fn should_check_lives(&self) -> bool {
        self.cycle % self.cycles_to_die == 0
    }

    pub fn update_cycles_to_die(&mut self) {
        self.checks_since_last_decrement += 1;
        
        if self.live_count_since_last_check >= NBR_LIVE {
            self.cycles_to_die -= CYCLE_DELTA;
            self.checks_since_last_decrement = 0;
        } else if self.checks_since_last_decrement >= MAX_CHECKS {
            self.cycles_to_die -= CYCLE_DELTA;
            self.checks_since_last_decrement = 0;
        }
        
        self.live_count_since_last_check = 0;
    }
}