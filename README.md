# Amnosia

A simple CLI tool to store and manage reminders in your filesystem.

## What is Amnosia?

Amnosia is a minimal command-line reminder tool that stores entries locally.  
It is designed to help you "replace amnesia" by keeping your thoughts persistent across terminal sessions.

## Installation

### Option 1: Cargo install (recommended)

```bash
cargo install --path .
```

### Option 2: Build manually

```bash
git clone https://github.com/GennaroBiondi/amnosia
cd amnosia
cargo build --release
```

Then move the binary:

```bash
cp target/release/amnosia ~/.local/bin/
```

Make sure `~/.local/bin` is in your PATH.

## Usage

### Add a reminder

```bash
amnosia mind "your reminder text"
```

### List reminders

```bash
amnosia remind
```

Options:

```text
-d, --include-dates    Show timestamps
```

### Show reminder file path

```bash
amnosia get_reminder_path
```

(Default: `~/.local/share/amnosia/reminders.txt`)

## Auto-run on shell start

To display reminders every time you open a terminal:

```bash
amnosia remind
```

Add it to your shell config:

- `.zshrc`
- `.bashrc`
```
