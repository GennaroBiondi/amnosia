use anyhow::{Result, bail};
use newtype::UnixTimestamp;

pub struct MindInfo {
    pub entry: String,
    pub timestamp: UnixTimestamp,
}

pub struct RemindInfo {
    pub include_date: bool,
}

pub enum Command {
    Remind(RemindInfo),
    Mind(MindInfo),
    GetTodoPath,
}

impl Command {
    pub fn new_from_args() -> Result<Self> {
        use std::env;

        let arguments: Vec<String> = env::args().skip(1).collect();
        let command_str = arguments.get(0).map(String::as_str);
        match command_str {
            None | Some("remind") => {
                let flags: Vec<String> = arguments
                    .iter()
                    .filter(|e| e.starts_with('-'))
                    .cloned()
                    .collect();

                Ok(Self::Remind(RemindInfo {
                    include_date: (flags.contains(&"-d".to_string())
                        || flags.contains(&"--include-dates".to_string())),
                }))
            }
            Some("mind") => {
                let entry = arguments.into_iter().skip(1).collect::<Vec<_>>().join(" ");

                if entry.is_empty() {
                    bail!("Entry can't be empty!");
                }

                if entry.contains("|") {
                    bail!("Entry can't contain an '|' character!");
                }

                let now = UnixTimestamp::now();

                Ok(Self::Mind(MindInfo {
                    entry,
                    timestamp: now,
                }))
            }
            Some("get_todo") => Ok(Self::GetTodoPath),
            Some(s) => bail!("Argument {} is not a valid command!", s),
        }
    }
}
