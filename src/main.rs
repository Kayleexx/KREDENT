mod zk;
mod serialization;
mod contract_gen;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use owo_colors::OwoColorize;

use zk::{generate_parameters, generate_proof};
use serialization::{vk_to_json, proof_to_json};
use contract_gen::generate_contract;

#[derive(Debug, Parser)]
#[command(name = "kredent", about = "KREDENT: ZK Rosetta Stone", version)]
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
}

fn main() -> Result<()> {
    println!("{}", "KREDENT".bright_red().bold());
    println!("{}", "Rust → Groth16 Proofs → Mina".bright_blue().bold());
    println!();

    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateKeys { out_dir } => {
            println!("{}", "[*] Generating proving and verifying keys...".yellow());
            let (pk, vk) = generate_parameters()?;

            let dir = out_dir.unwrap_or_else(|| PathBuf::from("."));
            fs::create_dir_all(&dir)?;

            fs::write(dir.join("vk.json"), serde_json::to_string_pretty(&vk_to_json(&vk)?)?)?;

            println!("{}", "✔ Keys generated".bright_green());
            println!("Saved to: {:?}", dir);
        }

        Commands::Prove { secret, out } => {
            println!("{}", "[*] Generating proof...".yellow());

            let secret_f = zk::Fr::from(secret.parse::<u64>()?);
            let (proof, public_hash, nullifier) = generate_proof(&load_pk()?, secret_f)?;

            fs::write(&out, serde_json::to_string_pretty(&proof_to_json(&proof, public_hash, nullifier)?)?)?;

            println!("{}", "✔ Proof generated".bright_green());
            println!("Saved at: {:?}", out);
        }

        Commands::GenerateContract { out_dir } => {
            println!("{}", "[*] Generating Mina verifier contract...".yellow());
            generate_contract(&PathBuf::from("vk.json"), &out_dir)?;
            println!("{}", "✔ Verifier.ts created".bright_green());
            println!("Location: {:?}", out_dir);
        }
    }

    Ok(())
}

fn load_pk() -> Result<zk::Groth16ProvingKey> {
    anyhow::bail!("TODO: load proving key storage (coming soon)")
}
