use std::env;
use assembler::run_file;
use shared::file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("USAGE: assembler [arguments..]\nyou should atleast enter one argument.");
        return;
    }
    for arg in &args[1..] {
        match file::validate_assembly_file(arg) {
            Ok(path) => {
                match run_file(arg) {
                    Ok(bin) => {
                        match file::write_binary_to_file(bin, &path) {
                            Ok(_) => println!("the assembly file: {} converted successfully to binary file: {}", arg, &path),
                            Err(e) => eprintln!("{}", e),
                        }
                    },
                    Err(e) => eprintln!("{}", e),
                }
            },
            Err(e) => eprintln!("{}", e),
        }
        
    }
}
