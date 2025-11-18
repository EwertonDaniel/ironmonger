use ironmonger::cli::build_cli;
use ironmonger::domain::errors::Result;
use ironmonger::infrastructure::env_writer::EnvFileWriter;
use ironmonger::infrastructure::secret_generator::SecretGenerator;
use std::path::Path;

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
