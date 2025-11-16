use std::{fs, path::PathBuf};
use anyhow::Result;
use crate::serialization::{VkJson};
use serde_json::from_str;

pub fn generate_contract(vk_path: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    let vk_str = fs::read_to_string(vk_path)?;
    let vk: VkJson = from_str(&vk_str)?;

    fs::create_dir_all(out_dir)?;

    let contract_path = out_dir.join("Verifier.ts");

    let gamma_abc = vk.gamma_abc_g1
        .iter()
        .map(|v| format!("\"{}\"", v))
        .collect::<Vec<_>>()
        .join(", ");

    let ts = format!(
r#"import {{ Field, Bool, Struct }} from "o1js";
import {{ Groth16Proof, Groth16Verifier }} from "o1js";

export class Verifier {{
  static vk = {{
    alpha: "{alpha}",
    beta: "{beta}",
    gamma: "{gamma}",
    delta: "{delta}",
    gamma_abc: [{gamma_abc}]
  }};

  static verify(proof: Groth16Proof, publicInput: Field[]): Bool {{
    return Groth16Verifier.verify(proof, Verifier.vk, publicInput);
  }}
}}
"#,
alpha = vk.alpha_g1,
beta = vk.beta_g2,
gamma = vk.gamma_g2,
delta = vk.delta_g2,
gamma_abc = gamma_abc
    );

    fs::write(contract_path, ts)?;

    Ok(())
}
