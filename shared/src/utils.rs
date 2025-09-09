use anyhow::Result;
use std::fs::File;
use std::io::Write;
use anyhow::anyhow;

pub fn write_cor_file(path: &str, bytes: Vec<u8>) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

pub fn read_u8(bytes: &[u8], cursor: &mut usize) -> Result<u8> {
    let b = *bytes
        .get(*cursor)
        .ok_or_else(|| anyhow!("Unexpected EOF at {}", cursor))?;
    *cursor += 1;
    Ok(b)
}

pub fn read_i16_be(bytes: &[u8], cursor: &mut usize) -> Result<i16> {
    let slice = bytes
        .get(*cursor..*cursor + 2)
        .ok_or_else(|| anyhow!("Unexpected EOF reading i16 at {}", cursor))?;
    *cursor += 2;
    Ok(i16::from_be_bytes([slice[0], slice[1]]))
}

pub fn read_i32_be(bytes: &[u8], cursor: &mut usize) -> Result<i32> {
    let slice = bytes
        .get(*cursor..*cursor + 4)
        .ok_or_else(|| anyhow!("Unexpected EOF reading i32 at {}", cursor))?;
    *cursor += 4;
    Ok(i32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]))
}
