pub mod blake3 {
use std::{fs::File, io::{Read, Write}};
use anyhow::Result;
use camino::Utf8Path;

pub fn calc_hash(reader: &mut dyn Read) -> Result<Vec<u8>> {
    let mut hasher = blake3::Hasher::new();
    let mut buffer = vec![0; 1024 * 1024];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash_result = hasher.finalize();
    Ok(Vec::from(hash_result.as_slice()))
}

pub fn calc_file_hash(path: &Utf8Path) -> Result<Vec<u8>> {
    let mut file = std::fs::File::open(path)?;
    calc_hash(&mut file)
}

pub fn create_hash_file(path: &Utf8Path) -> Result<Vec<u8>> {
    let hash = calc_file_hash(path)?;

    let hash_file_path = path.to_string() + ".hash";
    let mut file = File::create(&hash_file_path)?;
    file.write_all(&hash)?;

    Ok(hash)
}

}

