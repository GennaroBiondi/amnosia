use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short = 'p', long = "path")]
    pub reminder_path: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    Mind {
        #[arg(short, long)]
        entry: String,
    },
    Remind {
        #[arg(short = 'd', long)]
        include_dates: Option<bool>,
    },
    Demind {
        #[arg(short, long)]
        query: String,
    },
}
