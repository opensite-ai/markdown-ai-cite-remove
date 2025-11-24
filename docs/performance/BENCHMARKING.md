# Benchmarking Guide

## Understanding Benchmark Results

This guide explains how to interpret the benchmark output from `cargo bench` and what the various metrics mean.

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench chatgpt_format

# Save baseline for comparison
cargo bench -- --save-baseline main

# Compare against baseline
cargo bench -- --baseline main
```

## Understanding the Output

### Sample Output Explained

```
simple_citations/inline_numeric
                        time:   [577.42 ns 580.48 ns 583.82 ns]
                        thrpt:  [90.638 MiB/s 91.161 MiB/s 91.643 MiB/s]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
```

**What each line means:**

1. **`time: [577.42 ns 580.48 ns 583.82 ns]`**
   - Lower bound, median, upper bound of execution time
   - `580.48 ns` = typical execution time (median)
   - The range shows measurement confidence interval (95%)

2. **`thrpt: [90.638 MiB/s 91.161 MiB/s 91.643 MiB/s]`**
   - Throughput in megabytes per second
   - Higher is better
   - Shows how much data can be processed per second

3. **`Found 5 outliers among 100 measurements (5.00%)`**
   - Outliers are measurements that deviate from the norm
   - **This is normal and expected** (typically 3-13%)
   - Caused by OS scheduling, CPU frequency scaling, background processes

### Why Tests Show as "Ignored"

When you run `cargo bench`, you'll see output like:

```
running 58 tests
test result: ok. 0 passed; 0 failed; 58 ignored; 0 measured; 0 filtered out
```

**This is completely normal!** Here's why:

- `cargo bench` runs the benchmark harness, not the test harness
- Regular tests are automatically "ignored" during benchmarking
- This prevents tests from interfering with benchmark timing
- All tests still pass when you run `cargo test`

**To verify tests pass:**
```bash
cargo test  # All 58 tests should pass
```

## Performance Metrics

### Typical Results (Apple Silicon M-series)

| Benchmark | Time | Throughput | Input Size |
|-----------|------|------------|------------|
| Simple inline citations | ~580 ns | 91 MiB/s | 53 bytes |
| Complex document | ~2.5 μs | 287 MiB/s | 731 bytes |
| Real ChatGPT output | ~18 μs | 645 MiB/s | 11.8 KB |
| Real Perplexity output | ~245 μs | 224 MiB/s | 54.9 KB |
| Batch (5 documents) | ~2.2 μs | 43 MiB/s | 95 bytes total |
| No citations | ~320 ns | 393 MiB/s | 126 bytes |
| Only citations | ~280 ns | 107 MiB/s | 30 bytes |

### Performance Characteristics

**Latency:**
- Sub-microsecond for simple documents
- ~1-20 μs for typical AI responses
- ~100-300 μs for very large documents (50+ KB)

**Throughput:**
- 100-650 MB/s depending on document complexity
- Higher throughput for documents with more citations (more work per byte)
- Lower throughput for passthrough (no citations to remove)

**Scalability:**
- Linear with document size
- No memory allocations in hot path
- Regex patterns compiled once and reused

## Understanding Outliers

### What Are Outliers?

Outliers are measurements that fall outside the expected range. They're classified as:

- **Mild outliers**: 1.5-3x the interquartile range
- **Severe outliers**: >3x the interquartile range

### Why Do Outliers Occur?

1. **OS Scheduling**: Your process gets preempted by the OS
2. **CPU Frequency Scaling**: CPU changes clock speed during measurement
3. **Cache Effects**: Data not in CPU cache on some runs
4. **Background Processes**: Other programs using CPU
5. **Thermal Throttling**: CPU reduces speed due to heat

### Are Outliers a Problem?

**No!** Outliers of 3-13% are completely normal and expected. Criterion:
- Automatically detects outliers
- Excludes them from final statistics
- Reports them for transparency

**When to worry:**
- >20% outliers consistently
- Bimodal distribution (two distinct peaks)
- Increasing trend over time

## Gnuplot Warning

### What It Means

```
Gnuplot not found, using plotters backend
```

This is just an informational message. Criterion can use Gnuplot for generating plots, but falls back to an alternative backend if it's not installed.

### Should You Install Gnuplot?

**Optional but recommended** for:
- Visual comparison of benchmarks
- Trend analysis over time
- Publication-quality plots

**Installation:**
```bash
# macOS
brew install gnuplot

