use anyhow::Result;
use command::Command;

use reminder::{get_reminder_path, ReminderEntries, ReminderEntry};

mod command;
mod other_commands;
mod reminder;

fn check_for_instant_flags(flags: &[String]) -> bool {
    for flag in flags {
        match flag.as_str() {
            "--help" | "-h" => {
                other_commands::help();
                return true;
            }
            "--version" | "-v" => {
                other_commands::get_version();
                return true;
            }
            _ => {}
        }
    }
    false
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flags: Vec<String> = args
        .iter()
        .filter(|e| e.starts_with('-'))
        .cloned()
        .collect();

    if check_for_instant_flags(&flags) {
        return Ok(());
    }

    let command: Command = Command::new_from_args(args)?;
    let reminder_path = get_reminder_path()?;

    match command {
        Command::Remind(remind_info) => {
            let reminder_entries = ReminderEntries::from_file(&reminder_path)?;
            reminder_entries.list(remind_info.include_date);
            Ok(())
        }

        Command::Demind(demind_info) => Ok(()),

        Command::Mind(mind_info) => {
            let reminder_entry = ReminderEntry {
                content: mind_info.entry,
                timestamp: mind_info.timestamp,
            };

            reminder_entry.dump_to_file(&reminder_path)?;
            Ok(())
        }

        Command::GetReminderPath => {
            println!("{}", reminder_path.display());
            Ok(())
        }
    }
}
