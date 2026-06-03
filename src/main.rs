use anyhow::Result;
use clap::Parser;
use commands::{Cli, Commands};
use newtype::UnixTimestamp;
use reminder::Reminder;
use reminder_list::ReminderList;
use std::path::PathBuf;

mod commands;
mod reminder;
mod reminder_list;

fn expand_path(s: &str) -> Result<PathBuf> {
    if let Some(rest) = s.strip_prefix("~/") {
        let home = dirs_next::home_dir().ok_or(anyhow::anyhow!("Home directory not found!"))?;
        Ok(home.join(rest))
    } else {
        Ok(PathBuf::from(s))
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let path = cli
        .reminder_path
        .map(|p| expand_path(&p.to_string_lossy()))
        .unwrap_or_else(|| expand_path("~/.local/share/amnosia/reminders.txt"))?;

    match cli.command {
        Commands::Mind { entry } => {
            let timestamp = UnixTimestamp::now();
            let reminder_entry: Reminder = Reminder {
                entry,
                timestamp: timestamp,
            };
            reminder_entry.append_to_file(&path)?;
        }
        Commands::Remind { include_dates } => {
            let reminder_list = ReminderList::from_file(&path)?;
            for reminder in reminder_list.get_vec() {
                match include_dates {
                    Some(true) => {
                        println!("[{}]: {}", reminder.timestamp.prettify(), reminder.entry)
                    }
                    _ => println!("{}", reminder.entry),
                }
            }
        }
        Commands::Demind { query } => {
            let mut reminder_list = ReminderList::from_file(&path)?;
            if let None = reminder_list.delete_reminder_by_entry_fuzzy(&query) {
                println!("No entry found matching query: {}", query);
            }
            reminder_list.dump_to_file(&path)?;
        }
    }

    Ok(())
}
