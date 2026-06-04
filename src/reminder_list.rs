use crate::reminder::ReminderType;
use anyhow::{bail, Result};
use newtype::UnixTimestamp;
use std::path::Path;

pub struct ReminderList(Vec<ReminderType>);

fn get_all_positions(s: &str, c: char) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();

    for (i, character) in s.chars().enumerate() {
        if character == c {
            indices.push(i);
        }
    }

    indices
}

impl ReminderList {
    pub fn from_file(path: &Path) -> Result<Self> {
        use std::io::{BufRead, BufReader};
        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut reminders: Vec<ReminderType> = Vec::new();

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            let divisors = get_all_positions(&line, '|');

            let reminder = match divisors.len() {
                1 => {
                    let entry_str = &line[..divisors[0]];
                    let timestamp_str = &line[divisors[0] + 1..];

                    let entry = entry_str.to_string();

                    let timestamp_int = timestamp_str.parse::<i64>()?;
                    let timestamp = UnixTimestamp(timestamp_int);

                    ReminderType::NormalReminder { entry, timestamp }
                }
                2 => {
                    let entry_str = &line[..divisors[0]];
                    let timestamp_str = &line[divisors[0] + 1..divisors[1]];
                    let timestamp_end_str = &line[divisors[1] + 1..];

                    let entry = entry_str.to_string();

                    let timestamp_int = timestamp_str.parse::<i64>()?;
                    let timestamp = UnixTimestamp(timestamp_int);

                    let timestamp_end_int = timestamp_end_str.parse::<i64>()?;
                    let end = UnixTimestamp(timestamp_end_int);

                    ReminderType::TimedReminder {
                        entry,
                        timestamp,
                        end,
                    }
                }
                _ => bail!("Malformed entry on line {i}: expected 1 or 2 '|' separators"),
            };

            reminders.push(reminder);
        }

        Ok(ReminderList(reminders))
    }

    pub fn get_vec(&self) -> &[ReminderType] {
        self.0.as_ref()
    }

    pub fn find_reminders_by_fuzzy_entry(&self, query: &str) -> Vec<(usize, &ReminderType)> {
        self.get_vec()
            .iter()
            .enumerate()
            .filter(|(_, x)| x.get_content().contains(&query))
            .collect()
    }

    pub fn find_reminders_by_exact_entry(&self, query: &str) -> Vec<(usize, &ReminderType)> {
        self.get_vec()
            .iter()
            .enumerate()
            .filter(|(_, x)| x.get_content() == query)
            .collect()
    }

    pub fn wipe(&mut self) {
        self.0 = Vec::new();
    }

    pub fn delete_reminder_by_index(&mut self, index: usize) -> ReminderType {
        let vec = &mut self.0;
        vec.remove(index)
    }

    pub fn dump_to_file(&self, path: &Path) -> Result<()> {
        use std::io::{BufWriter, Write};
        let file = std::fs::File::create(path)?;
        let mut writer = BufWriter::new(file);
        for entry in self.get_vec() {
            entry.write_to(&mut writer)?;
        }
        writer.flush()?;
        Ok(())
    }
}
