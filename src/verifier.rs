use crate::invoice::Invoice;
use crate::response::X402Response;

// Verify payment amount only - LND settlement check done on server side
pub fn verify_payment(invoice: &Invoice, paid_sats: u64) -> X402Response {
    // Check if paid amount meets invoice requirement
    if paid_sats < invoice.amount_sats {
        return X402Response::Underpaid {
            code: 402,
            message: format!("Underpaid: {} sats paid, {} sats required", paid_sats, invoice.amount_sats)
        };
    }

    // Amount verified
    X402Response::Ok {
        code: 200,
        message: "Payment amount verified".to_string(),
        txid: invoice.id.clone()
    }
}
