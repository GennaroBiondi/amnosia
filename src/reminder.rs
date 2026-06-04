use anyhow::Result;
use std::{fs::OpenOptions, path::Path};

pub enum ReminderType {
    NormalReminder {
        entry: String,
        timestamp: newtype::UnixTimestamp,
    },
    TimedReminder {
        entry: String,
        timestamp: newtype::UnixTimestamp,
        end: newtype::UnixTimestamp,
    },
}

impl std::fmt::Display for ReminderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReminderType::NormalReminder { entry, timestamp } => {
                writeln!(f, "[{}]: {}", timestamp.prettify(), entry)
            }
            ReminderType::TimedReminder {
                entry,
                timestamp,
                end,
            } => {
                if timestamp.is_due() {
                    writeln!(
                        f,
                        "[{}]: {}... Due! ({})",
                        timestamp.prettify(),
                        entry,
                        end.prettify()
                    )
                } else {
                    writeln!(
                        f,
                        "[{}]: {}... due on: {}",
                        timestamp.prettify(),
                        entry,
                        end.prettify()
                    )
                }
            }
        }
    }
}

impl ReminderType {
    pub fn get_content(&self) -> &str {
        match self {
            Self::NormalReminder { entry, .. } | Self::TimedReminder { entry, .. } => entry,
        }
    }

    pub fn get_timestamp(&self) -> &newtype::UnixTimestamp {
        match self {
            Self::NormalReminder { timestamp, .. } | Self::TimedReminder { timestamp, .. } => {
                timestamp
            }
        }
    }

    pub fn write_to(&self, dest: &mut impl std::io::Write) -> Result<()> {
        match self {
            ReminderType::NormalReminder { entry, timestamp } => {
                writeln!(dest, "{}|{}", entry, timestamp.0)?;
            }
            ReminderType::TimedReminder {
                entry,
                timestamp,
                end,
            } => {
                writeln!(dest, "{}|{}|{}", entry, timestamp.0, end.0)?;
            }
        }
        Ok(())
    }

    pub fn append_to_file(&self, path: &Path) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)?;
        self.write_to(&mut file)
    }

    pub fn display(&self, show_dates: bool) {
        let output = match self {
            ReminderType::NormalReminder { entry, timestamp } => {
                if show_dates {
                    format!("[{}]: {}", timestamp.prettify(), entry)
                } else {
                    entry.clone()
                }
            }
            ReminderType::TimedReminder {
                entry,
                timestamp,
                end,
            } => {
                let due = timestamp.is_due();
                if show_dates {
                    match due {
                        true => format!(
                            "[{}]: {}... Due! ({})",
                            timestamp.prettify(),
                            entry,
                            end.prettify()
                        ),
                        false => format!(
                            "[{}]: {}... due on: {}",
                            timestamp.prettify(),
                            entry,
                            end.prettify()
                        ),
                    }
                } else {
                    match due {
                        true => format!("{}... Due!", entry),
                        false => format!("{}... not due!", entry),
                    }
                }
            }
        };
        println!("{}", output);
    }
}
