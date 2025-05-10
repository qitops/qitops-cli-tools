# Contributing to QitOps

Thank you for your interest in contributing to QitOps! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Environment](#development-environment)
- [Project Structure](#project-structure)
- [Contribution Workflow](#contribution-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Issue and PR Labels](#issue-and-pr-labels)
- [Release Process](#release-process)

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. Please be respectful, inclusive, and considerate in all interactions.

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo
- Git

### Setting Up Your Development Environment

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/qitops-cli-tools.git
   cd qitops-cli-tools
   ```
3. Add the upstream repository as a remote:
   ```bash
   git remote add upstream https://github.com/qitops/qitops-cli-tools.git
   ```
4. Build the project:
   ```bash
   cargo build
   ```
5. Run the tests:
   ```bash
   cargo test
   ```

## Development Environment

### Recommended Tools

- **IDE**: VS Code with rust-analyzer extension or IntelliJ IDEA with Rust plugin
- **Linting**: Clippy (`cargo clippy`)
- **Formatting**: Rustfmt (`cargo fmt`)
- **Documentation**: Rustdoc (`cargo doc`)

### Building with Features

QitOps supports optional features that can be enabled during build:

```bash
# Build with AI features
cargo build --features ai

# Build with all features
cargo build --all-features
```

## Project Structure

The project follows a modular structure:

```
qitops/
├── src/
│   ├── main.rs        # CLI parsing using clap
│   ├── api.rs         # API testing implementation
│   ├── performance.rs # Performance testing implementation
│   ├── security.rs    # Security testing implementation
│   ├── web.rs         # Web testing implementation
│   ├── ai.rs          # AI-powered test generation
│   ├── reporting.rs   # Report generation
│   ├── common.rs      # Shared functionality and interfaces
│   └── error.rs       # Error handling
├── tests/
│   └── configs/       # JSON test configuration files
├── .github/
│   └── workflows/     # CI configuration
├── docs/              # Documentation
├── src/templates/     # Template files
└── Cargo.toml         # Dependencies
```

## Contribution Workflow

### Finding Issues to Work On

- Check the [Issues](https://github.com/qitops/qitops-cli-tools/issues) page
- Look for issues labeled `good-first-issue` or `help-wanted`
- Review the [Project Board](https://github.com/qitops/qitops-cli-tools/projects) to see current priorities

### Making Changes

1. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Make your changes
3. Run tests to ensure your changes don't break existing functionality:
   ```bash
   cargo test
   ```
4. Format your code:
   ```bash
   cargo fmt
   ```
5. Run linting checks:
   ```bash
   cargo clippy -- -D warnings
   ```
6. Commit your changes with a descriptive commit message:
   ```bash
   git commit -m "Add feature: your feature description"
   ```
7. Push your changes to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
8. Create a Pull Request against the `master` branch of the main repository

### Pull Request Guidelines

- Provide a clear description of the changes
- Link to any related issues using keywords like "Fixes #123" or "Resolves #456"
- Include tests for new functionality
- Update documentation as needed
- Ensure CI checks pass
- Be responsive to feedback and review comments

## Coding Standards

### Rust Style Guidelines

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use Rustfmt for consistent formatting
- Use Clippy to catch common mistakes and non-idiomatic code
- Write clear, descriptive variable and function names
- Add comments for complex logic
- Use proper error handling with the `Result` type

### Core Principles

When contributing, please keep in mind the core principles of the project:
- Maintain CLI-first approach with no UI dependencies
- Keep dependencies minimal and native
- Ensure compatibility with static binary compilation
- Preserve clear module boundaries
- Design for extensibility

## Testing Guidelines

- Write unit tests for all new functionality
- Include integration tests for end-to-end workflows
- Test edge cases and error conditions
- Use test fixtures in the `tests/` directory
- Mock external dependencies when appropriate

## Documentation

- Update the README.md with any user-facing changes
- Add inline documentation using rustdoc comments (`///`)
- Update the user guide in the `docs/` directory
- Include examples for new features

## Issue and PR Labels

We use the following label categories:

### Type
- `type:bug`: Bug fixes
- `type:feature`: New features
- `type:enhancement`: Improvements to existing features
- `type:documentation`: Documentation updates
- `type:refactor`: Code refactoring
- `type:test`: Test improvements

### Module
- `module:api`: API testing module
- `module:performance`: Performance testing module
- `module:security`: Security testing module
- `module:web`: Web testing module
- `module:ai`: AI integration
- `module:common`: Common functionality
- `module:cli`: Command-line interface
- `module:reporting`: Reporting functionality

### Phase
- `phase:0`: Core functionality
- `phase:1`: Parity features
- `phase:2`: Differentiators
- `phase:3`: AI & Ecosystem

### Status
- `status:blocked`: Blocked by another issue
- `status:help-wanted`: Looking for contributors
- `status:good-first-issue`: Good for newcomers

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with the new version and changes
3. Create a new tag with the version number:
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   ```
4. Push the tag to trigger the release workflow:
   ```bash
   git push origin v0.1.0
   ```
5. The CI/CD pipeline will automatically:
   - Build the release binaries
   - Publish to crates.io
   - Create a GitHub Release
   - Push Docker images

## Thank You!

Your contributions help make QitOps better for everyone. We appreciate your time and effort!
