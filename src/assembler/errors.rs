use thiserror::Error;

#[derive(Debug, Error)]
pub enum AsmError {
    #[error("Unknown instruction: {0}")]
    UnknownInstruction(String),

    #[error("Invalid parameter for instruction {0}")]
    InvalidParam(String),

    #[error("Label not found: {0}")]
    MissingLabel(String),
}
