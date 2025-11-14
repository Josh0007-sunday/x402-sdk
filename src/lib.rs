mod crypto;
mod invoice;
mod response;
mod verifier;
mod payment;
mod neon_bindings;

use neon::prelude::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    neon_bindings::register(&mut cx)?;
    Ok(())
}
