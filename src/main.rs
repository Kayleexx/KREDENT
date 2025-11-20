mod zk;
mod serialization;
mod contract_gen;
mod pay;
use pay::*;


use anyhow::Result;
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};
use owo_colors::OwoColorize;
use figlet_rs::FIGfont;

use zk::{generate_parameters, generate_proof};
use serialization::{vk_to_json, proof_to_json, save_pk, load_pk};
use contract_gen::generate_contract;

#[derive(Debug, Parser)]
#[command(name = "kredent", version)]
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
        #[arg(long, default_value = "pk.bin")]
        pk: String,
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

        Commands::Prove { secret, out, pk } => {
            println!("{}", "[*] generating proof...".yellow());
            let s = zk::Fr::from(secret.parse::<u64>()?);
            let pk_loaded = load_pk(&pk)?;
            let (proof, pubhash, nullifier) = generate_proof(&pk_loaded, s)?;
            fs::write(&out, serde_json::to_string_pretty(&proof_to_json(&proof, pubhash, nullifier)?)?)?;
            println!("{}", "✔ proof written".bright_green());
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
    }

    Ok(())
}
