# Minigrep

A small command-line tool written in Rust to search for lines matching a pattern in a file.
Supports:
- Case-insensitive search (`-i`)
- Line numbers (`-n`)
- Whole word matching (`-w`)
- Inverted matches (`-v`)

## Installation

1. Clone the repo:
    ´´´bash
    git clone https://github.com/yourusername/minigrep
    cd minigrep
    ´´´

2. Build it:
    ´´´bash
    cargo build --release
    ´´´
3. Add the binary to PATH:
    Either add target/release to PATH or move target/release/minigrep.exe to a folder that is in PATH

### Usage

minigrep [OPTIONS] <PATTERN> <FILE>

Options:
- -i: Ignore case
- -n: Show line numbers
- -w: Match whole words only
- -v: Invert match (show lines that do not match)

minigrep to poem.txt
minigrep -i to poem.txt
minigrep "to be" poem.txt
minigrep -w -n "To" poem.txt


### How It Works