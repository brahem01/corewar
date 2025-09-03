use std::{fs::File, io::{BufRead, BufReader}, path::Path};
use anyhow::Result;

use crate::instructions::{Instruction, ParamType, INSTRUCTIONS};
use super::lexer::{Token, tokenize};

#[derive(Debug)]
pub struct Param {
    pub param_type: ParamType,
    pub value: i32,
}

#[derive(Debug)]
pub struct InstructionInstance {
    pub instr: &'static Instruction,
    pub params: Vec<Param>,
}

pub fn parse_file(path: &Path) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let tokens = tokenize(&line);

        if tokens.is_empty() { continue; }

        match parse_tokens(&tokens) {
            Ok(inst) => println!("Parsed: {:?}", inst),
            Err(e) => eprintln!("Error line {}: {}", line_num + 1, e),
        }
    }
    Ok(())
}

// Convert tokens into InstructionInstance
fn parse_tokens(tokens: &[Token]) -> Result<InstructionInstance, String> {
    let mut iter = tokens.iter();
    let instr_token = iter.next().ok_or("Empty line")?;

    let instr_name = match instr_token {
        Token::Instr(name) => name,
        Token::LabelDef(_) => return Err("Label line, skipping".to_string()),
        _ => return Err("Expected instruction".to_string()),
    };

    // Lookup instruction in INSTRUCTIONS table
    let instr = INSTRUCTIONS.iter()
        .find(|i| i.name == instr_name.as_str())
        .ok_or(format!("Unknown instruction: {}", instr_name))?;

    let mut params = Vec::new();
    for token in iter {
        match token {
            Token::Register(r) => params.push(Param { param_type: ParamType::Register, value: *r as i32 }),
            Token::Direct(v) => params.push(Param { param_type: ParamType::Direct, value: *v }),
            Token::Indirect(v) => params.push(Param { param_type: ParamType::Indirect, value: *v }),
            Token::Comma => continue,
            _ => return Err(format!("Unexpected token: {:?}", token)),
        }
    }

    if params.len() != instr.nb_params as usize {
        return Err(format!("Expected {} params, got {}", instr.nb_params, params.len()));
    }

    Ok(InstructionInstance { instr, params })
}