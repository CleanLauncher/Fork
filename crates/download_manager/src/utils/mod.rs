use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Read, Result};

pub fn verify_file_hash(filepath: &str, expected_hash: &str) -> Result<bool> {
    let mut file = File::open(filepath)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let result = hasher.finalize();
    let file_hash = hex::encode(result);

    Ok(file_hash.eq_ignore_ascii_case(expected_hash))
}
