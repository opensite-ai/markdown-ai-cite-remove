# Frequently Asked Questions (FAQ)

## Installation & Setup

### Q: What version of Rust do I need?

**A:** Rust 1.70 or later. Check your version with:
```bash
rustc --version
```

If you need to update:
```bash
rustup update stable
```

### Q: How do I install the CLI tool?

**A:** Two options:

```bash
# From crates.io (when published)
cargo install markdown-ai-cite-remove --features cli

# From source
git clone https://github.com/opensite-ai/markdown-ai-cite-remove.git
cd markdown-ai-cite-remove
cargo install --path . --features cli
```

### Q: The `md-cite-clean` command isn't found after installation

**A:** Add Cargo's bin directory to your PATH:

```bash
# Add to ~/.bashrc, ~/.zshrc, or equivalent
export PATH="$HOME/.cargo/bin:$PATH"

# Reload your shell
source ~/.bashrc  # or source ~/.zshrc
```

### Q: Do I need Gnuplot installed?

**A:** No, it's optional. Gnuplot is only used for benchmark visualization. Benchmarks work fine without it, but you'll see a harmless warning message. To install:

```bash
# macOS
brew install gnuplot

# Ubuntu/Debian
sudo apt-get install gnuplot
```

## Testing & Benchmarking

### Q: Why do tests show as "ignored" when I run `cargo bench`?

**A:** This is **completely normal** Rust behavior. When running benchmarks, the test harness automatically skips regular tests to avoid interfering with timing measurements. All tests still pass when you run `cargo test`.

To verify:
```bash
cargo test  # All 58 tests should pass
```

### Q: What are "outliers" in benchmark results?

**A:** Outliers are measurements that deviate from the typical execution time. They're caused by:
- Operating system scheduling
- CPU frequency scaling
- Background processes
- Cache effects

**3-13% outliers is completely normal** and doesn't indicate a problem. Criterion automatically detects and excludes them from statistics.

### Q: How do I know if my changes improved performance?

**A:** Use baseline comparison:

```bash
# Save current performance
cargo bench -- --save-baseline before

# Make your changes...

# Compare
cargo bench -- --baseline before
```

Look for the "change" line in output. Changes with p < 0.05 are statistically significant.

### Q: Can I run benchmarks faster?

**A:** Yes, use quick mode:

```bash
cargo bench -- --quick
```

This reduces sample size for faster results (less accurate but good for development).

### Q: How do I view benchmark visualizations?

**A:** After running `cargo bench`, Criterion generates HTML reports with charts:

```bash
# View main report
open target/criterion/report/index.html        # macOS
xdg-open target/criterion/report/index.html    # Linux
start target/criterion/report/index.html       # Windows
```

The reports include:
- Line charts showing performance over time
- Violin plots showing distribution
- Statistical analysis (PDF/CDF plots)
- Comparison charts (if using baselines)

**Note**: Gnuplot must be installed for the best visualizations. See installation instructions in the main README.

## Usage

### Q: How do I clean a file in-place?

**A:** Use a temporary file:

```bash
md-cite-clean input.md -o temp.md && mv temp.md input.md
```

Or in a script:
```bash
#!/bin/bash
for file in *.md; do
  md-cite-clean "$file" -o "$file.tmp"
  mv "$file.tmp" "$file"
done
```

### Q: Can I process multiple files at once?

**A:** Yes, several ways:

```bash
# Loop
for file in *.md; do
  md-cite-clean "$file" -o "cleaned_${file}"
done

# Find
find . -name "*.md" -exec md-cite-clean {} -o {}.clean \;

# Parallel (if installed)
ls *.md | parallel md-cite-clean {} -o cleaned_{}
```

See [CLI_GUIDE.md](CLI_GUIDE.md) (this file) for more examples.

### Q: Does it work with stdin/stdout?

**A:** Yes! Perfect for piping:

```bash
# From stdin
echo "Text[1] here." | md-cite-clean

# Pipe from file
cat document.md | md-cite-clean

# Chain commands
cat document.md | md-cite-clean | pandoc -f markdown -t html
```

### Q: How do I verify citations were removed?

**A:** Use verbose mode:

```bash
md-cite-clean input.md -o output.md --verbose
```

This shows input/output sizes. Or compare files:
```bash
diff input.md output.md
```

## Library Usage

### Q: How do I use this in my Rust project?

**A:** Add to `Cargo.toml`:

```toml
[dependencies]
markdown-ai-cite-remove = "0.1"
```

Then in your code:
```rust
use markdown_ai_cite_remove::clean;

let cleaned = clean("Text[1] here.");
```

### Q: Can I customize what gets removed?

**A:** Yes, use custom configuration:

```rust
use markdown_ai_cite_remove::{CitationCleaner, CleanerConfig};

// Remove only inline citations
let config = CleanerConfig::inline_only();
let cleaner = CitationCleaner::with_config(config);
let cleaned = cleaner.clean("Text[1].\n\n[1]: https://example.com");

// Custom configuration
let config = CleanerConfig {
    remove_inline_citations: true,
    remove_reference_links: false,
    // ... other options
    ..Default::default()
};
```

