use std::env;
use shared::file;
use disassembler::{Disassembler, writer::write_s_file};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("USAGE: assembler [arguments..]\nyou should atleast enter one argument.");
        return;
    }
    for arg in &args[1..] {
        match file::validate_core_file(arg) {
            Ok(path) => {
                match Disassembler::read_cor_file(arg) {
                    Ok(dis) => {
                        match write_s_file(&dis, &path) {
                            Ok(_) => println!("the binary file: {} converted successfuly to the assembly file: {}", arg, path),
                            Err(e) => eprintln!("error writing the file: {}", e),
                        }
                    },
                    Err(e) => eprintln!("{}", e),
                }
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
