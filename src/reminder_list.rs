use crate::reminder::Reminder;
use anyhow::Result;
use newtype::UnixTimestamp;
use std::path::Path;

struct ReminderList(Vec<Reminder>);

impl ReminderList {
    pub fn from_file(path: &Path) -> Result<Self> {
        use std::io::{BufRead, BufReader};
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut reminders: Vec<Reminder> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let reminder = match line.find('|') {
                Some(s) => Reminder {
                    entry: line[..s].to_string(),
                    timestamp: Some(UnixTimestamp(line[s + 1..].parse::<i64>()?)),
                },
                None => Reminder {
                    entry: line.to_string(),
                    timestamp: None,
                },
            };
            reminders.push(reminder);
        }

        Ok(ReminderList(reminders))
    }

    pub fn get_vec(&self) -> &Vec<Reminder> {
        self.0.as_ref()
    }

    pub fn dump_to_file(&self, path: &Path) -> Result<()> {
        use std::io::{BufWriter, Write};
        let file = std::fs::File::create(path)?;
        let mut writer = BufWriter::new(file);
        for entry in self.get_vec() {
            match &entry.timestamp {
                Some(timestamp) => writeln!(writer, "{}|{}", entry.entry, timestamp.0)?,
                None => writeln!(writer, "{}", entry.entry)?,
            }
        }
        writer.flush()?;
        Ok(())
    }
}
