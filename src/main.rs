// src/main.rs
use clap::{Parser, Subcommand};
use anyhow::Result;
use ironmonger::core::secret_generator::SecretGenerator;

#[derive(Parser)]
#[command(name = "ironmonger")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CreateSecret,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::CreateSecret => {
            let secret = SecretGenerator::generate()?;
            SecretGenerator::write_to_env(&secret)?;
            println!("New APP_SECRET generated and saved: {}", secret);
        }
    }
    Ok(())
}