pub struct Cpu {
      Registers: [i32; 16],
      Carry: bool,
      Cycle: u8,
}

impl Cpu {
      pub fn new() -> Self {
            Self{
                  Registers: [0; 16],
                  Carry: false,
                  Cycle: 0,
            }
      }
}