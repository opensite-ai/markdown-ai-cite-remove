# CLI Usage Guide

Complete guide for using the `md-cite-remove` command-line tool.

## Installation

### Prerequisites

- **Rust 1.70+** and Cargo
  - Install from https://rustup.rs/
  - Verify: `rustc --version` should show 1.70 or later

### Install the CLI Tool

```bash
# Option 1: Install from crates.io (when published)
cargo install markdown-ai-cite-remove

# Option 2: Install from local source
cd /path/to/markdown-ai-cite-remove
cargo install --path .

# Option 3: Build without installing
cargo build --release
# Binary will be at: target/release/mdcr
```

### Verify Installation

```bash
mdcr --version
# Should output: mdcr 0.2.3

mdcr --help
# Shows usage information
```

### Troubleshooting Installation

**Command not found:**
```bash
# Add Cargo bin directory to PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Make permanent by adding to ~/.bashrc or ~/.zshrc
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Basic Usage

### 1. Process from Stdin to Stdout

Perfect for piping and command chaining:

```bash
# Simple pipe
echo "Text[1] with citations[2]." | mdcr
# Output: Text with citations.

# From file to stdout
cat document.md | mdcr

# Chain with other commands
cat document.md | mdcr | wc -w
```

### 2. Process a File (Auto-Generated Output)

The easiest way - just provide the input file:

```bash
# Auto-generate output file with __cite_removed suffix
mdcr ai_response.md
# Creates: ai_response__cite_removed.md

mdcr chatgpt_output.md
# Creates: chatgpt_output__cite_removed.md
```

### 3. Process a File (Custom Output)

Specify a custom output filename:

```bash
# Read file, save to specific file
mdcr input.md -o output.md
mdcr input.md --output output.md  # Long form
```

### 4. Verbose Mode

See what's happening:

```bash
mdcr input.md --verbose
# Output:
# Reading from file: input.md
# Removing citations (input size: 1234 bytes)...
# Citations removed (output size: 1100 bytes)
# Writing to file: input__cite_removed.md
# Done!
```

## Common Workflows

### File Processing

**1. Single file (auto-generated output):**

```bash
mdcr document.md
# Creates: document__cite_removed.md
```

**2. Single file (custom output):**

```bash
mdcr document.md -o clean_document.md
```

**3. Multiple files (auto-generated outputs):**

```bash
# Process all markdown files - easiest way!
for file in *.md; do
  mdcr "$file"
done
# Creates: file1__cite_removed.md, file2__cite_removed.md, etc.
```

**4. Multiple files (custom naming):**

```bash
# Process with custom output names
for file in *.md; do
  mdcr "$file" -o "cleaned_${file}"
done

# Process files in a directory
for file in ./docs/*.md; do
  filename=$(basename "$file")
  mdcr "$file" -o "./cleaned/$filename"
done
```

### In-Place Editing

**1. In-place cleaning (overwrites original):**

```bash
# Using temporary file
mdcr input.md -o temp.md && mv temp.md input.md

# In a loop
for file in *.md; do
  mdcr "$file" -o "$file.tmp"
  mv "$file.tmp" "$file"
done
```

**2. Backup before cleaning:**

```bash
# Create backup then clean
cp document.md document.md.backup
mdcr document.md -o document.md.tmp && mv document.md.tmp document.md
```

### Integration with Other Tools

**1. With find (auto-generated outputs):**

```bash
# Process all .md files recursively - simple!
find . -name "*.md" -exec mdcr {} \;
# Creates files with __cite_removed suffix in same directories
```

**2. With find (custom outputs):**

```bash
# Process and preserve directory structure
find ./input -name "*.md" | while read file; do
  output="${file/input/output}"
  mkdir -p "$(dirname "$output")"
  mdcr "$file" -o "$output"
done
```

**3. With xargs:**

```bash
# Process files in parallel (if xargs supports -P)
find . -name "*.md" | xargs -P 4 -I {} mdcr {} -o {}.clean
```

**4. With other commands:**

```bash
# Remove citations and count lines
mdcr document.md -o - | wc -l

# Remove citations and preview
mdcr document.md -o - | less
```

**5. With git (version control):**

```bash
# Remove citations from all changed markdown files
git diff --name-only | grep '\.md$' | while read file; do
  mdcr "$file" -o "$file.tmp" && mv "$file.tmp" "$file"
done
```

### Automation Examples

**1. Watch directory and auto-clean:**

```bash
#!/bin/bash
# watch_and_clean.sh - Auto-clean new files with custom output directory

INPUT_DIR="./ai_output"
OUTPUT_DIR="./cleaned"

mkdir -p "$OUTPUT_DIR"

# Using fswatch (macOS) or inotifywait (Linux)
fswatch -0 "$INPUT_DIR" | while read -d "" event; do
  if [[ "$event" == *.md ]]; then
    filename=$(basename "$event")
    mdcr "$event" -o "$OUTPUT_DIR/$filename" --verbose
  fi
done
```

**2. Batch processing with custom naming pattern:**

```bash
#!/bin/bash
# batch_clean.sh - Process files with timestamp in output name

for file in ./ai_output/*.md; do
  filename=$(basename "$file" .md)
  timestamp=$(date +%Y%m%d_%H%M%S)
  mdcr "$file" -o "./cleaned/${filename}_cleaned_${timestamp}.md"
done
```

**3. Pre-commit hook:**

```bash
#!/bin/bash
# .git/hooks/pre-commit - Remove citations from markdown before commit

for file in $(git diff --cached --name-only | grep '\.md$'); do
  if [ -f "$file" ]; then
    mdcr "$file" -o "$file.tmp"
    mv "$file.tmp" "$file"
    git add "$file"
  fi
done
```

**4. CI/CD pipeline:**

```yaml
# .github/workflows/clean-docs.yml
name: Remove citations from Documentation

on:
  push:
    paths:
      - 'docs/**/*.md'

