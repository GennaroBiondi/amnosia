use anyhow::Result;
use command::Command;

use crate::todo::{TodoEntries, TodoEntry, get_todo_path};

mod command;
mod todo;

fn main() -> Result<()> {
    let command: Command = Command::new_from_args()?;
    let todo_path = get_todo_path()?;

    match command {
        Command::Remind(remind_info) => {
            let todo_entries = TodoEntries::from_file(&todo_path)?;
            todo_entries.list(remind_info.include_date);

            Ok(())
        }

        Command::Mind(mind_info) => {
            let todo_entry = TodoEntry {
                content: mind_info.entry,
                timestamp: mind_info.timestamp,
            };

            todo_entry.dump_to_file(&todo_path)?;
            Ok(())
        }
        Command::GetTodoPath => {
            println!("{}", todo_path.display());
            Ok(())
        }
    }
}
