# GitHub Pages Setup

This document explains how the QitOps documentation is deployed to GitHub Pages.

## Overview

The QitOps documentation is built using [mdBook](https://rust-lang.github.io/mdBook/) and automatically deployed to GitHub Pages whenever changes are pushed to the `master` branch that affect the documentation.

The deployed documentation is available at: [https://qitops.github.io/qitops-cli-tools/](https://qitops.github.io/qitops-cli-tools/)

## Deployment Workflow

The deployment is handled by a GitHub Actions workflow defined in `.github/workflows/docs.yml`. This workflow:

1. Triggers when changes are pushed to the `master` branch that affect:
   - Files in the `docs/` directory
   - The `README.md` file
   - The workflow file itself

2. Sets up mdBook using the `peaceiris/actions-mdbook@v1` action

3. Copies key files to the docs directory:
   - `README.md` → `docs/index.md`
   - `CHANGELOG.md` → `docs/changelog.md`
   - `ROADMAP.md` → `docs/roadmap.md`
   - `CONTRIBUTING.md` → `docs/contributing.md`

4. Creates a `book.toml` configuration file if it doesn't exist

5. Builds the documentation using mdBook

6. Deploys the built documentation to the `gh-pages` branch using the `peaceiris/actions-gh-pages@v3` action

## Enabling GitHub Pages

To enable GitHub Pages for the QitOps documentation:

1. Go to the repository on GitHub
2. Navigate to Settings > Pages
3. Under "Build and deployment", set the following:
   - Source: "Deploy from a branch"
   - Branch: "gh-pages"
   - Folder: "/ (root)"
4. Click "Save"

### Using the GitHub CLI

If you have the GitHub CLI installed and authenticated, you can enable GitHub Pages with the following command:

```bash
gh api -X PUT repos/qitops/qitops-cli-tools/pages \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  -f build_type=workflow \
  -f source.branch=gh-pages \
  -f source.path=/
```

This requires a personal access token with the `repo` scope.

## Configuration

The documentation is configured in the `docs/book.toml` file, which includes:

- Book metadata (title, authors, etc.)
- HTML output configuration
- GitHub repository information
- Search settings
- Custom CSS and JavaScript

Key configuration settings:

```toml
[book]
authors = ["Jonathan Opperman"]
language = "en"
multilingual = false
src = "."
title = "QitOps Documentation"

[output.html]
git-repository-url = "https://github.com/qitops/qitops-cli-tools"
git-repository-icon = "fa-github"
edit-url-template = "https://github.com/qitops/qitops-cli-tools/edit/master/docs/{path}"
site-url = "/qitops-cli-tools/"
```

The `site-url` setting is particularly important for GitHub Pages, as it ensures that all relative URLs are correctly resolved.

## Verifying the Setup

Once GitHub Pages is enabled, you can verify that the documentation is accessible by visiting:
https://qitops.github.io/qitops-cli-tools/

## Troubleshooting

If the documentation is not accessible after enabling GitHub Pages:

1. Check that the gh-pages branch exists
2. Check that the GitHub Actions workflow has run successfully
3. Check that the gh-pages branch contains the built documentation
4. Wait a few minutes for the changes to propagate
5. Verify that the `site-url` in `book.toml` is set correctly
6. Check the GitHub Actions logs for any errors

## Local Development

To preview the documentation locally:

1. Install mdBook:
   ```bash
   cargo install mdbook
   ```

2. Run the development server:
   ```bash
   cd docs
   mdbook serve --open
   ```

3. This will open the documentation in your browser and automatically reload when you make changes

## Adding New Pages

To add a new page to the documentation:

1. Create a new Markdown file in the `docs` directory
2. Add the page to the `SUMMARY.md` file to include it in the navigation

Example `SUMMARY.md` entry:
```markdown
# Summary

- [Introduction](index.md)
- [Your New Page](your-new-page.md)
```

## Customization

The documentation uses custom CSS and JavaScript files for styling and interactivity:

- `custom.css`: Contains custom styles for the documentation
- `custom.js`: Contains custom JavaScript for interactive features

These files are referenced in the `book.toml` file.
