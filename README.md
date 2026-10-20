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

Clone the repository and build the release executable:

```bash
git clone https://github.com/codeyahya/real-cli.git
cd real-cli
cargo build --release
```

The executable is created at `target\release\real.exe`.

## Add the executable to PATH

To run `real` from any terminal, add the executable folder to your Windows
PATH:

1. Copy the full path to the project's `target\release` folder.
2. Open Windows Search and enter `Edit the system environment variables`.
3. Click `Environment Variables`.
4. Under `User variables`, select `Path` and click `Edit`.
5. Click `New`, paste the `target\release` path, and confirm all dialogs.
6. Open a new terminal.

You can now run the program directly:

```bash
real --help
```

## Usage

Run the program with the `real` command:

```bash
real <command> [arguments]
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
real file notes.txt
real write notes.txt "My first note"
real append notes.txt "Another line"
real read notes.txt
real ls
real del notes.txt
real tic-tac-toe
```

## Build

To create an optimized executable:

```bash
cargo build --release
```

The executable is created in `target\release\real.exe`.

## License

No license has been selected for this project yet.
