use hex; // For converting hash bytes to hexadecimal string
use sha3::{Digest, Keccak256}; // Ethereum uses Keccak256 for hashing 

pub fn hash(data: &str) -> String {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let result = hasher.finalize();
    // Convert the hash result to a hexadecimal string
    let hex_result = hex::encode(result);
    hex_result
}

pub fn hash_pair(left: &str, right: &str) -> String {
    let combined = format!("{}{}", left, right);
    hash(&combined)
}
