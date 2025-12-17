use anyhow::{Result, anyhow};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use super::{
    lexer::{Token, tokenize},
    instruction::{InstructionInstance, Param, ValueType},
};
use shared::instructions::{INSTRUCTIONS, ParamType};


#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub comment: String,
    pub instructions: Vec<InstructionInstance>,
    pub labels: HashMap<String, usize>,
}

impl Player {
    fn new() -> Self {
        Player {
            name: String::new(),
            comment: String::new(),
            instructions: Vec::new(),
            labels: HashMap::new(),
        }
    }
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
                    if !player.name.is_empty() {
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
        if line_trimmed.starts_with(".description") {
            if let Some(start) = line_trimmed.find('"') {
                if let Some(end) = line_trimmed[start + 1..].find('"') {
                    if !player.comment.is_empty() {
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

        let tokens = tokenize(&line)?;
        match parse_tokens(&tokens) {
            Ok(instr) => {
                player.instructions.push(instr);
            },
            Err(e) => return Err(anyhow!(format!("Error line {}: {}", line_num + 1, e))),
        }
    }
    Ok(player)
}

// Convert tokens into InstructionInstance
// here should the return should be a inst or labelDef(String) so we need a general type contains the both
fn parse_tokens(tokens: &[Token]) -> Result<InstructionInstance, String> {
    let mut iter = tokens.iter();
    let mut token = iter.next().ok_or("Empty line")?;
    let mut instruction_instance = InstructionInstance::new();
    if let Token::LabelDef(lbl) = token {
        instruction_instance.label = Some(lbl.clone());
        let option_token = iter.next();
        match option_token {
            Some(tkn) => token = tkn,
            None=> return  Ok(instruction_instance)
        }
    }
    let instr_name = match token {
        Token::Instr(name) => name,
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
                value: ValueType::value(*r as i32),
            }),
            Token::Direct(v) => params.push(Param {
                param_type: ParamType::Direct,
                value: ValueType::value(*v),
            }),
            Token::Indirect(v) => params.push(Param {
                param_type: ParamType::Indirect,
                value: ValueType::value(*v),
            }),
            Token::LabelRef(v) => params.push(Param {
                param_type: ParamType::Direct,
                value: ValueType::label(v.clone()),
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
    instruction_instance.instr = Some(instr);
    instruction_instance.params = params;
    Ok(instruction_instance)
}
