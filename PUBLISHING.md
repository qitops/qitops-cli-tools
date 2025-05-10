# Publishing QitOps to crates.io

This guide explains how to publish QitOps to crates.io, making it available for installation via `cargo install qitops`.

## Prerequisites

1. Create an account on [crates.io](https://crates.io)
2. Verify your email address

## Manual Publishing Process

### 1. Get an API Token

1. Log in to [crates.io](https://crates.io)
2. Go to your account settings
3. Navigate to the API Tokens section
4. Create a new token with the "publish" scope
5. Copy the token (you won't be able to see it again)

### 2. Login to crates.io from your local machine

```bash
cargo login YOUR_API_TOKEN
```

### 3. Verify your package

Before publishing, verify that your package is ready:

```bash
# Check for any issues
cargo publish --dry-run

# Verify the package contents
cargo package --list
```

### 4. Publish the package

```bash
cargo publish
```

## Automated Publishing with GitHub Actions

We've set up a GitHub Actions workflow to automatically publish new versions when you create a new tag:

1. Update the version in `Cargo.toml`
2. Commit the changes
3. Create and push a new tag:

```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

4. The GitHub Actions workflow will automatically publish the new version to crates.io

### Setting up the GitHub Secret

For the automated workflow to work, you need to add your crates.io API token as a GitHub secret:

1. Go to your GitHub repository
2. Navigate to Settings > Secrets and variables > Actions
3. Click "New repository secret"
4. Name: `CRATES_IO_TOKEN`
5. Value: Your crates.io API token
6. Click "Add secret"

## Version Management

Follow [Semantic Versioning](https://semver.org/) for version numbers:

- MAJOR version for incompatible API changes
- MINOR version for new functionality in a backward compatible manner
- PATCH version for backward compatible bug fixes

Update the CHANGELOG.md file with each new release.
