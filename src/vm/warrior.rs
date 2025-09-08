#[derive(Clone)]
pub struct Warrior{
      pub id: u8,
      pub name: String,
      pub comment: String,
      pub registers: [i32; 16],
      pub pc: usize,
      pub cycles: u8,
      pub cary: bool,
      pub alive: bool,
}

impl Warrior{
      pub fn new(name: String, comment: String) -> Warrior {
            Warrior{
                  id: 0,
                  name,
                  comment,
                  registers: [0; 16],
                  pc: 0,
                  cycles: 0,
                  cary: false,
                  alive: true,
            }
      }
}