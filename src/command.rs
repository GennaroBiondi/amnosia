use anyhow::{bail, Result};
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
    GetReminderPath,
}

impl Command {
    pub fn new_from_args(flags: Vec<String>) -> Result<Self> {
        use std::env;

        let arguments: Vec<String> = env::args().skip(1).collect();
        let command_str = arguments.get(0).map(String::as_str);

        match command_str {
            None | Some("remind") => Ok(Self::Remind(RemindInfo {
                include_date: flags.iter().any(|f| f == "-d" || f == "--include-dates"),
            })),
            Some("mind") => {
                let entry = arguments.into_iter().skip(1).collect::<Vec<_>>().join(" ");

                if entry.is_empty() {
                    bail!("Entry can't be empty!");
                }

                if entry.contains('|') {
                    bail!("Entry can't contain '|' character!");
                }

                Ok(Self::Mind(MindInfo {
                    entry,
                    timestamp: UnixTimestamp::now(),
                }))
            }
            Some("get_reminder_path") => Ok(Self::GetReminderPath),
            Some(s) => bail!("Argument {} is not a valid command!", s),
        }
    }
}
