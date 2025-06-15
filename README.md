# TSPS (Tmux Split Panes)

Quickly set up your tmux workspace by splitting a window into multiple panes at a specified directory with a single command.

## Problem it solves

When working with tmux, you often want to set up the same pane layout repeatedly - multiple panes all navigated to your project directory. Manually splitting panes and navigating each one to the right directory becomes tedious, especially when opening new tmux windows frequently.

![Setup Example](tsps-setup-window.png)

TSPS automates this repetitive setup process. Instead of manually:

1. Creating each pane
2. Navigating to your project directory in each pane with `cd /path/to/project`
3. Arranging panes to your preferred layout

You can now simply run: `tsps 4 /path/to/project` and get your ideal workspace instantly.

## Installation

### Via Cargo

```bash
cargo install tsps
```

### Via GitHub Release (macOS Apple Silicon)

```bash
# Download the latest release
curl -L https://github.com/yyossy5/tsps/releases/latest/download/tsps-aarch64-apple-darwin -o tsps

# Make it executable
chmod +x tsps

# Move to your PATH
mv tsps ~/.local/bin/
```

## Usage

```bash
# Create 4 panes, all navigated to /path/to/project
tsps 4 /path/to/project

# Create 3 panes in current directory
tsps 3 .

# Create 2 panes in home directory
tsps 2 ~
```

## Features

- Create specified number of panes in current tmux window
- All panes automatically navigate to specified directory
- Alternates between horizontal and vertical splits
- Automatically arranges panes in tiled layout
- Error handling (tmux session check, directory validation, etc.)

## Requirements

- tmux
- bash

## Update

### If installed via Cargo

```bash
cargo install tsps --force

# If you have cargo-update package
cargo install-update tsps
```

### If installed via GitHub Release

```bash
# Download the latest release
curl -L https://github.com/yyossy5/tsps/releases/latest/download/tsps-aarch64-apple-darwin -o tsps

# Make it executable
chmod +x tsps

# Replace the existing binary
mv tsps ~/.local/bin/
```

## Uninstall

### If installed via Cargo

```bash
cargo uninstall tsps
```

### If installed via GitHub Release

```bash
rm ~/.local/bin/tsps
```
