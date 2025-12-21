use super::instruction::InstructionInstance;
use crate::parser::Player;
use std::collections::HashMap;

struct CorHeader {
    magic: u32,
    name: [u8; 128],
    padding: [u8; 4],
    prog_size: u32,
    comment: [u8; 2048],
}

impl CorHeader {
    fn new(name_str: &str, comment_str: &str, prog_size: u32) -> Self {
        let mut name = [0u8; 128];
        let mut comment = [0u8; 2048];
        name[..name_str.len()].copy_from_slice(name_str.as_bytes());
        comment[..comment_str.len()].copy_from_slice(comment_str.as_bytes());
        Self {
            magic: 0x00ea83f3,
            name,
            padding: [0u8; 4],
            prog_size,
            comment,
        }
    }
}

fn compute_program_size(instructions: &[InstructionInstance]) -> u32 {
    instructions.iter().map(InstructionInstance::compute_instruction_size).sum()
}

pub fn encode(player: Player) -> Result<Vec<u8>, String> {
    let prog_size = compute_program_size(&player.instructions);
    let head = CorHeader::new(&player.name, &player.comment, prog_size);

    let mut buffer = Vec::new();

    // Serialize header
    buffer.extend(&head.magic.to_be_bytes());
    buffer.extend(&head.name);
    buffer.extend(&head.padding);
    buffer.extend(&head.prog_size.to_be_bytes());
    buffer.extend(&head.comment);
    buffer.extend(&head.padding);

    let labels = first_pass(&player.instructions)?;
    let body = second_pass(&player.instructions, &labels)?;
    buffer.extend_from_slice(&body);

    Ok(buffer)
}

// First pass: just calculate positions, don't encode yet
fn first_pass(instructions: &[InstructionInstance]) -> Result<HashMap<String, usize>, String> {
    let mut labels = HashMap::new();
    let mut byte_position = 0;

    for inst in instructions {
        // Record label if present
        if let Some(label_name) = &inst.label {
            if labels.insert(label_name.clone(), byte_position).is_some() {
                return Err(format!("the label {} doubled", label_name));
            }
        }
        if let Some(instruction) = inst.instr {
            if instruction.nb_params != (inst.params.len() as u8) {
                return Err(
                    format!(
                            "the {} instruction should have {} parameters instead of {}",
                            instruction.name,
                            instruction.nb_params,
                            inst.params.len()
                        )
                );
            }
        }
        // Calculate instruction size (don't encode yet)
        byte_position += inst.calculate_instruction_size();
    }

    Ok(labels)
}

// Second pass: encode with all labels known
fn second_pass(
    instructions: &[InstructionInstance],
    labels: &HashMap<String, usize>
) -> Result<Vec<u8>, String> {
    let mut bytecode = Vec::new();
    let mut current_pos = 0;

    for inst in instructions {
        let encoded = inst.encode(current_pos, labels)?;
        current_pos += encoded.len();
        bytecode.extend(encoded);
    }

    Ok(bytecode)
}
