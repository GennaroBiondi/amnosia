use anyhow::{anyhow, bail, Result};
use clap::Parser;
use commands::{Cli, Commands};
use newtype::UnixTimestamp;
use reminder::ReminderType;
use reminder_list::ReminderList;
use std::{fs::File, io::Write, path::Path, path::PathBuf};

mod commands;
mod reminder;
mod reminder_list;

fn ask_user() -> Result<bool> {
    use std::io::stdin;

    let mut response = String::new();

    match stdin().read_line(&mut response) {
        Ok(_n) => match response.trim().to_lowercase().as_str() {
            "yes" | "y" | "yeah" | "positive" | "sure" | "absolutely" | "why_not" | "you_bet" => {
                Ok(true)
            }
            _ => Ok(false),
        },
        Err(error) => bail!("Error reading input: {error}"),
    }
}

fn expand_path(s: &str) -> Result<PathBuf> {
    let s_lower = s.to_lowercase();
    let prefixes = ["~/", "~\\", "%userprofile%/", "%userprofile%\\"];

    for prefix in prefixes {
        if s_lower.starts_with(prefix) {
            let home = dirs_next::home_dir().ok_or_else(|| anyhow!("Home directory not found"))?;
            let rest = &s[prefix.len()..];
            return Ok(home.join(rest));
        }
    }

    if s == "~" || s == "%USERPROFILE%" {
        return dirs_next::home_dir().ok_or_else(|| anyhow!("Home directory not found"));
    }

    Ok(PathBuf::from(s))
}

// Thanks @phitazero
fn is_a_valid_reminders_file(path: &Path) -> Result<()> {
    if !path.exists() {
        let parent_dir = path.parent().ok_or_else(|| {
            anyhow!(
                "Provided path {} has no parent and is likely invalid",
                path.display()
            )
        })?;

        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir)?;
        }

        File::create(path)?;
    }

    if !path.is_file() {
        bail!("Reminders file isn't a file: {}", path.display());
    }

    Ok(())
}

fn get_reminders_file_path() -> Result<PathBuf> {
    let base =
        dirs_next::data_dir().ok_or_else(|| anyhow::anyhow!("No data directory available"))?;

    Ok(base.join("amnosia").join("reminders.txt"))
}

fn string_to_unix_timestamp(s: &str) -> Option<UnixTimestamp> {
    let s = s.trim();

    match s {
        "today" => return Some(UnixTimestamp::now()),
        "tomorrow" => return Some(UnixTimestamp(UnixTimestamp::now().0 + 86400)),
        "yesterday" => return Some(UnixTimestamp(UnixTimestamp::now().0 - 86400)),
        _ => {}
    }

    let (sign, rest) = if let Some(r) = s.strip_prefix('-') {
        (-1i64, r)
    } else if let Some(r) = s.strip_prefix('+') {
        (1i64, s.strip_prefix('+').unwrap_or(r))
    } else {
        (1i64, s)
    };

    let unit_start = rest.find(|c: char| c.is_alphabetic())?;
    let (num_str, unit) = rest.split_at(unit_start);
    let n: i64 = num_str.parse().ok()?;

    let seconds = match unit {
        "m" | "min" | "mins" | "minute" | "minutes" => 60,
        "h" | "hr" | "hrs" | "hour" | "hours" => 3600,
        "d" | "day" | "days" => 86400,
        "w" | "week" | "weeks" => 86400 * 7,
        _ => return None,
    };

    Some(UnixTimestamp(UnixTimestamp::now().0 + sign * n * seconds))
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let default_path = get_reminders_file_path()?;

    let path = match cli.reminder_path {
        Some(p) => expand_path(&p.to_string_lossy())?,
        None => default_path,
    };

    is_a_valid_reminders_file(&path)?;

    match cli.command {
        Commands::Mind { entry, deadline } => {
            let timestamp = UnixTimestamp::now();
            let reminder_entry = match deadline {
                Some(dl) => {
                    let end = string_to_unix_timestamp(&dl)
                        .ok_or_else(|| anyhow::anyhow!("Invalid deadline: {dl}"))?;
                    ReminderType::TimedReminder {
                        entry,
                        timestamp,
                        end,
                    }
                }
                None => ReminderType::NormalReminder { entry, timestamp },
            };
            reminder_entry.append_to_file(&path)?;
        }
        Commands::Remind {
            include_dates,
            number_limit,
        } => {
            let reminder_list = ReminderList::from_file(&path)?;
            let vec = reminder_list.get_vec();
            let iter = match number_limit {
                Some(n) => vec.iter().rev().take(n).collect::<Vec<_>>(),
                None => vec.iter().collect::<Vec<_>>(),
            };
            for reminder in iter {
                reminder.display(include_dates);
            }
        }
        Commands::Demind { query } => {
            let mut reminder_list = ReminderList::from_file(&path)?;

            let (search_str, fuzzy) = match (query.query, query.exact_query) {
                (Some(q), _) => (q, true),
                (_, Some(e)) => (e, false),
                _ => unreachable!(),
            };

            let mut search_literally = true;
            if fuzzy {
                match search_str.to_lowercase().as_str() {
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
            }

            if search_literally {
                let to_delete_list = if fuzzy {
                    reminder_list.find_reminders_by_fuzzy_entry(&search_str)
                } else {
                    reminder_list.find_reminders_by_exact_entry(&search_str)
                };
                if to_delete_list.is_empty() {
                    println!("No reminders found matching query: {}", search_str);
                    return Ok(());
                }
                if to_delete_list.len() > 1 {
                    println!("Multiple matches found!");
                }
                let mut indices_to_delete: Vec<usize> = Vec::new();
                for (index, reminder) in &to_delete_list {
                    println!("Do you want to delete this reminder?");
                    println!("  Content: {}", reminder.get_content());
                    println!("  At:      {}", reminder.get_timestamp().prettify());
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
