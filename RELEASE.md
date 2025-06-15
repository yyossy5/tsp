# Release Process

This document describes the release process for tsp.

## Automated Release

When you push a tag, GitHub Actions automatically executes the following:

1. **Code Quality Checks**

   - Format check
   - Clippy (linter) execution

2. **Version Verification**

   - Check if Cargo.toml version matches the tag

3. **Binary Build**

   - macOS aarch64 (Apple Silicon)
   - Additional platforms can be easily added to the matrix

4. **crates.io Publication**

   - Automatically publish package to crates.io

5. **GitHub Release Creation**
   - Auto-generate release notes and create GitHub Release
   - Attach pre-built binaries for supported platforms

## Manual Release Steps

1. **Version Bump**

   ```bash
   # Update version in Cargo.toml
   vim Cargo.toml

   # Commit changes
   git add Cargo.toml
   git commit -m "bump version to X.Y.Z"
   ```

2. **Tag Creation and Push**

   ```bash
   # Create tag (version must match Cargo.toml)
   git tag v0.1.1

   # Push tag (triggers CD pipeline)
   git push origin v0.1.1
   ```

## Required Setup

### GitHub Secrets

The following secret must be configured in your GitHub repository:

- `CARGO_REGISTRY_TOKEN`: crates.io API token
  1. Generate API token at [crates.io](https://crates.io/settings/tokens)
  2. Set in GitHub repository Settings > Secrets and variables > Actions

### First Publication Notes

- Recommended to test with `cargo publish --dry-run` before first publication
- Verify package name availability on crates.io beforehand

## Versioning Rules

Follows [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: Backward-compatible feature additions
- **PATCH**: Backward-compatible bug fixes

## Troubleshooting

### When Release Fails

2. **Format Errors**: Fix with `cargo fmt`
3. **Clippy Warnings**: Check and fix with `cargo clippy`
4. **Version Mismatch**: Verify Cargo.toml and tag versions match
5. **crates.io Publication Error**: Verify token validity

### Check Pipeline Status

You can check workflow execution status in the "Actions" tab on GitHub.
