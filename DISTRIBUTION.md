# QitOps Distribution Guide

This document provides a comprehensive guide for distributing QitOps through various channels.

## Distribution Channels

QitOps is available through the following channels:

1. **crates.io**: The official Rust package registry
2. **GitHub Releases**: Pre-built binaries for multiple platforms
3. **Docker Hub**: Container images
4. **Homebrew**: Package manager for macOS
5. **Snap Store**: Package manager for Linux

## Publishing to crates.io

See [PUBLISHING.md](PUBLISHING.md) for detailed instructions.

## GitHub Releases

GitHub Releases are automatically created when you push a new tag. The workflow in `.github/workflows/release.yml` handles:

1. Building binaries for multiple platforms
2. Creating a GitHub Release
3. Uploading the binaries as release assets

## Docker Hub

Docker images are automatically built and pushed to Docker Hub when you push a new tag. The workflow in `.github/workflows/docker.yml` handles:

1. Building a multi-platform Docker image
2. Pushing the image to Docker Hub with appropriate tags

### Docker Hub Setup

1. Create a Docker Hub account if you don't have one
2. Create a repository named `qitops/qitops`
3. Add the following secrets to your GitHub repository:
   - `DOCKERHUB_USERNAME`: Your Docker Hub username
   - `DOCKERHUB_TOKEN`: Your Docker Hub access token

## Homebrew

To make QitOps available via Homebrew:

1. After your first release, calculate the SHA256 hash of the release tarball:
   ```bash
   curl -L https://github.com/qitops/qitops-cli-tools/archive/refs/tags/v0.1.0.tar.gz | shasum -a 256
   ```

2. Update the SHA256 in `homebrew/qitops.rb`

3. Create a tap repository on GitHub:
   ```bash
   # Create a new repository named homebrew-qitops
   # Clone it locally
   git clone https://github.com/qitops/homebrew-qitops.git
   cd homebrew-qitops
   
   # Copy the formula
   cp /path/to/qitops/homebrew/qitops.rb .
   
   # Commit and push
   git add qitops.rb
   git commit -m "Add QitOps formula"
   git push
   ```

4. Users can then install QitOps with:
   ```bash
   brew tap qitops/qitops
   brew install qitops
   ```

## Snap Store

To publish QitOps to the Snap Store:

1. Create an account on [snapcraft.io](https://snapcraft.io)
2. Install the Snap tools:
   ```bash
   sudo snap install snapcraft --classic
   ```

3. Build the snap package:
   ```bash
   cd /path/to/qitops
   snapcraft
   ```

4. Register the snap name:
   ```bash
   snapcraft register qitops
   ```

5. Push the snap to the store:
   ```bash
   snapcraft push --release=stable qitops_0.1.0_amd64.snap
   ```

6. Users can then install QitOps with:
   ```bash
   sudo snap install qitops
   ```

## Website and Documentation

The documentation website is automatically built and deployed to GitHub Pages when you push changes to the `master` branch. The workflow in `.github/workflows/docs.yml` handles:

1. Building the documentation with mdBook
2. Deploying it to GitHub Pages

To set up GitHub Pages:

1. Go to your GitHub repository
2. Navigate to Settings > Pages
3. Set the source to "Deploy from a branch"
4. Select the `gh-pages` branch and the `/ (root)` folder
5. Click "Save"

The documentation will be available at `https://qitops.github.io/qitops-cli-tools/`.
