use crate::config::{IDX_MOD, MEM_SIZE};

#[derive(Debug, Clone)]
pub struct PC {
    pub addr: usize,
}

impl PC {
    pub fn new(pc: usize) -> Self {
        Self { addr: pc }
    }

    pub fn inc(&mut self) {
        self.addr = (self.addr + 1) % MEM_SIZE;
    }

    pub fn _reset(&mut self) {
        self.addr = 0;
    }

    pub fn add(&mut self, size: usize) {
        self.set(self.get() + size, false)
    }

    pub fn set(&mut self, new_addr: usize, use_idx_mod: bool) {
        let addr = if use_idx_mod {
            new_addr % IDX_MOD
        } else {
            new_addr
        };
        self.addr = addr % MEM_SIZE; // always wrap around arena
    }

    pub fn relative_jump(&mut self, offset: i32, use_idx_mod: bool) {
        let offset = if use_idx_mod {
            offset % IDX_MOD as i32
        } else {
            offset
        };

        let mut new_pc = self.get() as i32 + offset;

        new_pc %= MEM_SIZE as i32;

        if new_pc < 0 {
            new_pc += MEM_SIZE as i32;
        }
        self.addr = new_pc as usize;
    }

    pub fn get(&self) -> usize {
        return self.addr;
    }
}

// Unit tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_relative_jump() {
        // no idx mod
        // inc
        let mut pc = PC::new(0);
        pc.relative_jump(1, false);
        assert_eq!(pc.get(), 1);
        pc.relative_jump(1, false);
        assert_eq!(pc.get(), 2);
        pc.relative_jump(1, true);
        assert_eq!(pc.get(), 3);

        // negative
        pc.relative_jump(0, true);
        assert_eq!(pc.get(), 3);
        pc.relative_jump(-1, true);
        assert_eq!(pc.get(), 2);
        pc.relative_jump(-1, true);
        assert_eq!(pc.get(), 1);
        pc.relative_jump(-1, true);
        assert_eq!(pc.get(), 0);
        pc.relative_jump(-1, true);
        assert_eq!(pc.get(), MEM_SIZE - 1);
        pc.relative_jump(1, true);
        assert_eq!(pc.get(), 0);
        // use idx mod
        pc._reset();
        pc.relative_jump(1000, true);
        assert_eq!(pc.get(), 1000 % IDX_MOD);
        pc._reset();
        pc.relative_jump(-1000, true);
        assert_eq!(pc.get(), MEM_SIZE - 1000 % IDX_MOD);
    }

    #[test]
    fn test_counter_set() {
        // no idx mod
        let mut pc = PC::new(0);
        pc.set(1, false);
        assert_eq!(pc.get(), 1);
        pc.set(MEM_SIZE - 1, false);
        assert_eq!(pc.get(), MEM_SIZE - 1);
        // with idx mod
        pc._reset();
        pc.set(1000, true);
        let should_be = 1000 % IDX_MOD;
        assert_eq!(pc.get(), should_be);
    }

    #[test]
    fn test_counter_reset() {
        let mut pc = PC::new(0);
        pc.add(1);
        pc._reset();
        assert_eq!(pc.get(), 0);
    }

    #[test]
    fn test_counter_inc() {
        let mut pc = PC::new(0);
        pc.inc();
        assert_eq!(pc.get(), 1);

        pc.add(MEM_SIZE - 2);
        assert_eq!(pc.get(), MEM_SIZE - 1);

        pc.inc();
        assert_eq!(pc.get(), 0);

        pc.inc();
        pc.inc();
        assert_eq!(pc.get(), 2);
    }

    #[test]
    fn test_counter_add() {
        let mut pc = PC::new(0);
        pc.add(1);
        assert_eq!(pc.get(), 1);
        pc.add(MEM_SIZE);
        assert_eq!(pc.get(), 1);
        pc.add(0);
        assert_eq!(pc.get(), 1);
    }

    #[test]
    fn test_couter_initialization() {
        let pc = PC::new(0);
        assert_eq!(pc.get(), 0);
    }
}
