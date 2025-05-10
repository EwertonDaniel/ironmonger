// src/main.rs
use clap::Command;
use anyhow::Result;
use ironmonger::core::secret_generator::SecretGenerator;

fn build_cli() -> Command {
    Command::new("ironmonger")
        .about("Ironmonger CLI tools")
        .subcommand(
            Command::new("create:secret")
                .alias("create-secret")
                .about("Generate and store a new APP_SECRET"),
        )
}

fn main() -> Result<()> {
    let matches = build_cli().get_matches();
    match matches.subcommand_name() {
        Some("create:secret") => {
            let secret = SecretGenerator::generate()?;
            SecretGenerator::write_to_env(&secret)?;
            println!("New APP_SECRET generated and saved: {}", secret);
        },
        Some(&_) => todo!(),
        None => todo!(),

    }
    Ok(())
}