### Q: Is it thread-safe?

**A:** Yes! The cleaner is stateless and can be safely shared across threads:

```rust
use std::sync::Arc;
use markdown_ai_cite_remove::CitationCleaner;

let cleaner = Arc::new(CitationCleaner::new());

// Use in multiple threads
let cleaner_clone = cleaner.clone();
std::thread::spawn(move || {
    let cleaned = cleaner_clone.clean("Text[1]");
});
```

### Q: Can I reuse a cleaner instance?

**A:** Yes, and it's recommended for batch processing:

```rust
let cleaner = CitationCleaner::new();

for document in documents {
    let cleaned = cleaner.clean(&document);
    // Process cleaned document...
}
```

## Performance

### Q: How fast is it?

**A:** Very fast! Typical performance:
- Simple documents: ~580 ns (sub-microsecond)
- Complex documents: ~2-20 μs
- Large documents (50+ KB): ~100-300 μs
- Throughput: 100-650 MB/s

### Q: Does it allocate memory?

**A:** Minimal allocations. The regex patterns are compiled once and reused. Each cleaning operation allocates only for the output string.

### Q: Can it handle large files?

**A:** Yes! Performance scales linearly with file size. A 50 KB document processes in ~250 μs.

### Q: Is it faster than other solutions?

**A:** Yes, significantly. Rust's zero-cost abstractions and optimized regex engine make it much faster than Python/JavaScript alternatives.

## Features & Limitations

### Q: What citation formats are supported?

**A:** All common AI citation formats:
- Inline numeric: `[1][2][3]`
- Named: `[source:1][ref:2][cite:3][note:4]`
- Reference links: `[1]: https://...`
- Reference headers: `## References`, `# Citations`, etc.
- Bibliographic entries: `[1] Author (2024). Title...`

### Q: Does it preserve markdown formatting?

**A:** Yes! It preserves:
- Bold, italic, code
- Links and images
- Lists and tables
- Headings
- Code blocks

### Q: Does it remove citations from code blocks?

**A:** Currently yes (known limitation in v0.1). This is acceptable for most use cases since citations in code blocks are rare. Future versions may add code block detection.

### Q: What about citations in inline code?

**A:** They're removed. If you need to preserve `[1]` in inline code, this is a limitation of v0.1.

### Q: Can I add custom citation patterns?

**A:** Not in v0.1, but this is planned for future releases. Current patterns cover all major AI tools (ChatGPT, Claude, Perplexity, Gemini).

## Troubleshooting

### Q: I'm getting compilation errors

**A:** Check your Rust version:
```bash
rustc --version  # Should be 1.70+
rustup update stable
```

### Q: Tests are failing

**A:** Make sure you're running the full test suite:
```bash
cargo test --all-features
```

If tests still fail, please open an issue with the error output.

### Q: Benchmarks show inconsistent results

**A:** This is normal. For more consistent results:
1. Close unnecessary applications
2. Ensure laptop is plugged in
3. Let system cool down between runs
4. Run multiple times and compare

### Q: The output looks wrong

**A:** Please open an issue with:
1. Input markdown
2. Expected output
3. Actual output
4. Command or code used

## Contributing

### Q: How can I contribute?

**A:** Contributions welcome! You can:
- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation
- Add test cases

See the main [README.md](../../README.md) for development setup.

### Q: What's the development workflow?

**A:**
```bash
# Clone and setup
git clone https://github.com/opensite-ai/markdown-ai-cite-remove.git
cd markdown-ai-cite-remove

# Make changes...

# Run tests
cargo test

# Run benchmarks
cargo bench

# Format code
cargo fmt

# Run linter
cargo clippy

# Build docs
cargo doc --open
```

### Q: How do I add a new test?

**A:** Add to `tests/integration_tests.rs`:

```rust
#[test]
fn test_my_new_case() {
    let input = "Your test input[1]";
    let expected = "Your expected output";
    assert_eq!(clean(input), expected);
}
```

Then run:
```bash
cargo test test_my_new_case
```

## Getting Help

### Q: Where can I get more help?

**A:**
- **Documentation**: [README.md](../../README.md), [CLI_GUIDE.md](CLI_GUIDE.md), [BENCHMARKING.md](../performance/BENCHMARKING.md)
- **API Docs**: Run `cargo doc --open`
- **Examples**: Check the `examples/` directory
- **Issues**: Open an issue on GitHub
- **Email**: contact@opensite.ai

### Q: How do I report a bug?

**A:** Open a GitHub issue with:
1. Description of the problem
2. Steps to reproduce
3. Expected vs actual behavior
4. Your environment (OS, Rust version)
5. Minimal code example

### Q: Is there a Discord/Slack?

**A:** Not yet, but we're considering it based on community interest. For now, use GitHub issues for questions and discussions.

