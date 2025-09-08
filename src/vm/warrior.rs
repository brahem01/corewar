use crate::vm::config::*;


#[derive(Clone)]
pub struct Warrior{
      pub id: u8,
      pub name: String,
      pub comment: String,
      pub registers: [i32; REG_NUMBER],
      pub pc: usize,
      pub cycles: u8,
      pub carry: bool,
      pub alive: bool,
}

impl Warrior{
      pub fn new(name: String, comment: String) -> Warrior {
            Warrior{
                  id: 0,
                  name,
                  comment,
                  registers: [0; REG_NUMBER],
                  pc: 0,
                  cycles: 0,
                  carry: false,
                  alive: true,
            }
      }
}