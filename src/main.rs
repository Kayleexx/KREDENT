use anyhow::Result;
use kredent::zk::{generate_parameters, generate_proof, Fr};
use kredent::serialization::{vk_to_json, proof_to_json};
use std::fs;

fn main() -> Result<()> {
    let (pk, vk) = generate_parameters()?;
    fs::write("vk.json", serde_json::to_string_pretty(&vk_to_json(&vk)?)?)?;

    let secret = Fr::from(42u64);
    let (proof, public) = generate_proof(&pk, secret)?;
    fs::write("proof.json", serde_json::to_string_pretty(&proof_to_json(&proof, public)?)?)?;

    Ok(())
}