# Ubuntu/Debian
sudo apt-get install gnuplot

# Windows
# Download from http://www.gnuplot.info/
```

### Viewing Benchmark Visualizations

After running `cargo bench`, Criterion generates HTML reports with interactive charts:

**1. View the main report:**
```bash
# macOS
open target/criterion/report/index.html

# Linux
xdg-open target/criterion/report/index.html

# Windows
start target/criterion/report/index.html
```

**2. View individual benchmark reports:**
```bash
# Simple citations benchmark
open target/criterion/simple_citations/inline_numeric/report/index.html

# Real-world ChatGPT benchmark
open target/criterion/real_world/chatgpt_format/report/index.html

# Complex document benchmark
open target/criterion/complex_document/full_document/report/index.html
```

**3. What you'll see:**
- **Line charts** - Performance over time
- **Violin plots** - Distribution of measurements
- **PDF/CDF plots** - Statistical analysis
- **Comparison charts** - Before/after comparisons (with baselines)
- **Detailed statistics** - Mean, median, std dev, outliers

**4. Report structure:**
```
target/criterion/
├── report/
│   └── index.html              # Main report (start here)
├── simple_citations/
│   └── inline_numeric/
│       └── report/
│           └── index.html      # Individual benchmark report
├── complex_document/
│   └── full_document/
│       └── report/
│           └── index.html
├── real_world/
│   ├── chatgpt_format/
│   │   └── report/
│   │       └── index.html
│   └── perplexity_format/
│       └── report/
│           └── index.html
└── ... (other benchmarks)
```

**Tip**: Bookmark `target/criterion/report/index.html` for easy access after each benchmark run!

## Comparing Benchmarks

### Save a Baseline

```bash
# Before making changes
cargo bench -- --save-baseline before

# Make your changes...

# Compare
cargo bench -- --baseline before
```

### Interpreting Comparison Output

```
                        time:   [580.48 ns 583.82 ns 587.42 ns]
                        change: [-2.5% -1.2% +0.3%] (p = 0.15 > 0.05)
                        No change in performance detected.
```

- **change**: Percentage change from baseline
- **p-value**: Statistical significance (p < 0.05 = significant)
- **Interpretation**: No significant change if p > 0.05

## Best Practices

### For Accurate Benchmarks

1. **Close unnecessary applications**
2. **Disable CPU frequency scaling** (if possible)
3. **Run multiple times** and compare
4. **Use release mode** (benchmarks automatically use release)
5. **Warm up the system** (first run may be slower)

### For Development

```bash
# Quick check (fewer samples)
cargo bench -- --quick

# Full benchmark suite
cargo bench

# Specific benchmark
cargo bench chatgpt

# Save and compare
cargo bench -- --save-baseline main
# ... make changes ...
cargo bench -- --baseline main
```

## Troubleshooting

### Benchmarks Taking Too Long

```bash
# Reduce sample size
cargo bench -- --sample-size 50

# Quick mode (less accurate but faster)
cargo bench -- --quick
```

### Inconsistent Results

- Close background applications
- Ensure laptop is plugged in (not on battery)
- Let system cool down between runs
- Check for system updates running

### Memory Issues

This crate uses minimal memory (~200-300 bytes per operation), so memory issues are unlikely. If you encounter them:

1. Check available system memory
2. Reduce benchmark sample size
3. Run benchmarks individually

## Further Reading

- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Benchmarking Best Practices](https://easyperf.net/blog/2018/08/26/Basics-of-profiling)

