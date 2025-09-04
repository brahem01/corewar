use anyhow::{anyhow, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use super::lexer::{tokenize, Token};
use crate::instructions::{Instruction, ParamType, INSTRUCTIONS};

#[derive(Debug)]
pub struct Param {
    pub param_type: ParamType,
    pub value: i32,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub comment: String,
    pub instructions: Vec<InstructionInstance>,
}

impl Player {
    fn new() -> Self {
        Player {
            name: String::new(),
            comment: String::new(),
            instructions: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct InstructionInstance {
    pub instr: &'static Instruction,
    pub params: Vec<Param>,
}

pub fn parse_file(path: &Path) -> Result<Player> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut player = Player::new();
    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        let line_trimmed = line.trim();
        if line_trimmed.is_empty() || line_trimmed.starts_with(';') {
            continue;
        }
        // Capture .name directive
        if line_trimmed.starts_with(".name") {
            if let Some(start) = line_trimmed.find('"') {
                if let Some(end) = line_trimmed[start + 1..].find('"') {
                    if player.name.is_empty() {
                        return Err(anyhow!(format!(
                            "Error line {}: multiple name declaration",
                            line_num + 1
                        )));
                    }
                    player.name = line_trimmed[start + 1..start + 1 + end].to_string();
                    continue;
                }
            }
            return Err(anyhow!(
                "Error line {}: Invalid .name directive",
                line_num + 1
            ));
        }

        // Capture .comment directive
        if line_trimmed.starts_with(".comment") {
            if let Some(start) = line_trimmed.find('"') {
                if let Some(end) = line_trimmed[start + 1..].find('"') {
                    if player.comment.is_empty() {
                        return Err(anyhow!(format!(
                            "Error line {}: multiple comment declaration",
                            line_num + 1
                        )));
                    }
                    player.comment = line_trimmed[start + 1..start + 1 + end].to_string();
                    continue;
                }
            }
            return Err(anyhow!(
                "Error line {}: Invalid .comment directive",
                line_num + 1
            ));
        }
        let tokens = tokenize(&line);

        match parse_tokens(&tokens) {
            Ok(inst) => player.instructions.push(inst),
            Err(e) => return Err(anyhow!(format!("Error line {}: {}", line_num + 1, e))),
        }
    }
    Ok(player)
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
    let instr = INSTRUCTIONS
        .iter()
        .find(|i| i.name == instr_name.as_str())
        .ok_or(format!("Unknown instruction: {}", instr_name))?;

    let mut params = Vec::new();
    for token in iter {
        match token {
            Token::Register(r) => params.push(Param {
                param_type: ParamType::Register,
                value: *r as i32,
            }),
            Token::Direct(v) => params.push(Param {
                param_type: ParamType::Direct,
                value: *v,
            }),
            Token::Indirect(v) => params.push(Param {
                param_type: ParamType::Indirect,
                value: *v,
            }),
            Token::Comma => continue,
            _ => return Err(format!("Unexpected token: {:?}", token)),
        }
    }

    if params.len() != instr.nb_params as usize {
        return Err(format!(
            "Expected {} params, got {}",
            instr.nb_params,
            params.len()
        ));
    }

    Ok(InstructionInstance { instr, params })
}
