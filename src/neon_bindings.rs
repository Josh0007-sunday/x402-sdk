use neon::prelude::*;
use serde_json;
use crate::invoice::Invoice;
use crate::verifier;
use crate::payment;

// createInvoice(memo: string, amount_sats: number, receiver_pubkey?: string) -> JSON invoice
fn js_create_invoice(mut cx: FunctionContext) -> JsResult<JsString> {
    let memo = cx.argument::<JsString>(0)?.value(&mut cx);
    let amount = cx.argument::<JsNumber>(1)?.value(&mut cx) as u64;

    // Optional receiver pubkey (Lightning node pubkey to receive sats)
    let receiver_pubkey = cx.argument_opt(2)
        .and_then(|arg| arg.downcast::<JsString, _>(&mut cx).ok())
        .map(|s| s.value(&mut cx));

    let mut invoice = Invoice::new(&memo, amount, receiver_pubkey.as_deref());

    // Try to create invoice on LND if configured; ignore error (still return local invoice)
    if let Err(e) = payment::create_invoice_on_lnd(&mut invoice) {
        let _ = e; // ignore or log
    }

    let s = serde_json::to_string(&invoice).unwrap();
    Ok(cx.string(s))
}

// verifyPayment(invoice_json, paid_sats) -> response json
// Simplified: only checks amount, LND settlement verified separately on server
fn js_verify_payment(mut cx: FunctionContext) -> JsResult<JsString> {
    let invoice_json = cx.argument::<JsString>(0)?.value(&mut cx);
    let paid_sats = cx.argument::<JsNumber>(1)?.value(&mut cx) as u64;

    let invoice: Invoice = match serde_json::from_str(&invoice_json) {
        Ok(inv) => inv,
        Err(e) => return cx.throw_error(format!("Invalid invoice JSON: {}", e))
    };

    let response = verifier::verify_payment(&invoice, paid_sats);
    let s = serde_json::to_string(&response).unwrap();
    Ok(cx.string(s))
}

pub fn register(cx: &mut ModuleContext) -> NeonResult<()> {
    cx.export_function("createInvoice", js_create_invoice)?;
    cx.export_function("verifyPayment", js_verify_payment)?;
    Ok(())
}
