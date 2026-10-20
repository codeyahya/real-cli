use anyhow::{Context, Result};
use clap::builder::OsStr;
use clap::{Parser, Subcommand};
use console::style;
use core::time;
use dialoguer::{Confirm, Input};
use rand::Rng;
use std::fs::{
    File, OpenOptions, create_dir_all, read_dir, read_to_string, remove_dir_all, remove_file,
};
use std::io::{self, BufWriter, Write};
use std::os::windows::fs::MetadataExt;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "real", about = "Real CLI tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    File {
        path: PathBuf,
    },
    Dir {
        path: PathBuf,
    },
    Read {
        path: PathBuf,
    },
    Del {
        path: PathBuf,
    },
    Ren {
        new_name: String,
        path: PathBuf,
    },
    Ace,
    Write {
        path: PathBuf,
        content: String,
    },
    Append {
        path: PathBuf,
        content: String,
    },
    Ls {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    TicTacToe,
    Build {
        #[arg(short, long)]
        release: bool,
    },
}

fn create_file(path: &PathBuf) -> Result<()> {
    let _ = File::create(path).with_context(|| style("Could not create file").red())?;

    println!(
        "{}",
        style(format!(
            "File created successfully at {}",
            path.to_string_lossy().into_owned()
        ))
        .magenta()
    );
    Ok(())
}

fn create_dir(path: &PathBuf) -> Result<()> {
    println!("Path Dir: {}", path.to_string_lossy().into_owned());

    let _ = create_dir_all(path);
    Ok(())
}

fn read_file(path: &PathBuf) -> Result<()> {
    let content = read_to_string(path).with_context(|| style("No content found").red())?;
    println!("{}", content);
    Ok(())
}

fn delete(path: &PathBuf) -> Result<()> {
    let target = path;

    if Confirm::new()
        .with_prompt(format!("Confirm delete {} ? y/N", target.display()))
        .interact()?
    {
        if target.is_dir() {
            remove_dir_all(&target).with_context(|| {
                style(format!("Failed to delete directory {}", target.display()))
            })?;
        } else {
            remove_file(&target)
                .with_context(|| style(format!("Failed to delete file {}", target.display())))?;
        }
        let msg = format!("Confirmed: Deleted {}", target.display());
        println!("{}", style(msg).magenta());
    } else {
        println!("Canceled");
    }

    Ok(())
}

fn rename_file(path: &PathBuf, new_name: &String) -> Result<()> {
    let content =
        read_to_string(path).with_context(|| style("Error while reading file 1 content").red())?;

    let new_file =
        File::create(new_name).with_context(|| style("Error while creating file").red())?;
    let mut writer = BufWriter::new(new_file);

    writeln!(writer, "{}", content).with_context(|| style("Error while writing to file").red())?;
    remove_file(path).with_context(|| style("Error while removing file 1").red())?;

    println!("{}", style("Renamed file successfully !").magenta());

    Ok(())
}

fn open_ace() -> Result<()> {
    let _ = std::process::Command::new("ace").output()?;

    Ok(())
}

fn write_to_file(path: &PathBuf, content: &str, create: bool) -> Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(create)
        .append(!create)
        .open(path)?;

    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", content)?;

    println!("{}", style("Success !").magenta());

    Ok(())
}