jobs:
  clean:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install mdcr
        run: cargo install markdown-ai-cite-remove

      - name: Remove citations from markdown files
        run: |
          find docs -name "*.md" | while read file; do
            mdcr "$file" -o "$file.tmp"
            mv "$file.tmp" "$file"
          done

      - name: Commit changes
        run: |
          git config user.name "Bot"
          git config user.email "bot@example.com"
          git add docs/
          git commit -m "Remove citations from docs" || true
          git push
```

**5. Conditional processing based on file content:**

```bash
#!/bin/bash
# smart_clean.sh - Only process files that contain citations

for file in *.md; do
  # Check if file contains citations
  if grep -q '\[[0-9]\+\]' "$file"; then
    echo "Processing $file (contains citations)"
    mdcr "$file" --verbose
  else
    echo "Skipping $file (no citations found)"
  fi
done
```

## Use Cases

### 1. Blog Publishing

```bash
# Remove citations from AI-generated blog post (auto-output)
mdcr ai_draft.md
# Creates: ai_draft__cite_removed.md

# Remove citations and publish directly
mdcr ai_draft.md -o - | ./publish_to_cms.sh
```

### 2. Documentation Generation

```bash
# Remove citations from AI-generated docs
mdcr ai_docs.md -o README.md --verbose
```

### 3. Content Aggregation

```bash
# Remove citations from multiple AI responses (auto-output)
for source in chatgpt claude perplexity; do
  mdcr "${source}_response.md"
done
# Creates: chatgpt_response__cite_removed.md, etc.

# Or with custom naming
for source in chatgpt claude perplexity; do
  mdcr "${source}_response.md" -o "cleaned_${source}.md"
done

# Combine cleaned responses
cat cleaned_*.md > combined.md
```

### 4. Research Paper Cleanup

```bash
# Remove citations from research summary
mdcr research_summary.md
# Creates: research_summary__cite_removed.md

# Verify citations removed
diff research_summary.md research_summary__cite_removed.md
```

## Tips and Tricks

### Performance

```bash
# Process large files efficiently (already optimized)
mdcr large_file.md

# Batch process with parallel (if installed)
ls *.md | parallel mdcr {}
```

### Debugging

```bash
# See what changed
mdcr input.md --verbose
diff input.md input__cite_removed.md

# Check file sizes
ls -lh input.md input__cite_removed.md
```

### Validation

```bash
# Ensure no citations remain
if mdcr input.md -o - | grep -q '\[[0-9]\+\]'; then
  echo "Warning: Citations still present!"
else
  echo "All citations removed successfully"
fi
```

## Common Issues

### Issue: Command not found

**Solution:**
```bash
# Check if installed
which mdcr

# If not found, add to PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

### Issue: Permission denied

**Solution:**
```bash
# Make binary executable
chmod +x ~/.cargo/bin/mdcr

# Or reinstall
cargo install --path . --force
```

### Issue: Want to overwrite input file

**Solution:**
```bash
# Use temporary file
mdcr input.md -o input.md.tmp
mv input.md.tmp input.md

# Or use the auto-generated file and rename
mdcr input.md
mv input__cite_removed.md input.md

# Or use in-place script
md-cite-remove input.md -o temp && mv temp input.md
```

## Getting Help

```bash
# Show help
md-cite-remove --help

# Show version
md-cite-remove --version
```

For more information, see the main [README.md](../../README.md) or [BENCHMARKING.md](../performance/BENCHMARKING.md).

