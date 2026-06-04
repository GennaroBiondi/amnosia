use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn gen_file(path: &Path, entries: usize) {
    println!("Generating content for file at path: {}", path.display());

    let words = [
        "meeting",
        "doctor",
        "shopping",
        "call mom",
        "fix bug",
        "deploy",
        "review PR",
    ];
    let mut file = File::create(path).unwrap();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    for i in 0..entries {
        let entry = words[i % words.len()];
        let ts = now - (entries - i) as i64 * 60;
        writeln!(file, "{} {}|{}", entry, i, ts).unwrap();
    }
}

fn main() {
    std::fs::create_dir_all("benches/testing_data").unwrap();
    let test_10k = PathBuf::from("benches/testing_data/file_10k.test");
    let test_100k = PathBuf::from("benches/testing_data/file_100k.test");

    gen_file(&test_10k, 10_000);
    gen_file(&test_100k, 100_000);
}
