# Real CLI

Real CLI is a small command-line tool built with Rust. It helps you manage
files and directories from the terminal and includes a simple Tic-Tac-Toe
game.

## Features

- Create files and directories
- Read, write, and append text files
- List directory contents
- Rename and delete files or directories
- Open the Ace editor from the terminal
- Play Tic-Tac-Toe against a simple random-move computer player
- Use colored output and interactive delete confirmation

## Requirements

- Rust and Cargo
- Windows, for full support of the current file size display code
- Ace editor in your `PATH` if you want to use the `ace` command

## Installation

Clone the repository and build the project:

```bash
git clone <repository-url>
cd 7_RealCLITool
cargo build --release
```

## Usage

Run the program with Cargo:

```bash
cargo run -- <command> [arguments]
```

Available commands:

| Command | Description |
| --- | --- |
| `file <path>` | Create a file |
| `dir <path>` | Create a directory and its parent directories |
| `read <path>` | Display a file |
| `write <path> <content>` | Replace a file with new content |
| `append <path> <content>` | Add content to a file |
| `ls [path]` | List files and directories; uses the current directory by default |
| `ren <new-name> <path>` | Rename a file |
| `del <path>` | Delete a file or directory after confirmation |
| `ace` | Open the Ace editor |
| `tic-tac-toe` | Start the Tic-Tac-Toe game |
| `build [--release]` | Show the selected build mode |

Examples:

```bash
cargo run -- file notes.txt
cargo run -- write notes.txt "My first note"
cargo run -- append notes.txt "Another line"
cargo run -- read notes.txt
cargo run -- ls
cargo run -- del notes.txt
cargo run -- tic-tac-toe
```

## Build

To create an optimized executable:

```bash
cargo build --release
```

The executable is created in `target/release`.

## License

No license has been selected for this project yet.
