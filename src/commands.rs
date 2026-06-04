use clap::{Args, Parser, Subcommand};
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short = 'p', long = "path", global = true)]
    pub reminder_path: Option<PathBuf>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Query {
    /// Fuzzy search
    #[arg(short, long)]
    pub query: Option<String>,

    /// Exact match
    #[arg(short = 'e', long)]
    pub exact_query: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Mind {
        #[arg(short, long)]
        entry: String,

        #[arg(short, long)]
        deadline: Option<String>,
    },
    Remind {
        #[arg(short = 'n', long = "number-limit")]
        number_limit: Option<usize>,

        #[arg(short = 'd', long, action = clap::ArgAction::SetTrue)]
        include_dates: bool,
    },
    Demind {
        #[command(flatten)]
        query: Query,
    },
}
