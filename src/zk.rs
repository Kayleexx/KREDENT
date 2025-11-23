use anyhow::Result;
use ark_bn254::Bn254;
use ark_ec::pairing::Pairing;
use ark_ff::Field;
use ark_groth16::{Groth16, Proof, ProvingKey, VerifyingKey};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_r1cs_std::{alloc::AllocVar, eq::EqGadget, fields::fp::FpVar};
use ark_r1cs_std::fields::FieldVar; 
use ark_snark::SNARK;
use ark_std::rand::rngs::OsRng;

pub type Curve = Bn254;
pub type Fr = <Curve as Pairing>::ScalarField;
pub type Groth16ProvingKey = ProvingKey<Curve>;
pub type Groth16VerifyingKey = VerifyingKey<Curve>;
pub type Groth16Proof = Proof<Curve>;

#[derive(Clone)]
pub struct HashPreimageCircuit {
    pub secret: Option<Fr>,
    pub hash: Option<Fr>,
}

impl HashPreimageCircuit {
    fn hash_native(x: Fr) -> Fr {
        let r = |s: Fr, c: u64| s.square().square() * s + Fr::from(c);
        r(r(r(x, 5), 7), 11)
    }

    pub fn compute_public_hash(secret: Fr) -> Fr {
        Self::hash_native(secret)
    }
}

pub fn compute_nullifier(secret: Fr) -> Fr {
    let mut x = secret + Fr::from(123456u64);
    x = x.square();
    x
}

impl ConstraintSynthesizer<Fr> for HashPreimageCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let s = FpVar::<Fr>::new_witness(cs.clone(), || self.secret.ok_or(SynthesisError::AssignmentMissing))?;
        let h = FpVar::<Fr>::new_input(cs.clone(), || self.hash.ok_or(SynthesisError::AssignmentMissing))?;

        let round = |x: FpVar<Fr>, c: u64| -> Result<FpVar<Fr>, SynthesisError> {
            Ok(x.square()?.square()? * x + FpVar::constant(Fr::from(c)))
        };

        let mut st = s;
        st = round(st, 5)?;
        st = round(st, 7)?;
        st = round(st, 11)?;
        st.enforce_equal(&h)?;
        Ok(())
    }
}

pub fn generate_parameters() -> Result<(Groth16ProvingKey, Groth16VerifyingKey)> {
    let mut rng = OsRng;
    let circuit = HashPreimageCircuit { secret: None, hash: None };
    let (pk, vk) = Groth16::<Curve>::circuit_specific_setup(circuit, &mut rng)?;
    Ok((pk, vk))
}

pub fn generate_proof(pk: &Groth16ProvingKey, secret: Fr) -> Result<(Groth16Proof, Fr, Fr)> {
    let mut rng = OsRng;

    let public_hash = HashPreimageCircuit::compute_public_hash(secret);
    let nullifier = compute_nullifier(secret);

    let circuit = HashPreimageCircuit {
        secret: Some(secret),
        hash: Some(public_hash),
    };

    let proof = Groth16::<Curve>::prove(pk, circuit, &mut rng)?;
    Ok((proof, public_hash, nullifier))
}


pub fn verify_proof(vk: &Groth16VerifyingKey, proof: &Groth16Proof, public_hash: Fr, nullifier: Fr) -> Result<bool> {
    Ok(Groth16::<Curve>::verify(vk, &[public_hash, nullifier], proof)?)
}
