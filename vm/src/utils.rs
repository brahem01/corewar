use crate::player::Player;
use crate::config;
use std::fs::File;
use std::io::Read;

pub fn parse_arguments(files: &[String]) -> Result<Vec<Player>, String> {
    if files.len() < 2 || files.len() > 4 {
        return Err("Usage: vm [-v] <file1.cor> <file2.cor> [file3.cor] [file4.cor]".into());
    }
    let mut players = Vec::new();
    
    for (i, file_name) in files.iter().enumerate() {
        if !file_name.ends_with(".cor") {
            return Err(format!("Invalid file extension: {}", file_name));
        }
        // if i == 0 {continue}
        let mut file = File::open(file_name)
            .map_err(|e| format!("Error opening {}: {}", file_name, e))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Error reading {}: {}", file_name, e))?;

        // ---- magic ----
        if buffer.len() < 4 || buffer[..4] != [0x00, 0xea, 0x83, 0xf3] {
            return Err(format!("Invalid magic number in {}", file_name));
        }

        let mut offset = 4;

        // ---- name ----
        let name = std::str::from_utf8(&buffer[offset..offset + 128])
            .map_err(|_| "Invalid name encoding")?
            .trim_matches('\0')
            .to_string();
        offset += 128 + 4;

        // ---- program size ----
        let size = u32::from_be_bytes(buffer[offset..offset + 4].try_into().unwrap());
        if size as usize > config::PLAYER_MAX_SIZE {
            return Err("Program too large".into());
        }
        offset += 4;

        // ---- comment ----
        let comment = std::str::from_utf8(&buffer[offset..offset + 2048])
            .map_err(|_| "Invalid comment encoding")?
            .trim_matches('\0')
            .to_string();
        offset += 2048 + 4;

        // ---- program ----
        let program = buffer[offset..offset + size as usize].to_vec();

        players.push(Player::new(
            -1,
            name,
            comment,
            program,
            size,
            0,
        ));
    }

    players.reverse();
    Ok(players)
}
