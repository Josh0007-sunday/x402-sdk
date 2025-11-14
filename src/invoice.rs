use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::crypto::hash_preimage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Invoice {
    pub id: String,
    pub memo: String,
    pub amount_sats: u64,
    pub receiver_pubkey: Option<String>,  // Lightning node pubkey to receive payment
    pub preimage: String,
    pub preimage_hash: String,
    // optional fields for LND:
    pub bolt11: Option<String>,
    pub expiry: Option<u64>,
}

impl Invoice {
    pub fn new(memo: &str, amount_sats: u64, receiver_pubkey: Option<&str>) -> Self {
        let id = Uuid::new_v4().to_string();
        let preimage = format!("x402-preimage-{}", Uuid::new_v4());
        let preimage_hash = hash_preimage(&preimage);
        Invoice {
            id,
            memo: memo.to_string(),
            amount_sats,
            receiver_pubkey: receiver_pubkey.map(|s| s.to_string()),
            preimage,
            preimage_hash,
            bolt11: None,
            expiry: None,
        }
    }
}
