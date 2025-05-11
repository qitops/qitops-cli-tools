#!/bin/bash

# Script to find potentially problematic documentation files

echo "=== Checking for empty files ==="
find . -name "*.md" -type f -empty
echo

echo "=== Checking for files with only headings ==="
for file in *.md; do
  if [ -f "$file" ]; then
    # Count lines that are not headings, not empty, and not code blocks
    content_lines=$(grep -v '^#' "$file" | grep -v '^$' | grep -v '^\s*```' | grep -v '^>' | grep -v '^---' | wc -l)
    if [ "$content_lines" -lt 3 ]; then
      echo "$file: Only has $content_lines content lines"
      echo "First 10 lines:"
      head -n 10 "$file"
      echo "---"
    fi
  fi
done
echo

echo "=== Checking for files referenced in SUMMARY.md that don't exist ==="
for file in $(grep -o '\[.*\](.*\.md[^)]*)' SUMMARY.md | sed 's/\[.*\](\(.*\))/\1/' | sed 's/#.*$//' | sort | uniq); do
  if [ ! -f "$file" ]; then
    echo "Missing file: $file"
  fi
done
echo

echo "=== Checking for files not referenced in SUMMARY.md ==="
for file in *.md; do
  if [ "$file" != "SUMMARY.md" ] && ! grep -q "$file" SUMMARY.md; then
    echo "Unreferenced file: $file"
  fi
done
echo

echo "=== Checking for duplicate entries in SUMMARY.md ==="
grep -o '\[.*\](.*\.md[^)]*)' SUMMARY.md | sort | uniq -c | grep -v "^ *1 "
echo

echo "Done!"
