extern crate sha3;

use sha3::{Digest, Sha3_256};

pub fn compute_sha3(input: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
