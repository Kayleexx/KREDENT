mod zk;
mod serialization;
mod contract_gen;
mod pay;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};

use owo_colors::OwoColorize;
use figlet_rs::FIGfont;

use crate::zk::{generate_parameters, generate_proof, Fr};
use crate::serialization::{proof_to_json, save_pk, vk_to_json};
use crate::contract_gen::generate_contract;
use crate::pay::{send_shielded, PaymentRequest};

#[derive(Debug, Parser)]
#[command(
    name = "kredent",
    version,
    about = "Zero-Knowledge Credential & Privacy Tool – Rust → Groth16 → Mina → Zcash",
    long_about = "KREDENT allows users to prove attributes privately using zk-SNARKs (Groth16), generate proof artifacts, 
verify on-chain via Mina zkApps, and create shielded Zcash transactions — without revealing secrets."
)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    GenerateKeys {
        #[arg(long)]
        out_dir: Option<PathBuf>,
    },
    Prove {
        #[arg(long)]
        secret: String,
        #[arg(long)]
        out: PathBuf,
    },
    GenerateContract {
        #[arg(long)]
        out_dir: PathBuf,
    },
    Pay {
        #[arg(long)]
        to: String,
        #[arg(long)]
        amount: u64,
        #[arg(long)]
        memo: String,
    },
    VerifyOnchain {
    #[arg(long)]
    proof: PathBuf,
    #[arg(long)]
    contract: String,
    #[arg(long)]
    key: String,
},

}

fn main() -> Result<()> {
    let font = FIGfont::standard().unwrap();
    let text = font.convert("KREDENT").unwrap();
    print!("{}", text.to_string().bright_green());
    println!("{}", "Rust → Groth16 → Mina → Zcash".bright_blue());

    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateKeys { out_dir } => {
            println!("{}", "[*] generating proving/verifying keys...".yellow());
            let (pk, vk) = generate_parameters()?;
            let dir = out_dir.unwrap_or_else(|| PathBuf::from("."));

            fs::create_dir_all(&dir)?;
            save_pk(&pk, &dir.join("pk.bin").to_string_lossy())?;
            fs::write(dir.join("vk.json"), serde_json::to_string_pretty(&vk_to_json(&vk)?)?)?;

            println!("{}", "✔ keys saved".bright_green());
        }

        Commands::Prove { secret, out } => {
            println!("{}", "[*] generating proof...".yellow());

            let (pk, _) = generate_parameters()?;
            let secret_fr = Fr::from(secret.parse::<u64>()?);

            let (proof, public_hash, nullifier) = generate_proof(&pk, secret_fr)?;
            let json = proof_to_json(&proof, public_hash, nullifier)?;

            fs::write(&out, serde_json::to_string_pretty(&json)?)?;
            println!("{}", "✔ proof generated with nullifier".bright_green());
        }

        Commands::GenerateContract { out_dir } => {
            println!("{}", "[*] generating o1js verifier...".yellow());
            generate_contract(&PathBuf::from("vk.json"), &out_dir)?;
            println!("{}", "✔ Verifier.ts created".bright_green());
        }

        Commands::Pay { to, amount, memo } => {
            println!("{}", "[*] building shielded transaction...".yellow());
            send_shielded(PaymentRequest { to, amount, memo })?;
            println!("{}", "✔ offline Zcash tx created".bright_green());
        }

        Commands::VerifyOnchain { proof, contract, key } => {
            println!("{}", "[*] Sending on-chain verification...".yellow());
            std::process::Command::new("node").arg("contract/call.js")
            .arg(&proof)
            .arg(&contract)
            .arg(&key)
            .status()
            .expect("failed to execute node");
        println!("{}", "✔ Verification submitted".bright_green());
}

    }

    Ok(())
}
