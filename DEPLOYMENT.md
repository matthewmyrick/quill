# Deployment Guide for Quill Task

This guide explains how to deploy Quill Task to Homebrew and manage releases.

## Prerequisites

Before you can deploy to Homebrew, you need to complete these one-time setup steps:

### 1. Create a Homebrew Tap Repository

1. Create a new GitHub repository named `homebrew-quill` (the `homebrew-` prefix is required)
2. Initialize it with a README
3. Create the following directory structure:
   ```
   homebrew-quill/
   └── Formula/
       └── quill-task.rb
   ```

### 2. Set Up GitHub Secrets

In your main repository (`quill`), add the following GitHub secrets:

1. Go to your repository → Settings → Secrets and Variables → Actions
2. Add these secrets:
   - `HOMEBREW_TAP_TOKEN`: A GitHub Personal Access Token with write access to your homebrew tap repository

#### Creating the Personal Access Token:

1. Go to GitHub → Settings → Developer Settings → Personal Access Tokens → Tokens (classic)
2. Generate a new token with these scopes:
   - `repo` (Full control of private repositories)
   - `workflow` (Update GitHub Action workflows)
3. Copy the token and add it as `HOMEBREW_TAP_TOKEN` secret

### 3. Copy the Homebrew Formula

Copy the `Formula/quill-task.rb` file from this repository to your `homebrew-quill` repository:

```bash
# In your homebrew-quill repository
mkdir -p Formula
cp /path/to/quill/Formula/quill-task.rb Formula/
git add Formula/quill-task.rb
git commit -m "Initial Homebrew formula for quill-task"
git push
```

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
   - Build binaries for all supported platforms
   - Create a GitHub release with the binaries attached
   - Update the Homebrew formula in your tap repository
   - Calculate and update the SHA256 hash of the macOS binary
   - Commit the updated formula to the tap repository

### What Happens During Deployment

The GitHub Action workflow (`.github/workflows/release.yml`) performs these steps:

1. **Create Release**: Creates a GitHub release with the tag name
2. **Build Binaries**: Compiles for multiple platforms:
   - macOS (Intel and Apple Silicon)
   - Linux (GNU and musl)
   - Windows
3. **Upload Assets**: Attaches the compiled binaries to the release
4. **Update Homebrew**: Updates the formula in your tap repository with:
   - New version number
   - Updated download URL
   - Calculated SHA256 hash

### Manual Homebrew Updates (if needed)

If the automatic Homebrew update fails, you can manually update the formula:

1. Download the macOS binary from the release
2. Calculate its SHA256:
   ```bash
   shasum -a 256 quill-task-x86_64-apple-darwin.tar.gz
   ```
3. Update `Formula/quill-task.rb` in your homebrew tap repository:
   ```ruby
   class QuillTask < Formula
     desc "Git-context-aware task management TUI"
     homepage "https://github.com/MatthewMyrick/quill"
     url "https://github.com/MatthewMyrick/quill/releases/download/v0.2.0/quill-task-x86_64-apple-darwin.tar.gz"
     sha256 "new_calculated_sha256_here"
     version "0.2.0"
     
     # ... rest of the formula
   end
   ```

## Testing the Homebrew Installation

After deployment, test the installation:

```bash
# Add your tap (first time only)
brew tap MatthewMyrick/quill

# Install or upgrade
brew install quill-task
# or
brew upgrade quill-task

# Test the installation
quill --help
```

## Troubleshooting

### Common Issues

1. **SHA256 Mismatch**: The automatic SHA256 calculation failed
   - Solution: Manually calculate and update the SHA256 in the formula

2. **GitHub Action Fails**: Check the workflow logs for specific errors
   - Ensure all secrets are properly set
   - Verify the tap repository exists and is accessible

3. **Homebrew Install Fails**: 
   - Check that the binary URL is accessible
   - Verify the SHA256 matches
   - Ensure the binary is executable

### Release Rollback

If you need to rollback a release:

1. Delete the problematic tag and release:
   ```bash
   git tag -d v0.2.0
   git push origin :refs/tags/v0.2.0
   ```
2. Delete the GitHub release from the web interface
3. Revert the Homebrew formula to the previous version

## Version Strategy

Follow semantic versioning:
- `v1.0.0` - Major release (breaking changes)
- `v0.2.0` - Minor release (new features)
- `v0.1.1` - Patch release (bug fixes)

Always update the version in `Cargo.toml` before creating a release tag.

## Users Can Install With:

Once deployed, users can install quill-task with:

```bash
# Add the tap
brew tap MatthewMyrick/quill

# Install the application
brew install quill-task

# Use the application
quill
```

The Homebrew formula installs the binary as `quill` (not `quill-task`) for a better user experience.