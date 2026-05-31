use anyhow::{Result, bail};
use newtype::UnixTimestamp;
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub struct TodoEntry {
    pub content: String,
    pub timestamp: UnixTimestamp,
}

pub struct TodoEntries {
    pub entries: Vec<TodoEntry>,
}

fn handle_option<T>(opt: Option<T>) -> Result<T> {
    match opt {
        Some(x) => Ok(x),
        None => bail!("Todo Entries corrupted!"),
    }
}

pub fn get_todo_path() -> Result<PathBuf> {
    use dirs_next::home_dir;
    let base = home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    let todo_path = base.join(".local/share/amnosia/entries.txt");

    if !todo_path.exists() {
        println!(
            "Todo file not found. Creating one at {}",
            todo_path.display()
        );
        let parent = todo_path
            .parent()
            .ok_or_else(|| anyhow::anyhow!("Failed to get parent of {}", todo_path.display()))?;
        fs::create_dir_all(parent)?;
        fs::File::create(&todo_path)?;
    }

    if !todo_path.is_file() {
        bail!("Todo file is not a file. Please replace it with a text file.");
    }

    return Ok(todo_path);
}

impl TodoEntries {
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

            entries.push(TodoEntry { content, timestamp });
        }

        Ok(TodoEntries { entries })
    }

    pub fn list(&self, include_dates: bool) {
        if self.entries.is_empty() {
            return;
        }

        self.entries.iter().for_each(|e| {
            if include_dates {
                println!("[{}]: {}", e.timestamp.prettify(), e.content);
            } else {
                println!("{}", e.content);
            }
        });
    }
}

impl TodoEntry {
    pub fn dump_to_file(&self, path: &PathBuf) -> Result<()> {
        use std::io::Write;
        let mut file = OpenOptions::new().append(true).open(path)?;

        writeln!(file, "{}|{}", self.content, self.timestamp.0)?;

        Ok(())
    }
}
