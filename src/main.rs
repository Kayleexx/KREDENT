mod zk;
mod serialization;
mod contract_gen;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use contract_gen::generate_contract;

#[derive(Debug, Parser)]
#[command(
    name = "kredent",
    version,
    about = "KREDENT: ZK Rosetta Stone"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    GenerateKeys {},
    Prove {
        #[arg(long)]
        secret: String,
        #[arg(long)]
        out: PathBuf,
    },
    Verify {
        #[arg(long)]
        proof: PathBuf,
    },
    GenerateContract {
        #[arg(long)]
        out_dir: PathBuf,
    },
    Pay {
        #[arg(long)]
        to: String,
        #[arg(long)]
        memo: String,
        #[arg(long)]
        amount: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateContract { out_dir } => {
            generate_contract(&PathBuf::from("vk.json"), &out_dir)?;
            println!("Verifier.ts created in {:?}", out_dir);
        }
        _ => println!("Command not implemented yet"),
    }

    Ok(())
}
