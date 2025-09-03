mod assembler;
mod vm;
mod instructions;
// mod utils;


fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.s>", args[0]);
        std::process::exit(1);
    }
    
    assembler::run_file(&args[1])?;
    Ok(())
}
