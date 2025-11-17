use clap::{Arg, Command};
use ironmonger::domain::errors::Result;
use ironmonger::infrastructure::env_writer::EnvFileWriter;
use ironmonger::infrastructure::secret_generator::SecretGenerator;
use ironmonger::infrastructure::SECRET_KEY_NAME;
use std::path::Path;

fn build_cli() -> Command {
    Command::new("ironmonger")
        .about("Ironmonger CLI - Generate and manage application secrets")
        .version(env!("CARGO_PKG_VERSION"))
        .author("EchoSistema")
        .subcommand(
            Command::new("create:secret")
                .alias("create-secret")
                .about("Generate and store a new application secret")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .value_name("KEY_NAME")
                        .help("Name of the environment variable (default: APP_SECRET)")
                        .default_value(SECRET_KEY_NAME),
                )
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .value_name("FILE_PATH")
                        .help("Path to the .env file (default: .env)")
                        .default_value(".env"),
                ),
        )
}

fn main() -> Result<()> {
    let matches = build_cli().get_matches();

    match matches.subcommand() {
        Some(("create:secret", sub_matches)) => {
            let key_name = sub_matches.get_one::<String>("name").unwrap();
            let file_path = sub_matches.get_one::<String>("file").unwrap();

            let generator = SecretGenerator::new();
            let secret = generator.generate()?;

            let writer = EnvFileWriter::new(Path::new(file_path), key_name);
            writer.write(&secret)?;

            println!("✓ New {} generated and saved to {}", key_name, file_path);
            println!("  Secret: {}", secret);
        }
        _ => {
            println!("No valid subcommand provided. Use --help for more information.");
            std::process::exit(1);
        }
    }

    Ok(())
}
