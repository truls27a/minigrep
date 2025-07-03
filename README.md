# Minigrep

A small command-line tool written in Rust to search for lines matching a pattern in a file.
Supports:
- Case-insensitive search (`-i`)
- Line numbers (`-n`)
- Whole word matching (`-w`)
- Inverted matches (`-v`)

## Installation

1. Clone the repo:
    ```bash
    git clone https://github.com/yourusername/minigrep
    cd minigrep
    ```

2. Build it:
    ```bash
    cargo build --release
    ```
3. Add the binary to PATH:
    Either add target/release to PATH or move target/release/minigrep.exe to a folder that is in PATH

### Usage

`minigrep [OPTIONS] <PATTERN> <FILE>`

Options:
- -i: Ignore case
- -n: Show line numbers
- -w: Match whole words only
- -v: Invert match (show lines that do not match)

minigrep to poem.txt
minigrep -i to poem.txt
minigrep "to be" poem.txt
minigrep -w -n error src/main.rs


### How It Works

#### Overview

When the program starts, it reads the command line arguments and builds a config. Then, it reads the input file, performs the search based on the config, and highlights the text that matches the query. Finally, the result is printed out.

#### Step-by-Step Flow

1. `main.rs`
    - Collects CLI arguments
    - Calls `Config::build` to parse them
    - Calls `runner::run` to start the app logic

2. `config.rs`
    - Defines `Config` struct
    - Tries to parse CLI arguments, flags, and env variables (`-i`, `-n`, etc.) to build `Config` struct

3. `runner.rs`
    - Runs the app logic
    - Reads the file content
    - Constructs `Content` struct with file content
    - Calls search, highlight, and finally display on the `Content` struct

4. `model.rs`
    - Defines `Line` and `Content` structs

5. `search.rs`
    - Implements search logic on `Line` and by extension `Content`

6. `highlight.rs`
    - Highlights matched words or text using ANSI color codes

7. `display.rs`
    - Handles output format and printing to terminal

### Design Decisions

#### Modular Architecture

I split the logic across multiple files (model.rs, search.rs, highlight.rs, display.rs) instead of keeping it all in main.rs.
This keeps responsibilities separate which makes the code easier to test, read, and understand.

#### Custom Argument Parsing

Instead of using a library like `clap`, I manually parse `env::args()`. This is a deliberate choice as I want the application to be simple, clean, and efficient by not relying on third party crates.


#### Manual Highlighting with ANSI Codes

Again, instead of using a crate like `colored` I implement my own custom logic for coloring text in the terminal. This is on purpose as it is not necessary to import a whole external library just to color text in the terminal.


#### 1-Based Line Numbering

Line index starts at 1 instead of 0 since that is the convention when viewing files.