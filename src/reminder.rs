use anyhow::Result;
use std::{fs::OpenOptions, path::Path};

#[derive(Debug)]
pub struct Reminder {
    pub entry: String,
    pub timestamp: newtype::UnixTimestamp,
}

impl Reminder {
    pub fn append_to_file(&self, path: &Path) -> Result<()> {
        use std::io::Write;

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)?;

        writeln!(file, "{}|{}", self.entry, self.timestamp.0)?;

        Ok(())
    }
}
