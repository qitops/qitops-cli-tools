# Enabling GitHub Pages

This document explains how to enable GitHub Pages for the QitOps documentation.

## Prerequisites

- You need to have admin access to the QitOps repository
- The gh-pages branch should already exist (created by the GitHub Actions workflow)

## Steps to Enable GitHub Pages

1. Go to the repository on GitHub
2. Navigate to Settings > Pages
3. Under "Build and deployment", set the following:
   - Source: "Deploy from a branch"
   - Branch: "gh-pages"
   - Folder: "/ (root)"
4. Click "Save"

After enabling GitHub Pages, the documentation will be available at:
https://qitops.github.io/qitops-cli-tools/

## Verifying the Setup

Once GitHub Pages is enabled, you can verify that the documentation is accessible by visiting:
https://qitops.github.io/qitops-cli-tools/index.html

## Troubleshooting

If the documentation is not accessible after enabling GitHub Pages:

1. Check that the gh-pages branch exists
2. Check that the GitHub Actions workflow has run successfully
3. Check that the gh-pages branch contains the built documentation
4. Wait a few minutes for the changes to propagate

## Using the GitHub CLI

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
