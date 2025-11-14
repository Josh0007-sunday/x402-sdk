use crate::invoice::Invoice;
use serde::{Serialize, Deserialize};
use std::env;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use hex;
use std::fs;

#[derive(Serialize, Deserialize)]
struct AddInvoiceReq {
    memo: String,
    value: u64,
    // to set custom preimage: preimage field (base64) can be passed to LND AddInvoice; but requires exact format.
    // We'll not pass preimage by default. For X402 offline preimage approach, you'd want to set preimage.
}

#[derive(Serialize, Deserialize)]
struct AddInvoiceResp {
    payment_request: Option<String>,
    r_hash: Option<String>,
    // lnd returns other fields; we only parse what we need
}

pub fn create_invoice_on_lnd(invoice: &mut Invoice) -> Result<(), String> {
    let rest_url = env::var("LND_REST_URL").map_err(|_| "LND_REST_URL not set".to_string())?;
    // macaroon header: hex macaroon value
    let macaroon = match env::var("LND_MACAROON") {
        Ok(m) => m,
        Err(_) => {
            // try reading from path
            match env::var("LND_MACAROON_PATH") {
                Ok(p) => {
                    let b = fs::read(p).map_err(|e| format!("read macaroon file error: {}", e))?;
                    hex::encode(b)
                },
                Err(_) => return Err("LND_MACAROON or LND_MACAROON_PATH must be set".to_string())
            }
        }
    };

    let client = Client::builder().danger_accept_invalid_certs(true).build().map_err(|e| format!("{}", e))?;
    let mut headers = HeaderMap::new();
    headers.insert("Grpc-Metadata-macaroon", HeaderValue::from_str(&macaroon).map_err(|e| format!("{}", e))?);

    let body = AddInvoiceReq {
        memo: invoice.memo.clone(),
        value: invoice.amount_sats,
    };

    let resp = client.post(&format!("{}/v1/invoices", rest_url))
        .headers(headers)
        .json(&body)
        .send()
        .map_err(|e| format!("request error: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_else(|_| "".to_string());
        return Err(format!("LND addinvoice failed: {} {}", status, text));
    }

    let parsed: AddInvoiceResp = resp.json().map_err(|e| format!("parse addinvoice resp: {}", e))?;
    invoice.bolt11 = parsed.payment_request;
    Ok(())
}
