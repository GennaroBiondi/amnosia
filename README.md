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
| Command | Info |
| --- | --- |
| `amnosia mind "ENTRY"` | Add an entry |
| `amnosia remind` | Lists all reminders |
| `amnosia get_reminder_path` | Display reminder file path |
| `amnosia --help` | Display more detailed info about the program |

## Auto-run on shell start

To display reminders every time you open a terminal:

```bash
amnosia remind
```

Add it to your shell config:

```
- `.zshrc`
- `.bashrc`
```
