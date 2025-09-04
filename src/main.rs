mod assembler;
mod vm;
mod instructions;
mod utils;
mod disassembler;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file.s>", args[0]);
        std::process::exit(1);
    }
    
    let binary_code = assembler::run_file(&args[1]);
    if let Ok(b) = binary_code {
        utils::write_cor_file("player.cor", b)?
    }

    let dis = disassembler::Disassembler::read_cor_file("player.cor")?;
    disassembler::writer::write_s_file(&dis, "player_disassembled.s")?;
    println!("Disassembled file written!");
    //write into file;
    Ok(())
}
