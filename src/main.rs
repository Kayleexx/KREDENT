mod zk;

use anyhow::Result;
use zk::{generate_parameters, generate_proof, verify_proof, Fr};

fn main() -> Result<()> {
    println!(
        "kredent: zk core loaded (Step 1). Run `cargo test` for a full prove/verify roundtrip."
    );

    // Optional lightweight smoke test at runtime (non-fatal on error).
    if let Err(e) = smoke_test() {
        eprintln!("warning: zk smoke test failed: {e}");
    }

    Ok(())
}

fn smoke_test() -> Result<()> {
    let (pk, vk) = generate_parameters()?;

    let secret = Fr::from(42u64);
    let (proof, public_hash) = generate_proof(&pk, secret)?;
    let ok = verify_proof(&vk, &proof, public_hash)?;

    if ok {
        println!("zk smoke test: OK");
    } else {
        println!("zk smoke test: FAILED");
    }

    Ok(())
}
