use crate::config::MEM_SIZE;
use std::fmt::{Display, Formatter, Result};
//use vm::*;
use crate::Table;
// arena.rs
#[derive(Debug, Clone)]
pub struct Arena {
    pub memory: [u8; 4096],
}

impl Arena {
    pub fn new() -> Self {
        Self { memory: [0; 4096] }
    }
    pub fn write(&mut self, pos: usize, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.memory[(pos + i) % 4096] = byte;
        }
    }
    pub fn read(&self, pos: usize, size: usize) -> Vec<u8> {
        let mut arr = Vec::with_capacity(size);
        let mut current_pos = pos % MEM_SIZE;
        for _ in 0..size {
            arr.push(self.memory[current_pos]);
            current_pos = (current_pos + 1) % MEM_SIZE; // proper circular wrap
        }
        arr
    }
}

// configurable display width
const BYTES_PER_ROW: usize = 16; // change to 16, 32, or 64 as you like

impl Display for Arena {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        let mut table = Table::new();

        // header
        table.add_header("Addr");
        for i in 0..BYTES_PER_ROW {
            table.add_header(&format!("{:02X}", i));
        }

        // rows
        let mut row = Vec::new();
        //for chunk_start in (0..MEM_SIZE).step_by(BYTES_PER_ROW) {
        for chunk_start in (0..64).step_by(BYTES_PER_ROW) {
            row.clear();
            row.push(format!("{:04X}", chunk_start));
            for offset in 0..BYTES_PER_ROW {
                let idx = (chunk_start + offset) % MEM_SIZE;
                row.push(format!("{:02X}", self.memory[idx]));
            }
            table.add_row(&row);
        }

        //////println!("{table}");
        Ok(())
    }
}
