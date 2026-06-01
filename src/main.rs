use anyhow::Result;
use command::Command;

use crate::reminder::{get_reminder_path, ReminderEntries, ReminderEntry};

mod command;
mod reminder;

fn help() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
    const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

    println!("{} {}. By {}", NAME, VERSION, AUTHORS);
    println!("{}", DESCRIPTION);
    println!();
    println!(
        "{} mind [STRING]:             - add entry to reminders.txt",
        NAME
    );
    println!(
        "{} remind [OPTIONS]:          - read all entries from reminders.txt",
        NAME
    );
    println!(
        "  --include-dates | -d:       - also read the timestamps (when the entries were added)"
    );
    println!(
        "{} get_reminder_path:         - get the location of reminders.txt",
        NAME
    );
    println!("  (default is '~/.local/share/amnosia/reminders.txt')");
}

fn main() -> Result<()> {
    let flags: Vec<String> = std::env::args().filter(|e| e.starts_with('-')).collect();

    if flags.iter().any(|f| f == "--help" || f == "-h") {
        help();
        return Ok(());
    }

    let command: Command = Command::new_from_args(flags)?;
    let reminder_path = get_reminder_path()?;

    match command {
        Command::Remind(remind_info) => {
            let reminder_entries = ReminderEntries::from_file(&reminder_path)?;
            reminder_entries.list(remind_info.include_date);
            Ok(())
        }

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
