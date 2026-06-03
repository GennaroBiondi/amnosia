use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Mind {},
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => println!("Running: {file}"),
        Commands::Info => println!("Some info"),
    }
}
