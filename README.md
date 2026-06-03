# Amnosia
[![CI](https://github.com/GennaroBiondi/amnosia/actions/workflows/release.yml/badge.svg)](https://github.com/GennaroBiondi/amnosia/actions/workflows/release.yml)
[![License](https://img.shields.io/github/license/GennaroBiondi/amnosia)](LICENSE)
[![Latest Release](https://img.shields.io/github/v/release/GennaroBiondi/amnosia)](https://github.com/GennaroBiondi/amnosia/releases)
![Rust](https://img.shields.io/badge/built%20with-Rust-orange?logo=rust)

<img width="690" height="592" alt="image" src="https://github.com/user-attachments/assets/f431257d-0670-41fc-9234-81fd0f9595f8" />

*The reminder tool that gets in your face.*

## What is Amnosia?
You don't forget your reminders because you're disorganized,
you forget them because your todo app doesn't get in your face.
Amnosia does. It shows up every time you open a terminal, so your
reminders are always visible without you having to think about it.

## Installation

Clone the git repo and use cargo to install it in your path.
```bash
git clone https://github.com/GennaroBiondi/amnosia
cd amnosia
cargo install --path .
```

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

Benchmarked with Criterion.rs on various sized files:

| Operation | Time |
| --- | --- |
| Fuzzy search 10k reminders | ~177 µs |
| Fuzzy search 100k reminders | ~188 µs |
| Exact search 10k reminders | ~3 µs |
| Exact search 100k reminders | ~3 µs |
