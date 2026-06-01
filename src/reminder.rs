use anyhow::{Result, bail};
use newtype::UnixTimestamp;
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub struct ReminderEntry {
    pub content: String,
    pub timestamp: UnixTimestamp,
}

pub struct ReminderEntries {
    pub entries: Vec<ReminderEntry>,
}

fn handle_option<T>(opt: Option<T>) -> Result<T> {
    match opt {
        Some(x) => Ok(x),
        None => bail!("Reminder entries corrupted!"),
    }
}

pub fn get_reminder_path() -> Result<PathBuf> {
    use dirs_next::home_dir;

    let base = home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    let reminder_path = base.join(".local/share/amnosia/reminders.txt");

    if !reminder_path.exists() {
        println!(
            "Reminder file not found. Creating one at {}",
            reminder_path.display()
        );

        let parent = reminder_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Failed to get parent of {}", reminder_path.display()))?;

        fs::create_dir_all(parent)?;
        fs::File::create(&reminder_path)?;
    }

    if !reminder_path.is_file() {
        bail!("Reminder file is not a file. Replace it with a text file.");
    }

    Ok(reminder_path)
}

impl ReminderEntries {
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);

        let mut entries = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let info: Vec<&str> = line.split('|').collect();

            let content = handle_option(info.get(0))?.to_string();
            let ts_str = handle_option(info.get(1))?;
            let timestamp = UnixTimestamp(ts_str.parse()?);

            entries.push(ReminderEntry { content, timestamp });
        }

        Ok(ReminderEntries { entries })
    }

    pub fn list(&self, include_dates: bool) {
        if self.entries.is_empty() {
            return;
        }

        for e in &self.entries {
            if include_dates {
                println!("[{}]: {}", e.timestamp.prettify(), e.content);
            } else {
                println!("{}", e.content);
            }
        }
    }
}

impl ReminderEntry {
    pub fn dump_to_file(&self, path: &PathBuf) -> Result<()> {
        use std::io::Write;

        let mut file = OpenOptions::new().append(true).open(path)?;
        writeln!(file, "{}|{}", self.content, self.timestamp.0)?;
        Ok(())
    }
}
