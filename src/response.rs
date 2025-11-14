use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "status")]
pub enum X402Response {
    Ok { code: u16, message: String, txid: String },
    InvalidSignature { code: u16, message: String },
    Underpaid { code: u16, message: String },
    Expired { code: u16, message: String },
    Error { code: u16, message: String },
}
