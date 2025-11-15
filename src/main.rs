use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "kredent",
    version,
    about = "KREDENT: A Rust-native ZK 'Rosetta Stone' for Mina o1js and Zcash.",
    author = "KREDENT-ARCHITECT"
)]
struct Cli {
    #[arg(
        long,
        global = true,
        value_name = "DIR",
        help = "Base directory for KREDENT artifacts (defaults to current working directory)."
    )]
    base_dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    GenerateKeys {
        #[arg(
            long,
            value_name = "DIR",
            help = "Directory to store generated parameters (proving_key, verification_key)."
        )]
        out_dir: Option<PathBuf>,
    },

    Prove {
        #[arg(
            long,
            value_name = "SECRET",
            help = "Secret value such that Poseidon(secret) = public hash."
        )]
        secret: String,

        #[arg(
            long,
            value_name = "FILE",
            default_value = "proof.json",
            help = "Path to write the generated proof JSON."
        )]
        out: PathBuf,
    },

    Verify {
        #[arg(
            long,
            value_name = "FILE",
            default_value = "proof.json",
            help = "Path to the proof JSON file to verify."
        )]
        proof: PathBuf,
    },

    GenerateContract {
        #[arg(
            long,
            value_name = "DIR",
            default_value = "contract",
            help = "Directory to write the generated o1js verifier contract."
        )]
        out_dir: PathBuf,
    },

    Pay {
        #[arg(
            long,
            value_name = "ADDRESS",
            help = "Destination Zcash shielded address."
        )]
        to: String,

        #[arg(
            long,
            value_name = "TEXT",
            default_value = "",
            help = "Optional memo text for the payment."
        )]
        memo: String,

        #[arg(
            long,
            value_name = "AMOUNT",
            help = "Amount to send (unit to be defined in Step 5)."
        )]
        amount: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateKeys { out_dir } => {
            println!(
                "[kredent] generate-keys called (Step 1 will implement this). out_dir = {:?}",
                out_dir
            );
        }
        Commands::Prove { secret, out } => {
            println!(
                "[kredent] prove called (Step 1 & 3 will implement this). secret = ****, out = {:?}",
                out
            );
        }
        Commands::Verify { proof } => {
            println!(
                "[kredent] verify called (Step 1 will implement this). proof = {:?}",
                proof
            );
        }
        Commands::GenerateContract { out_dir } => {
            println!(
                "[kredent] generate-contract called (Step 2 & 3 will implement this). out_dir = {:?}",
                out_dir
            );
        }
        Commands::Pay { to, memo, amount } => {
            println!("[kredent] pay called (Step 5 will implement this).");
            println!("  to: {to}");
            println!("  memo: {memo}");
            println!("  amount: {amount}");
        }
    }

    Ok(())
}
