use sha2::{Digest, Sha256}; // Ethereum uses Keccak256 for hashing // Ethereum uses Keccak256 for hashing

pub fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn hash_pair(left: &str, right: &str) -> String {
    let combined = format!("{}{}", left, right);
    hash(&combined)
}
