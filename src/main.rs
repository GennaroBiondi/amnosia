use anyhow::{bail, Result};
use clap::Parser;
use commands::{Cli, Commands};
use newtype::UnixTimestamp;
use reminder::Reminder;
use reminder_list::ReminderList;
use std::{io::Write, path::PathBuf};

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

fn ask_user() -> Result<bool> {
    use std::io::stdin;

    let mut response = String::new();

    match stdin().read_line(&mut response) {
        Ok(_n) => match response.trim().to_lowercase().as_str() {
            "yes" | "y" | "yeah" | "positive" | "sure" => Ok(true),
            _ => Ok(false),
        },
        Err(error) => bail!("Error reading input: {error}"),
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
            let mut search_literally = true;
            match query.to_lowercase().as_str() {
                "all" => {
                    println!("You've entered 'all' as your query.");
                    println!("Did you mean to search for entries containing 'all'?");
                    print!("[y/n]: ");
                    std::io::stdout().flush()?;
                    if ask_user()? {
                        search_literally = true;
                    } else {
                        print!("Are you sure you want to delete ALL entries? [y/n]: ");
                        std::io::stdout().flush()?;
                        if ask_user()? {
                            println!("Deleting all entries...");
                            reminder_list.wipe();
                            reminder_list.dump_to_file(&path)?;
                        }
                        return Ok(());
                    }
                }
                _ => {}
            }

            if search_literally {
                let to_delete_list = &reminder_list.find_reminders_by_fuzzy_entry(&query);
                if to_delete_list.is_empty() {
                    println!("No reminders found matching query: {}", query);
                    return Ok(());
                }
                if to_delete_list.len() > 1 {
                    println!("Multiple matches found!");
                }
                let mut indices_to_delete: Vec<usize> = Vec::new();
                for (index, reminder) in to_delete_list {
                    println!("Do you want to delete this reminder?");
                    println!("  Content: {}", reminder.entry);
                    println!("  At:      {}", reminder.timestamp.prettify());
                    print!("\n[y/n]: ");
                    std::io::stdout().flush()?;
                    if ask_user()? {
                        indices_to_delete.push(*index);
                    }
                }
                indices_to_delete.sort_unstable_by(|a, b| b.cmp(a));
                for index in indices_to_delete {
                    reminder_list.delete_reminder_by_index(index);
                }
                reminder_list.dump_to_file(&path)?;
            }
        }
    }

    Ok(())
}
