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
cargo install markdown-ai-cite-remove --features cli

# Option 2: Install from local source
cd /path/to/markdown-ai-cite-remove
cargo install --path . --features cli

# Option 3: Build without installing
cargo build --release --features cli
# Binary will be at: target/release/md-cite-remove
```

### Verify Installation

```bash
md-cite-remove --version
# Should output: md-cite-remove 0.1.0

md-cite-remove --help
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
echo "Text[1] with citations[2]." | md-cite-remove
# Output: Text with citations.

# From file to stdout
cat document.md | md-cite-remove

# Chain with other commands
cat document.md | md-cite-remove | wc -w
```

### 2. Process a File

```bash
# Read file, output to stdout
md-cite-remove document.md

# Read file, save to another file
md-cite-remove input.md -o output.md
md-cite-remove input.md --output output.md  # Long form
```

### 3. Verbose Mode

See what's happening:

```bash
md-cite-remove input.md -o output.md --verbose
# Output:
# Reading from file: input.md
# Cleaning markdown (input size: 1234 bytes)...
# Cleaned markdown (output size: 1100 bytes)
# Writing to file: output.md
# Done!
```

## Advanced Usage

### Batch Processing

**Process all markdown files in a directory:**

```bash
# Using a for loop
for file in *.md; do
  md-cite-remove "$file" -o "cleaned_${file}"
done

# Using find
find . -name "*.md" -exec md-cite-remove {} -o {}.clean \;

# Process and preserve directory structure
find ./input -name "*.md" | while read file; do
  output="${file/input/output}"
  mkdir -p "$(dirname "$output")"
  md-cite-remove "$file" -o "$output"
done
```

**Batch script example:**

```bash
#!/bin/bash
# clean_all.sh - Remove citations from all markdown files

INPUT_DIR="./ai_output"
OUTPUT_DIR="./cleaned"

mkdir -p "$OUTPUT_DIR"

count=0
for file in "$INPUT_DIR"/*.md; do
  if [ -f "$file" ]; then
    filename=$(basename "$file")
    md-cite-remove "$file" -o "$OUTPUT_DIR/$filename" --verbose
    ((count++))
  fi
done

echo "Processed $count files"
```

### Integration with Other Tools

**1. With Pandoc (convert to HTML/PDF):**

```bash
# Remove citations from and convert to HTML
md-cite-remove input.md | pandoc -f markdown -t html -o output.html

# Remove citations from and convert to PDF
md-cite-remove input.md | pandoc -f markdown -o output.pdf
```

**2. With curl (process API responses):**

```bash
# Remove citations from AI API response
curl -s https://api.example.com/ai-response | md-cite-remove

# Save cleaned response
curl -s https://api.example.com/ai-response | md-cite-remove > cleaned.md
```

**3. With grep/sed (text processing):**

```bash
# Remove citations from and search
md-cite-remove document.md | grep "important"

# Remove citations from and count lines
md-cite-remove document.md | wc -l

# Remove citations from and preview
md-cite-remove document.md | less
```

**4. With git (version control):**

```bash
# Remove citations from all changed markdown files
git diff --name-only | grep '\.md$' | while read file; do
  md-cite-remove "$file" -o "$file.tmp" && mv "$file.tmp" "$file"
done
```

### Automation Examples

**1. Watch directory and auto-clean:**

```bash
#!/bin/bash
# watch_and_clean.sh - Auto-clean new files

INPUT_DIR="./ai_output"
OUTPUT_DIR="./cleaned"

mkdir -p "$OUTPUT_DIR"

# Using fswatch (macOS) or inotifywait (Linux)
fswatch -0 "$INPUT_DIR" | while read -d "" event; do
  if [[ "$event" == *.md ]]; then
    filename=$(basename "$event")
    md-cite-remove "$event" -o "$OUTPUT_DIR/$filename" --verbose
  fi
done
```

**2. Pre-commit hook:**

```bash
#!/bin/bash
# .git/hooks/pre-commit - Remove citations from markdown before commit

for file in $(git diff --cached --name-only | grep '\.md$'); do
  if [ -f "$file" ]; then
    md-cite-remove "$file" -o "$file.tmp"
    mv "$file.tmp" "$file"
    git add "$file"
  fi
done
```

**3. CI/CD pipeline:**

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
      
      - name: Install md-cite-remove
        run: cargo install markdown-ai-cite-remove --features cli
      
      - name: Remove citations from markdown files
        run: |
          find docs -name "*.md" | while read file; do
            md-cite-remove "$file" -o "$file.tmp"
            mv "$file.tmp" "$file"
          done
      
      - name: Commit changes
        run: |
          git config user.name "Bot"
          git config user.email "bot@example.com"
          git add docs/
          git commit -m "Remove citations from citations from docs" || true
          git push
```

## Use Cases

### 1. Blog Publishing

```bash
# Remove citations from AI-generated blog post
md-cite-remove ai_draft.md -o blog_post.md

# Remove citations from and publish
md-cite-remove ai_draft.md | ./publish_to_cms.sh
```

### 2. Documentation Generation

```bash
# Remove citations from AI-generated docs
md-cite-remove ai_docs.md -o README.md --verbose
```

### 3. Content Aggregation

```bash
# Remove citations from multiple AI responses
for source in chatgpt claude perplexity; do
  md-cite-remove "${source}_response.md" -o "cleaned_${source}.md"
done

# Combine cleaned responses
cat cleaned_*.md > combined.md
```

### 4. Research Paper Cleanup

```bash
# Remove citations from research summary
md-cite-remove research_summary.md -o clean_summary.md

# Verify citations removed
diff research_summary.md clean_summary.md
```

## Tips and Tricks

### Performance

```bash
# Process large files efficiently (already optimized)
md-cite-remove large_file.md -o output.md

# Batch process with parallel (if installed)
ls *.md | parallel md-cite-remove {} -o cleaned_{}
```

### Debugging

```bash
# See what changed
md-cite-remove input.md -o output.md --verbose
diff input.md output.md

# Check file sizes
ls -lh input.md output.md
```

### Validation

```bash
# Ensure no citations remain
if md-cite-remove input.md | grep -q '\[[0-9]\+\]'; then
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
which md-cite-remove

# If not found, add to PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

### Issue: Permission denied

**Solution:**
```bash
# Make binary executable
chmod +x ~/.cargo/bin/md-cite-remove

# Or reinstall
cargo install --path . --features cli --force
```

### Issue: Output file same as input

**Solution:**
```bash
# Use temporary file
md-cite-remove input.md -o input.md.tmp
mv input.md.tmp input.md

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