fn list(path: &PathBuf) -> Result<()> {
    let files = read_dir(path).with_context(|| style("Error while reading dir").red())?;
    println!(
        "{}",
        style("Name                 Type                 Size").magenta()
    );
    println!(
        "{}",
        style("----                 ----                 ----").magenta()
    );
    for entry in files {
        if let Ok(file) = entry.map_err(|e| e.to_string()) {
            let file_name = file
                .file_name()
                .to_string_lossy()
                .into_owned()
                .replace(r#"""#, " ");
            let file_metadata = file.metadata();
            let mut file_size_in_u64: u64 = 0;
            let mut file_type: String = String::from("file");
            if let Ok(metadata) = file_metadata {
                if metadata.file_type().is_dir() {
                    file_type = String::from("dir");
                };
                file_size_in_u64 = metadata.file_size();
            }

            let file_size = file_size_in_u64.clone().to_string();
            let mut styles = style(format!("{:?}", file_name));

            if file.path().is_dir() {
                styles = style(format!("{:?}", file_name))
                    .bg(console::Color::Blue)
                    .white();
            }

            println!(
                // "{}                 {}                 {}",
                "{}",
                styles,
                // file_type,
                // if file_size != "0" {
                //     file_size
                // } else {
                //     String::new()
                // }
            );
        }
    }
    Ok(())
}

fn draw_board(moves: &Vec<String>) {
    println!("1         |2         |3         ");
    println!("          |          |          ");
    println!(
        "     {}    |    {}     |    {}     ",
        style(&moves[0]).red(),
        style(&moves[1]).red(),
        style(&moves[2]).red()
    );
    println!("          |          |          ");
    println!("__________|__________|__________");
    println!("4         |5         |6         ");
    println!("          |          |          ");
    println!(
        "     {}    |    {}     |    {}     ",
        style(&moves[3]).red(),
        style(&moves[4]).red(),
        style(&moves[5]).red()
    );
    println!("          |          |          ");
    println!("__________|__________|__________");
    println!("7         |8         |9         ");
    println!("          |          |          ");
    println!(
        "     {}    |    {}     |    {}     ",
        style(&moves[6]).red(),
        style(&moves[7]).red(),
        style(&moves[8]).red()
    );
    println!("          |          |          ");
    println!("          |          |          ");
}

fn check_winner(moves: &[String]) -> bool {
    // Check rows, columns, and diagonals
    if (moves[0] != " " && moves[0] == moves[1] && moves[1] == moves[2])
        || (moves[3] != " " && moves[3] == moves[4] && moves[4] == moves[5])
        || (moves[6] != " " && moves[6] == moves[7] && moves[7] == moves[8])
        || (moves[0] != " " && moves[0] == moves[3] && moves[3] == moves[6])
        || (moves[1] != " " && moves[1] == moves[4] && moves[4] == moves[7])
        || (moves[2] != " " && moves[2] == moves[5] && moves[5] == moves[8])
        || (moves[0] != " " && moves[0] == moves[4] && moves[4] == moves[8])
        || (moves[6] != " " && moves[6] == moves[4] && moves[4] == moves[1])
    {
        return true;
    }
    false
}

fn play_tic_tac_toe() -> Result<()> {
    let player: &str = "X";
    let ai: &str = "O";
    let mut user_move: String = String::new();
    let mut ai_move: usize;
    let mut moves: Vec<String> = vec![
        String::from(" "),
        String::from(" "),
        String::from(" "),
        String::from(" "),
        String::from(" "),
        String::from(" "),
        String::from(" "),
        String::from(" "),
        String::from(" "),
    ];

    loop {
        let mut user_move_id: usize;
        draw_board(&moves);
        if check_winner(&moves) {
            print!("{}", style("========== GAME OVER ! ==========").bold());
            break;
        }
        loop {
            println!(
                "{}",
                style("Enter your position > ").bold().green().bright()
            );
            io::stdin()
                .read_line(&mut user_move)
                .expect("Error wrong input");
            if user_move.trim().len() == 0 {
                continue;
            }
            user_move_id = user_move
                .trim()
                .parse::<usize>()
                .with_context(|| style("Enter valid val between 0 & 8").red())?;

            if user_move_id >= 1 && user_move_id <= 9 && moves[user_move_id - 1] == " " {
                break;
            }
        }
        user_move_id -= 1;
        loop {
            ai_move = rand::thread_rng().gen_range(0..=8);
            if ai_move != user_move_id && moves[ai_move] == " " {
                break;
            }
        }
        moves[user_move_id] = String::from(player);
        moves[ai_move] = String::from(ai);
        user_move = String::new();
    }

    Ok(())
}

fn main() -> Result<()> {
    let real = Cli::parse();

    println!(
        "{}",
        style("Real CLI in Rust ---- Dev.Yahya\n").cyan().bold()
    );

    match &real.command {
        Commands::File { path } => create_file(path)
            .with_context(|| style("Error while creating file, missing arg maybe?").red())?,
        Commands::Dir { path } => create_dir(path)
            .with_context(|| style("Error while create dir, missing arg maybe?").red())?,
        Commands::Read { path } => read_file(path)
            .with_context(|| style("Error while reading file, missing arg maybe?").red())?,
        Commands::Del { path } => delete(path)
            .with_context(|| style("Error while deleting file, missing arg maybe?").red())?,
        Commands::Ren { path, new_name } => rename_file(path, new_name)
            .with_context(|| style("Error while renaming file, missing arg maybe?").red())?,
        Commands::Ace => open_ace().with_context(|| {
            style("Error while opening ace, Awesome Code Editor is not added to PATH.").red()
        })?,
        Commands::Write { path, content } => write_to_file(path, content, true)
            .with_context(|| style("Error while writing to file.").red())?,
        Commands::Append { path, content } => write_to_file(path, content, false)
            .with_context(|| style("Error while appending to file.").red())?,
        Commands::Ls { path } => {
            list(path).with_context(|| style("Error while listing dir.").red())?
        }
        Commands::TicTacToe => {
            play_tic_tac_toe().with_context(|| style("Could not start Tic Tac Toe game.").red())?
        }
        Commands::Build { release } => {
            println!("release: {}", release);
        }
    }

    Ok(())
}
