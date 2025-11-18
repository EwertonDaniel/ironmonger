use clap::{Arg, Command};

pub fn build_cli() -> Command {
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
                        .default_value(crate::infrastructure::SECRET_KEY_NAME),
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
