use std::env;
mod memory;
mod executer;
mod parser;
mod warrior;
mod gamestate;
mod config;
mod process;use anyhow::{anyhow, Result};


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("USAGE: assembler [arguments..]\nyou should atleast enter one argument.");
    }
    let mut warriors_data = Vec::new();
    let mut dump_cycles: Option<i32> = None;
    let mut cursor = 1;
    while cursor < args.len() {
        if args[cursor] == "-d" && cursor + 1 < args.len() {
            if let Ok(cycles) = args[cursor + 1].parse::<i32>() {
                dump_cycles = Some(cycles);
                cursor+=2;
            }else {
                panic!("the argument after -d should be number");
            }
        }
        let d = parser::parse_file(&args[cursor])?;
        warriors_data.push(d);
        cursor+=1;
    }
    let mut arena = memory::Arena::new();
    let warriors = arena.setup_warriors(warriors_data)?;
    println!("the arena is: {:?}", arena);
    let mut executer = executer::Executer::new(arena, warriors, dump_cycles);
    executer.execute_cycle();
    Ok(())
}
