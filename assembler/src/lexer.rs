
use shared::instructions::valid_instruction;
#[derive(Debug)]
pub enum Token {
    Instr(String),
    Register(u8),
    Direct(i32),
    Indirect(i32),
    LabelDef(String),
    LabelRef(String),
    Comma,
}

pub fn tokenize(line: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let line = line.split(|c| c == ';' || c == '#').next().unwrap_or(""); // remove comments

    for part in line.split(|c| char::is_whitespace(c) || c == ',').filter(|s| !s.is_empty()) {
        let p = part.trim();
        if p.ends_with(':') {
            tokens.push(Token::LabelDef(p.trim_end_matches(':').to_string()));
        } else if let Some(reg) = p.strip_prefix('r') {
            match validate_register(reg) {
                Ok(num) => tokens.push(Token::Register(num)),
                Err(e) => return Err(format!("{e}"))
            }
        } else if let Some(dir) = p.strip_prefix('%') {
            if let Ok(val) = dir.parse::<i32>() {
                tokens.push(Token::Direct(val));
            } else if let Some(dir) = dir.strip_prefix(':') {
                tokens.push(Token::LabelRef(dir.to_string()));
            }else {
                return Err(format!("invalid Direct argument"));
            }
        } else if valid_instruction(p){
            tokens.push(Token::Instr(p.to_string()));
        } else if let Ok(num) = p.parse::<i32>() {
            tokens.push(Token::Indirect(num));
        } else {
            return Err(format!("invalid Indirect argument"));
        }
    }

    Ok(tokens)
}


fn validate_register(r: &str) -> Result<u8, String> {
    let reg = r.parse::<u8>()
            .map_err(|e| format!("invalid register: {e}"))?;
    if reg > 16 || reg < 1 {
        return Err("register too long, or too smal".to_string());
    }
    Ok(reg)
}