mod assembler;
mod vm;
mod instructions;
mod utils;
mod disassembler;
use dotenv::dotenv;

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    //get the assembly files, and check they are between 2 and 4
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file.s>", args[0]);
        std::process::exit(1);
    }
    //loop and execute the assembler for every file
    let binary_code = assembler::run_file(&args[1]);
    if let Ok(b) = binary_code {
        utils::write_cor_file("player.cor", b)?
    }
    
    let dis = disassembler::Disassembler::read_cor_file("player.cor")?;
    disassembler::writer::write_s_file(&dis, "player_disassembled.s")?;
    println!("Disassembled file written!");
    //initialize the vertual machine with those files;
    Ok(())
}
