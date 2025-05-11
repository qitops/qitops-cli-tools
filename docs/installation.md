# Installation

QitOps is available for Linux, macOS, and Windows. This guide covers various installation methods to help you get started quickly.

## System Requirements

- **Operating System**: Linux, macOS, or Windows
- **Disk Space**: 50MB minimum
- **Memory**: 256MB minimum (2GB+ recommended for AI features)
- **Dependencies**: None (self-contained binary)

## Quick Installation

### Using the Install Script (Linux/macOS)

The easiest way to install QitOps is using our install script:

```bash
curl -sSL https://get.qitops.dev | bash
```

This script will download the latest version of QitOps and install it to `/usr/local/bin/qitops`.

### Manual Installation

#### Linux

```bash
# Download the latest release
curl -sSL https://github.com/qitops/qitops-cli-tools/releases/latest/download/qitops-linux-x86_64 -o qitops

# Make it executable
chmod +x qitops

# Move to a directory in your PATH
sudo mv qitops /usr/local/bin/
```

#### macOS

```bash
# Download the latest release
curl -sSL https://github.com/qitops/qitops-cli-tools/releases/latest/download/qitops-macos-x86_64 -o qitops

# Make it executable
chmod +x qitops

# Move to a directory in your PATH
sudo mv qitops /usr/local/bin/
```

#### Windows

```powershell
# Download the latest release
Invoke-WebRequest -Uri https://github.com/qitops/qitops-cli-tools/releases/latest/download/qitops-windows-x86_64.exe -OutFile qitops.exe

# Move to a directory in your PATH
Move-Item -Path qitops.exe -Destination "$env:USERPROFILE\AppData\Local\Microsoft\WindowsApps\"
```

## Package Managers

### Homebrew (macOS/Linux)

```bash
brew tap qitops/qitops
brew install qitops
```

### Scoop (Windows)

```powershell
scoop bucket add qitops https://github.com/qitops/scoop-bucket.git
scoop install qitops
```

### APT (Debian/Ubuntu)

```bash
# Add the QitOps repository
echo "deb [trusted=yes] https://apt.qitops.dev/ stable main" | sudo tee /etc/apt/sources.list.d/qitops.list

# Update package list
sudo apt update

# Install QitOps
sudo apt install qitops
```

### YUM/DNF (RHEL/Fedora/CentOS)

```bash
# Add the QitOps repository
sudo tee /etc/yum.repos.d/qitops.repo << EOF
[qitops]
name=QitOps Repository
baseurl=https://yum.qitops.dev/
enabled=1
gpgcheck=0
EOF

# Install QitOps
sudo yum install qitops
```

### NPM (Cross-platform)

```bash
npm install -g qitops
```

## Docker

QitOps is also available as a Docker image:

```bash
# Pull the latest image
docker pull qitops/qitops:latest

# Run QitOps
docker run --rm qitops/qitops:latest --version

# Run QitOps with mounted configuration
docker run --rm -v $(pwd)/tests:/tests qitops/qitops:latest api -c /tests/configs/api_test.json
```

## Building from Source

If you prefer to build QitOps from source:

```bash
# Clone the repository
git clone https://github.com/qitops/qitops-cli-tools.git
cd qitops-cli-tools

# Build the binary
make build

# Install the binary
make install
```

## Verifying the Installation

After installation, verify that QitOps is installed correctly:

```bash
qitops --version
```

You should see output similar to:

```
QitOps CLI v1.0.0
```

## Updating QitOps

### Using the Update Command

QitOps includes a self-update feature:

```bash
qitops update
```

### Manual Update

To update manually, simply download and install the latest version using the same method you used for the initial installation.

## Uninstalling QitOps

### Linux/macOS

```bash
sudo rm /usr/local/bin/qitops
```

### Windows

```powershell
Remove-Item "$env:USERPROFILE\AppData\Local\Microsoft\WindowsApps\qitops.exe"
```

### Package Managers

#### Homebrew

```bash
brew uninstall qitops
```

#### Scoop

```powershell
scoop uninstall qitops
```

#### APT

```bash
sudo apt remove qitops
```

#### YUM/DNF

```bash
sudo yum remove qitops
```

#### NPM

```bash
npm uninstall -g qitops
```

## Troubleshooting

### Common Issues

#### Permission Denied

If you see a "permission denied" error when running QitOps:

```bash
chmod +x /path/to/qitops
```

#### Command Not Found

If you see a "command not found" error:

1. Make sure QitOps is installed in a directory in your PATH
2. Try using the full path to the binary: `/usr/local/bin/qitops`

#### SSL Certificate Errors

If you see SSL certificate errors when downloading:

```bash
curl -sSL --insecure https://get.qitops.dev | bash
```

#### Proxy Issues

If you're behind a proxy:

```bash
export HTTP_PROXY=http://proxy.example.com:8080
export HTTPS_PROXY=http://proxy.example.com:8080
curl -sSL https://get.qitops.dev | bash
```

### Getting Help

If you encounter any issues during installation:

- Check the [Troubleshooting Guide](https://docs.qitops.dev/troubleshooting)
- Join our [Discord Community](https://discord.gg/qitops)
- Open an issue on [GitHub](https://github.com/qitops/qitops-cli-tools/issues)

## Next Steps

Now that you have QitOps installed, check out the [Quick Start](quick-start.md) guide to learn how to use it.
