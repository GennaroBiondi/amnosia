const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub fn help() {
    println!("{} {}. By {}", NAME, VERSION, AUTHORS);
    println!("{}", DESCRIPTION);
    println!();
    println!(
        "{} mind [STRING]:             - add entry to reminders.txt",
        NAME
    );
    println!(
        "{} remind [OPTIONS]:          - read all entries from reminders.txt",
        NAME
    );
    println!(
        "  --include-dates | -d:       - also read the timestamps (when the entries were added)"
    );
    println!(
        "{} get_reminder_path:         - get the location of reminders.txt",
        NAME
    );
    println!("  (default is '~/.local/share/amnosia/reminders.txt')");
}

pub fn get_version() {
    println!("{}", VERSION);
}
