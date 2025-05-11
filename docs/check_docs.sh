#!/bin/bash

# Script to check for empty or placeholder documentation files
# and identify fragment links in SUMMARY.md

echo "=== Checking for empty or minimal documentation files ==="
echo

# Check file sizes
echo "Files with fewer than 50 lines (potential placeholders):"
for file in *.md; do
  lines=$(wc -l < "$file")
  if [ "$lines" -lt 50 ] && [ "$file" != "SUMMARY.md" ]; then
    echo "  - $file: $lines lines"
  fi
done
echo

# Check for files with just headings and minimal content
echo "Files with minimal content (just headings or boilerplate):"
for file in *.md; do
  if [ "$file" != "SUMMARY.md" ]; then
    # Count non-empty, non-heading, non-comment lines
    content_lines=$(grep -v "^#" "$file" | grep -v "^$" | grep -v "^>" | grep -v "^---" | wc -l)
    if [ "$content_lines" -lt 10 ]; then
      echo "  - $file: $content_lines content lines"
    fi
  fi
done
echo

# Check for fragment links in SUMMARY.md
echo "=== Checking for fragment links in SUMMARY.md ==="
echo

echo "Fragment links found in SUMMARY.md:"
grep -o '\[.*\](.*\.md#[^)]*)' SUMMARY.md | sed 's/\[.*\](\(.*\))/\1/'
echo

# Check if files referenced in SUMMARY.md exist
echo "=== Checking for missing files referenced in SUMMARY.md ==="
echo

echo "Files referenced in SUMMARY.md that don't exist:"
for file in $(grep -o '\[.*\](.*\.md[^)]*)' SUMMARY.md | sed 's/\[.*\](\(.*\))/\1/' | sed 's/#.*$//' | sort | uniq); do
  if [ ! -f "$file" ]; then
    echo "  - $file"
  fi
done
echo

# Check for sections referenced in SUMMARY.md that don't exist in the target files
echo "=== Checking for missing sections referenced in SUMMARY.md ==="
echo

echo "Sections referenced in SUMMARY.md that might not exist in the target files:"
for link in $(grep -o '\[.*\](.*\.md#[^)]*)' SUMMARY.md | sed 's/\[.*\](\(.*\))/\1/'); do
  file=$(echo "$link" | sed 's/#.*$//')
  section=$(echo "$link" | sed 's/.*#//')
  if [ -f "$file" ]; then
    if ! grep -q "^##* .*{#$section}" "$file"; then
      echo "  - Section #$section in $file"
    fi
  fi
done
echo

echo "=== Recommendations ==="
echo
echo "1. Create dedicated .md files for each fragment link in SUMMARY.md"
echo "2. Move content from sections to these dedicated files"
echo "3. Update SUMMARY.md to point to the new files instead of fragments"
echo "4. Run 'mdbook clean && mdbook build' to rebuild the documentation"
echo

echo "Done!"
