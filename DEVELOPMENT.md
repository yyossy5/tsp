# Development

## Prerequisites

- Rust 1.70 or later
- tmux (for testing)

## Building from source

```bash
git clone https://github.com/yyossy5/tsps.git
cd tsps
cargo build --release
```

The binary will be available at `target/release/tsps`.

## Testing

To test the command locally without installing:

```bash
# Build in debug mode for faster compilation
cargo build

# Test help and version
./target/debug/tsps --help
./target/debug/tsps --version

# Test with tmux (must be run inside a tmux session)
tmux new-session -d -s test-session
tmux attach -t test-session

# Inside tmux session, test the command:
./target/debug/tsps 3 .
./target/debug/tsps 4 /tmp
```

## Formatting & Linting

```bash
cargo fmt     # Formatting
cargo clippy  # Linting
```

### Release process

1.  Update version in `Cargo.toml`
2.  Publish to crates.io: `cargo publish`
3.  Create a git tag: `git tag v0.1.0`
4.  Push tag: `git push origin v0.1.0`

## Publishing to crates.io

To publish the crate to crates.io:

```bash
# Login to crates.io (first time only)
cargo login

# Publish the crate
cargo publish
```
