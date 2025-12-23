use crate::config::MEM_SIZE;

use crate::arena::*;
use crate::config::IDX_MOD;
use crate::instructions::Parameter;
use crate::process::*;

// [ ]  account for the case of negative indirect
pub fn read_indirect(_process: &mut Process, arena: &mut Arena, opcode_addr: usize, at: i32) -> i32 {
    let read_from = wrap_address(opcode_addr, at as i16);
    //println!("reading value from address: {}", read_from);
    bytes_to_i32(&arena.read(read_from, 2).clone())
}

pub fn get_value(p: &Parameter, process: &Process, arena: &Arena, apply_idx_mod: bool) -> i32 {
    match p {
        // ----------------------------
        // 1) REGISTER
        // ----------------------------
        Parameter::Register(reg) => process.registers[*reg - 1] as i32,

        // ----------------------------
        // 2) DIRECT
        // ----------------------------
        Parameter::Direct(val) => *val,

        // ----------------------------
        // 3) INDIRECT
        // ----------------------------
        Parameter::Indirect(offset) => {
            //println!("reading indirect");
            let mut off = *offset;

            // apply IDX_MOD if instruction requires it
            if apply_idx_mod {
                off %= IDX_MOD as i32;
            }

            // match what read_indirect does:
            let addr = wrap_address(process.instction_pc, off as i16);
            //println!("reading at address {}", addr);
            // arena.read() returns &[u8] of length 4
            let bytes = arena.read(addr, 2);
            //println!("bytes read {:?}", bytes);
            bytes_to_i32(&bytes)
        }

        _ => panic!("none parameter"),
    }
}

pub fn bytes_to_i16(bytes: &[u8]) -> i16 {
    let mut arr = [0u8; 2]; // 2 bytes for i16
    let len = bytes.len();
    arr[2 - len..].copy_from_slice(bytes);
    i16::from_be_bytes(arr)
}
// pub fn bytes_to_i32(bytes: &[u8]) -> i32 {
//     let mut arr = [0u8; 4]; // 4 bytes for i32
//     let len = bytes.len();
//     // copy bytes to the end of the array (big-endian)
//     arr[4 - len..].copy_from_slice(bytes);
//     i32::from_be_bytes(arr)
// }

pub fn bytes_to_i32(bytes: &[u8]) -> i32 {
    match bytes.len() {
        2 => i16::from_be_bytes([bytes[0], bytes[1]]) as i32, // sign-extend
        4 => i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        _ => panic!("Unsupported byte length for bytes_to_i32"),
    }
}

pub fn wrap_address(pc: usize, offset: i16) -> usize {
    let mut addr = (pc as isize + offset as isize) % MEM_SIZE as isize;
    if addr < 0 {
        addr += MEM_SIZE as isize;
    }
    addr as usize
}
