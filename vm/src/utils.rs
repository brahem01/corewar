use crate::player::Player;
use crate::*;
use std::fs::File;
use std::io::Read;
pub fn parse_arguments(args: Vec<String>) -> Result<(Vec<Player>, Option<usize>, bool), String> {
    /*_____________read arguments___________________ */
    if args.len() < 2 {
        // panic!("USAGE: assembler [arguments..]\nyou should atleast enter one argument.");
        return Err(format!(
            "USAGE: ./vm first_file second_file ... Optional::(-d number_of_cycles_to_dumb)"
        ));
    }
    let mut verbos = false;

    let mut warriors_data = Vec::new();
    let mut dump_cycles: Option<usize> = None;
    let mut cursor = 1;
    let mut jid = 1;
    while cursor < args.len() {
        if args[cursor] == "-d" && cursor + 1 < args.len() {
            if let Ok(cycles) = args[cursor + 1].parse::<usize>() {
                dump_cycles = Some(cycles);
                cursor += 2;
            } else {
                return Err("the argument after -d should be number".to_string());
            }
            continue;
        }
        if args[cursor] == "-v" {
            verbos = true;
            cursor += 1;
            continue;
        }
        let d = parse_file(&args[cursor], jid)?;
        jid += 1;
        warriors_data.push(d);
        cursor += 1;
    }

    return Ok((warriors_data, dump_cycles, verbos));
}

fn parse_file(file_name: &str, jid: usize) -> Result<Player, String> {
    if !file_name.ends_with(".cor") {
        panic!("{}", red("bad file extention!"));
    }

    let mut file =
        File::open(file_name).map_err(|e| red(&format!("Error Opening the file, {e}")))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| red(&format!("Error reading the file, {e}")))?;

    //todo!
    // if buffer.len() < config::HEADERS_SIZE {
    //     return Err(red("the file are too smaaaaal"));
    // }

    let mut prev = 0;
    let mut next = 4;

    let magic = &buffer[prev..next];
    if magic != [0x00, 0xea, 0x83, 0xf3] {
        println!("invalid magic number: {:?}", magic);
        return Err(format!("invalid magic number: {:?}", magic));
    }
    // 128 + 4
    prev = next;
    next = next + 128;

    let name =
        std::str::from_utf8(&buffer[prev..next]).map_err(|e| red(&format!("small file {e}")))?;
    let name: String = name.chars().filter(|&c| c != '\0').collect();

    prev = next + 4; // skip 4 bytes
    next = prev + 4;

    let mut arr = [0u8; 4];
    arr.copy_from_slice(&buffer[prev..next]);
    let size = u32::from_be_bytes(arr);
    if (size as usize) > config::PLAYER_MAX_SIZE {
        return Err(red("the file size are too big"));
    }
    prev = next; // skip 4 bytes
    next = prev + 2048;

    let disc = std::str::from_utf8(&buffer[prev..next])
        .map_err(|e| red(&format!("small file {e}")))?
        .trim();
    let disc: String = disc.chars().filter(|&c| c != '\0').collect();

    prev = next + 4; // skip 4 bytes
    next = prev + (size as usize);

    let program = &buffer[prev..next];
    if program.len() != (size as usize) {
        return Err(red("the size is the header not the actual program size "));
    }
    let player = Player::new(
        -1 * jid as i32, //todo!()
        name.to_string(),
        disc.to_string(),
        program.to_vec(),
        size,
    );
    Ok(player)
}
