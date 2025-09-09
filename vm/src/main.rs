use std::env;
mod memory;
mod executer;
mod parser;
mod warrior;
mod config;
mod process;use anyhow::{anyhow, Result};


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("USAGE: assembler [arguments..]\nyou should atleast enter one argument.");
        return Err(anyhow!("jd"));
    }
    let mut warriors_data = Vec::new();
    for arg in &args[1..] {
        let d = parser::parse_file(arg)?;
        warriors_data.push(d);
    }
    let mut arena = memory::Arena::new();
    let warriors = arena.setup_warriors(warriors_data)?;
    println!("the arena is: {:?}", arena);
    let mut executer = executer::Executer::new(arena, warriors);
    executer.execute_cycle();
    Ok(())
}
