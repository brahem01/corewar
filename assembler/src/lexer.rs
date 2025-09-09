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

pub fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let line = line.split(';').next().unwrap_or(""); // remove comments

    for part in line.split(|c| c == ' ' || c == ',').filter(|s| !s.is_empty()) {
        let p = part.trim();
        if p.ends_with(':') {
            tokens.push(Token::LabelDef(p.trim_end_matches(':').to_string()));
        } else if let Some(reg) = p.strip_prefix('r') {
            if let Ok(num) = reg.parse::<u8>() {
                tokens.push(Token::Register(num));
            }
        } else if let Some(dir) = p.strip_prefix('%') {
            if let Ok(val) = dir.parse::<i32>() {
                tokens.push(Token::Direct(val));
            } else {
                tokens.push(Token::LabelRef(dir.to_string()));
            }
        } else if let Ok(num) = p.parse::<i32>() {
            tokens.push(Token::Indirect(num));
        } else {
            tokens.push(Token::Instr(p.to_string()));
        }
    }

    tokens
}
