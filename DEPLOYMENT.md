# Deployment Guide for Quill Task

This guide explains how to deploy Quill Task using GitHub releases for direct downloads.

## Prerequisites

Before you can create releases, ensure you have:

1. **GitHub repository** with proper permissions
2. **GitHub Actions enabled** in your repository
3. **Git tags** for versioning

## Deployment Process

### Creating a Release

To deploy a new version:

1. **Update the version** in `Cargo.toml`:
   ```toml
   [package]
   version = "0.2.0"  # Update this
   ```

2. **Commit and push** your changes:
   ```bash
   git add .
   git commit -m "Release v0.2.0"
   git push
   ```

3. **Create and push a Git tag**:
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

4. **The GitHub Action will automatically**:
   - Build binaries for all supported platforms:
     - macOS (Intel and Apple Silicon)
     - Linux (GNU and musl)
     - Windows
   - Create a GitHub release with the binaries attached
   - Make the release available for direct download

### What Happens During Deployment

The GitHub Action workflow (`.github/workflows/release.yml`) performs these steps:

1. **Create Release**: Creates a GitHub release with the tag name
2. **Build Binaries**: Compiles for multiple platforms:
   - `quill-x86_64-apple-darwin.tar.gz` (macOS Intel)
   - `quill-aarch64-apple-darwin.tar.gz` (macOS Apple Silicon)
   - `quill-x86_64-unknown-linux-gnu.tar.gz` (Linux GNU)
   - `quill-x86_64-unknown-linux-musl.tar.gz` (Linux musl)
   - `quill-x86_64-pc-windows-msvc.exe.zip` (Windows)
3. **Upload Assets**: Attaches the compiled binaries to the release

### Testing the Release

After deployment, test the installation:

```bash
# Download the appropriate binary for your platform
curl -L -o quill.tar.gz "https://github.com/MatthewMyrick/quill/releases/latest/download/quill-$(uname -m)-apple-darwin.tar.gz"

# Extract and test
tar -xzf quill.tar.gz
./quill-* --help
```

## Supported Platforms

The automated build creates binaries for:

- **macOS**: Intel (x86_64) and Apple Silicon (aarch64)
- **Linux**: GNU and musl libc variants
- **Windows**: 64-bit executable

## Installation for Users

Once deployed, users can install quill-task by:

1. **Downloading** from the [releases page](https://github.com/MatthewMyrick/quill/releases)
2. **Extracting** the appropriate archive for their platform
3. **Moving** the binary to their PATH (e.g., `/usr/local/bin/`)
4. **Running** `quill` to start the task manager

### Quick Install Script (macOS/Linux)

```bash
curl -L -o /tmp/quill.tar.gz "https://github.com/MatthewMyrick/quill/releases/latest/download/quill-$(uname -m)-$(uname -s | tr '[:upper:]' '[:lower:]').tar.gz"
tar -xzf /tmp/quill.tar.gz -C /tmp
sudo mv /tmp/quill-* /usr/local/bin/quill
chmod +x /usr/local/bin/quill
```

## Troubleshooting

### Common Issues

1. **Build Failures**: Check the GitHub Actions logs for specific errors
   - Ensure all dependencies are properly configured
   - Verify Rust toolchain compatibility

2. **Binary Not Executable**: 
   - Ensure proper permissions: `chmod +x quill`
   - Check if the binary is for the correct platform

3. **Missing Dependencies**: 
   - Linux users may need `libssl-dev` or `openssl-dev`
   - Ensure all system dependencies are installed

### Release Rollback

If you need to rollback a release:

1. Delete the problematic tag and release:
   ```bash
   git tag -d v0.2.0
   git push origin :refs/tags/v0.2.0
   ```
2. Delete the GitHub release from the web interface
3. Create a new tag with the corrected version

## Version Strategy

Follow semantic versioning:
- `v1.0.0` - Major release (breaking changes)
- `v0.2.0` - Minor release (new features)
- `v0.1.1` - Patch release (bug fixes)

Always update the version in `Cargo.toml` before creating a release tag.

## Binary Naming

The GitHub Actions workflow creates platform-specific archives containing the `quill` binary:
- Archives are named with platform information (e.g., `quill-x86_64-apple-darwin.tar.gz`)
- The binary inside is always named `quill` for consistent user experience
- Users can run the application with the simple `quill` command after installation