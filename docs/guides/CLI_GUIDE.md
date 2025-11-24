# CLI Usage Guide

Complete guide for using the `md-cite-clean` command-line tool.

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
# Binary will be at: target/release/md-cite-clean
```

### Verify Installation

```bash
md-cite-clean --version
# Should output: md-cite-clean 0.1.0

md-cite-clean --help
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
echo "Text[1] with citations[2]." | md-cite-clean
# Output: Text with citations.

# From file to stdout
cat document.md | md-cite-clean

# Chain with other commands
cat document.md | md-cite-clean | wc -w
```

### 2. Process a File

```bash
# Read file, output to stdout
md-cite-clean document.md

# Read file, save to another file
md-cite-clean input.md -o output.md
md-cite-clean input.md --output output.md  # Long form
```

### 3. Verbose Mode

See what's happening:

```bash
md-cite-clean input.md -o output.md --verbose
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
  md-cite-clean "$file" -o "cleaned_${file}"
done

# Using find
find . -name "*.md" -exec md-cite-clean {} -o {}.clean \;

# Process and preserve directory structure
find ./input -name "*.md" | while read file; do
  output="${file/input/output}"
  mkdir -p "$(dirname "$output")"
  md-cite-clean "$file" -o "$output"
done
```

**Batch script example:**

```bash
#!/bin/bash
# clean_all.sh - Clean all markdown files

INPUT_DIR="./ai_output"
OUTPUT_DIR="./cleaned"

mkdir -p "$OUTPUT_DIR"

count=0
for file in "$INPUT_DIR"/*.md; do
  if [ -f "$file" ]; then
    filename=$(basename "$file")
    md-cite-clean "$file" -o "$OUTPUT_DIR/$filename" --verbose
    ((count++))
  fi
done

echo "Processed $count files"
```

### Integration with Other Tools

**1. With Pandoc (convert to HTML/PDF):**

```bash
# Clean and convert to HTML
md-cite-clean input.md | pandoc -f markdown -t html -o output.html

# Clean and convert to PDF
md-cite-clean input.md | pandoc -f markdown -o output.pdf
```

**2. With curl (process API responses):**

```bash
# Clean AI API response
curl -s https://api.example.com/ai-response | md-cite-clean

# Save cleaned response
curl -s https://api.example.com/ai-response | md-cite-clean > cleaned.md
```

**3. With grep/sed (text processing):**

```bash
# Clean and search
md-cite-clean document.md | grep "important"

# Clean and count lines
md-cite-clean document.md | wc -l

# Clean and preview
md-cite-clean document.md | less
```

**4. With git (version control):**

```bash
# Clean all changed markdown files
git diff --name-only | grep '\.md$' | while read file; do
  md-cite-clean "$file" -o "$file.tmp" && mv "$file.tmp" "$file"
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
    md-cite-clean "$event" -o "$OUTPUT_DIR/$filename" --verbose
  fi
done
```

**2. Pre-commit hook:**

```bash
#!/bin/bash
# .git/hooks/pre-commit - Clean markdown before commit

for file in $(git diff --cached --name-only | grep '\.md$'); do
  if [ -f "$file" ]; then
    md-cite-clean "$file" -o "$file.tmp"
    mv "$file.tmp" "$file"
    git add "$file"
  fi
done
```

**3. CI/CD pipeline:**

```yaml
# .github/workflows/clean-docs.yml
name: Clean Documentation

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
      
      - name: Install md-cite-clean
        run: cargo install markdown-ai-cite-remove --features cli
      
      - name: Clean markdown files
        run: |
          find docs -name "*.md" | while read file; do
            md-cite-clean "$file" -o "$file.tmp"
            mv "$file.tmp" "$file"
          done
      
      - name: Commit changes
        run: |
          git config user.name "Bot"
          git config user.email "bot@example.com"
          git add docs/
          git commit -m "Clean citations from docs" || true
          git push
```

## Use Cases

### 1. Blog Publishing

```bash
# Clean AI-generated blog post
md-cite-clean ai_draft.md -o blog_post.md

# Clean and publish
md-cite-clean ai_draft.md | ./publish_to_cms.sh
```

### 2. Documentation Generation

```bash
# Clean AI-generated docs
md-cite-clean ai_docs.md -o README.md --verbose
```

### 3. Content Aggregation

```bash
# Clean multiple AI responses
for source in chatgpt claude perplexity; do
  md-cite-clean "${source}_response.md" -o "cleaned_${source}.md"
done

# Combine cleaned responses
cat cleaned_*.md > combined.md
```

### 4. Research Paper Cleanup

```bash
# Clean research summary
md-cite-clean research_summary.md -o clean_summary.md

# Verify citations removed
diff research_summary.md clean_summary.md
```

## Tips and Tricks

### Performance

```bash
# Process large files efficiently (already optimized)
md-cite-clean large_file.md -o output.md

# Batch process with parallel (if installed)
ls *.md | parallel md-cite-clean {} -o cleaned_{}
```

### Debugging

```bash
# See what changed
md-cite-clean input.md -o output.md --verbose
diff input.md output.md

# Check file sizes
ls -lh input.md output.md
```

### Validation

```bash
# Ensure no citations remain
if md-cite-clean input.md | grep -q '\[[0-9]\+\]'; then
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
which md-cite-clean

# If not found, add to PATH
export PATH="$HOME/.cargo/bin:$PATH"
```

### Issue: Permission denied

**Solution:**
```bash
# Make binary executable
chmod +x ~/.cargo/bin/md-cite-clean

# Or reinstall
cargo install --path . --features cli --force
```

### Issue: Output file same as input

**Solution:**
```bash
# Use temporary file
md-cite-clean input.md -o input.md.tmp
mv input.md.tmp input.md

# Or use in-place script
md-cite-clean input.md -o temp && mv temp input.md
```

## Getting Help

```bash
# Show help
md-cite-clean --help

# Show version
md-cite-clean --version
```

For more information, see the main [README.md](README.md) or [BENCHMARKING.md](BENCHMARKING.md).

