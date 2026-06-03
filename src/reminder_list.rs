use crate::reminder::Reminder;
use anyhow::{bail, Result};
use newtype::UnixTimestamp;
use std::collections::HashMap;
use std::{ops::Index, path::Path, vec};

pub struct ReminderList(Vec<Reminder>);

impl ReminderList {
    pub fn from_file(path: &Path) -> Result<Self> {
        use std::io::{BufRead, BufReader};
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut reminders: Vec<Reminder> = Vec::new();

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let reminder = match line.find('|') {
                Some(s) => Reminder {
                    entry: line[..s].to_string(),
                    timestamp: UnixTimestamp(line[s + 1..].parse::<i64>()?),
                },
                None => bail!("Malformed entry! Line: {}\n{}", i, line),
            };
            reminders.push(reminder);
        }

        Ok(ReminderList(reminders))
    }

    pub fn get_vec(&self) -> &Vec<Reminder> {
        self.0.as_ref()
    }

    pub fn find_reminders_by_fuzzy_entry(&self, query: &str) -> HashMap<usize, &Reminder> {
        self.get_vec()
            .iter()
            .enumerate()
            .filter(|(_, x)| x.entry.contains(query))
            .collect()
    }

    pub fn wipe(&mut self) {
        self.0 = Vec::new();
    }

    pub fn delete_reminder_by_index(&mut self, index: usize) -> Reminder {
        let vec = &mut self.0;
        vec.remove(index)
    }

    pub fn dump_to_file(&self, path: &Path) -> Result<()> {
        use std::io::{BufWriter, Write};
        let file = std::fs::File::create(path)?;
        let mut writer = BufWriter::new(file);
        for entry in self.get_vec() {
            writeln!(writer, "{}|{}", entry.entry, entry.timestamp.0)?
        }
        writer.flush()?;
        Ok(())
    }
}
