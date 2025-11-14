use sha2::{Digest, Sha256};
use hex;

// Hash preimage with SHA256 for invoice generation
pub fn hash_preimage(preimage: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(preimage.as_bytes());
    hex::encode(hasher.finalize())
}
