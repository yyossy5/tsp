use clap::Parser;
use std::env;
use std::path::Path;
use std::process::{exit, Command};

#[derive(Parser)]
#[command(name = "tsps")]
#[command(about = "Quickly set up tmux workspaces by splitting windows into multiple panes")]
#[command(version)]
struct Cli {
    /// Number of panes to create (including current pane)
    #[arg(value_name = "PANE_COUNT")]
    pane_count: u32,

    /// Directory to navigate to in all panes
    #[arg(value_name = "DIRECTORY")]
    directory: String,
}

fn main() {
    let cli = Cli::parse();

    if cli.pane_count < 1 {
        eprintln!("Error: pane_count must be a positive integer");
        exit(1);
    }

    let target_dir = Path::new(&cli.directory);
    if !target_dir.exists() {
        eprintln!("Error: Directory '{}' does not exist", cli.directory);
        exit(1);
    }

    // Check if we're in a tmux session
    if env::var("TMUX").is_err() {
        eprintln!("Error: Not in a tmux session");
        exit(1);
    }

    // Get absolute path
    let absolute_path = match target_dir.canonicalize() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Error: Cannot resolve path '{}': {}", cli.directory, e);
            exit(1);
        }
    };

    let path_str = absolute_path.to_string_lossy();

    // Move current pane to target directory
    if let Err(e) = Command::new("tmux")
        .args(["send-keys", &format!("cd '{}'", path_str), "Enter"])
        .status()
    {
        eprintln!("Error: Failed to execute tmux command: {}", e);
        exit(1);
    }

    // Create additional panes (pane_count - 1)
    for i in 1..cli.pane_count {
        let split_direction = if i % 2 == 1 { "-h" } else { "-v" };

        if let Err(e) = Command::new("tmux")
            .args(["split-window", split_direction, "-c", &path_str])
            .status()
        {
            eprintln!("Error: Failed to create pane {}: {}", i + 1, e);
            exit(1);
        }
    }

    // Arrange panes in a tiled layout
    if let Err(e) = Command::new("tmux")
        .args(["select-layout", "tiled"])
        .status()
    {
        eprintln!("Error: Failed to arrange panes: {}", e);
        exit(1);
    }

    println!(
        "Created {} panes in directory: {}",
        cli.pane_count, path_str
    );
}
