// Node wrapper that loads the compiled Neon native module.
// X402-SDK: Lightning Network payments for HTTP 402 Payment Required
let native;
try {
  native = require('./index.node');
} catch (e) {
  // Fallback to target directory
  native = require('./target/release/index.node');
}

module.exports = {
  // Create Lightning invoice
  // Usage: createInvoice(memo: string, amountSats: number, receiverPubkey?: string) -> JSON
  // receiverPubkey: Optional Lightning node public key to receive the payment
  createInvoice: native.createInvoice,

  // Verify payment amount
  // Usage: verifyPayment(invoiceJson: string, paidSats: number) -> JSON
  verifyPayment: native.verifyPayment
};
