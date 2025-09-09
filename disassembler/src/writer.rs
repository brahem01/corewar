use std::fs::File;
use std::io::Write;
use anyhow::Result;
use crate::{Disassembler};

pub fn write_s_file(dis: &Disassembler, path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, ".Name: {}", dis.header.name)?;
    writeln!(file, ".Comment: {}", dis.header.comment)?;
    writeln!(file)?;

    for inst in &dis.instructions {
        let mut line = inst.instr.name.to_string();
        let params_str: Vec<String> = inst.params.iter().enumerate().map(|(i, p)| {
            match inst.instr.params[i][0] {
                shared::instructions::ParamType::Register => format!("r{}", p),
                shared::instructions::ParamType::Direct => format!("%{}", p),
                shared::instructions::ParamType::Indirect => format!("{}", p),
            }
        }).collect();
        line.push_str(" ");
        line.push_str(&params_str.join(", "));
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
