use std::fmt::Display;
use std::fmt::Formatter;
//use vm::*;
use crate::*;
#[derive(Clone)]
pub struct Player {
    pub id: i32, // The player id (assigned by the VM, starting from 1)
    pub name: String,
    pub comment: String,
    pub code: Vec<u8>,
    pub size: u32,
    //pub start_address: usize, // Where in the arena the player's code is loaded
}

impl Player {
    pub fn new(id: i32, name: String, comment: String, code: Vec<u8>, size: u32) -> Self {
        Self {
            id,
            name,
            comment,
            code,
            size,
        }
    }
}

impl Display for Player {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // make a table
        let mut table = Table::new();

        table.add_header("id");
        table.add_header("name");
        table.add_header("comment");
        table.add_header("size");
        table.add_header("code");

        // add rows
        let hex_string = self
            .code
            .iter()
            .map(|b| format!("{:02x}", b)) // format each byte as two-digit hex
            .collect::<Vec<String>>()
            .join(" ");
        table.add_row(&vec![
            self.id.to_string(),
            self.name.clone(),
            self.comment.clone(),
            self.size.to_string(),
            hex_string,
        ]);
        println!("{table}");
        Ok(())
    }
}
