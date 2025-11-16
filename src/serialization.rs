use anyhow::Result;
use serde::{Serialize, Deserialize};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use std::fs::File;
use std::io::{Write, Read};
use ark_ff::PrimeField;
use ark_ff::BigInteger;


use crate::zk::{Groth16Proof, Groth16VerifyingKey, Groth16ProvingKey, Fr};

#[derive(Serialize, Deserialize)]
pub struct VkJson {
    pub alpha_g1: String,
    pub beta_g2: String,
    pub gamma_g2: String,
    pub delta_g2: String,
    pub gamma_abc_g1: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProofJson {
    pub a: String,
    pub b: String,
    pub c: String,
    pub public_inputs: Vec<String>,
}

fn to_hex<T: CanonicalSerialize>(v: &T) -> Result<String> {
    let mut buf = Vec::new();
    v.serialize_compressed(&mut buf)?;
    Ok(hex::encode(buf))
}

pub fn vk_to_json(vk: &Groth16VerifyingKey) -> Result<VkJson> {
    Ok(VkJson {
        alpha_g1: to_hex(&vk.alpha_g1)?,
        beta_g2: to_hex(&vk.beta_g2)?,
        gamma_g2: to_hex(&vk.gamma_g2)?,
        delta_g2: to_hex(&vk.delta_g2)?,
        gamma_abc_g1: vk.gamma_abc_g1.iter().map(|g| to_hex(g).unwrap()).collect(),
    })
}

pub fn proof_to_json(proof: &Groth16Proof, public: Fr, nullifier: Fr) -> Result<ProofJson> {
    Ok(ProofJson {
        a: to_hex(&proof.a)?,
        b: to_hex(&proof.b)?,
        c: to_hex(&proof.c)?,
        public_inputs: vec![
            hex::encode(public.into_bigint().to_bytes_be()),
            hex::encode(nullifier.into_bigint().to_bytes_be()),
        ],
    })
}

pub fn save_pk(pk: &Groth16ProvingKey, path: &str) -> Result<()> {
    let mut buf = Vec::new();
    pk.serialize_compressed(&mut buf)?;
    let mut f = File::create(path)?;
    f.write_all(&buf)?;
    Ok(())
}

pub fn load_pk(path: &str) -> Result<Groth16ProvingKey> {
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    Ok(Groth16ProvingKey::deserialize_compressed(&buf[..])?)
}
