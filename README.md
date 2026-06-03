# Amnosia
<img width="752" height="527" alt="image" src="https://github.com/user-attachments/assets/e8b348cf-6e8f-4fa0-b87e-053f1748059e" />

A simple CLI tool to store and manage reminders in your filesystem.

## What is Amnosia?
Amnosia is a minimal command-line reminder tool that stores entries locally.
Designed to be fast, simple, and always in your way — in a good sense.

## Installation

### Option 1: Cargo install (recommended)


```bash
git clone https://github.com/GennaroBiondi/amnosia
cd amnosia
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

Make sure `~/.local/bin` (or wherever you're moving the binary) is in your PATH.

## Usage
| Command | Info |
| --- | --- |
| `amnosia mind "ENTRY"` | Add an entry |
| `amnosia remind` | Lists all reminders |
| `amnosia demind` | Remove an entry by fuzzy searching through all the entries |
| `amnosia --help` | Display more detailed info about the program |

### Run on terminal start
Amnosia is designed to be invoked at the start of every terminal session,
so your reminders are always visible without you having to think about it.

Add this to your `.zshrc` or `.bashrc`:
```bash
amnosia remind -n 7
```

## Performance

Benchmarked with Criterion.rs on a 100k entry file:

| Operation | Time |
| --- | --- |
| Fuzzy search 10k reminders | ~177 µs |
| Fuzzy search 100k reminders | ~188 µs |
| Exact search 10k reminders | ~3 µs |
| Exact search 100k reminders | ~3 µs |
