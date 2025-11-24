# Documentation Index

Complete guide to all documentation for the `markdown-ai-cite-remove` crate.

## 📚 Main Documentation

### [README.md](../README.md) - Start Here!
**The main entry point for all users.**

Contains:
- Project overview and features
- Quick reference table
- Installation instructions (with prerequisites)
- Quick start examples
- Real-world use cases with code
- Library and CLI usage
- Performance metrics
- Testing guide
- Troubleshooting section
- Contributing guidelines

**Read this first** to understand what the crate does and how to use it.

---

## 🎯 Specialized Guides

### [CLI_GUIDE.md](guides/CLI_GUIDE.md) - Command-Line Tool
**Complete guide for using the `md-cite-clean` CLI tool.**

Contains:
- Prerequisites and installation steps
- Basic usage examples
- Advanced usage (batch processing, automation)
- Integration with other tools (pandoc, curl, git)
- CI/CD pipeline examples
- Common issues and solutions
- Tips and tricks

**Read this if** you want to use the command-line tool or automate citation removal.

---

### [BENCHMARKING.md](performance/BENCHMARKING.md) - Performance Guide
**Understanding benchmark results and performance metrics.**

Contains:
- How to run benchmarks
- Interpreting benchmark output
- Understanding outliers (why they're normal)
- Gnuplot explanation
- Performance comparison techniques
- Best practices for accurate benchmarks
- Troubleshooting benchmark issues

**Read this if** you:
- See "ignored tests" during `cargo bench`
- Wonder about performance outliers
- Want to compare performance changes
- Need to understand the "Gnuplot not found" warning

### [VIEWING_BENCHMARKS.md](performance/VIEWING_BENCHMARKS.md) - Benchmark Visualizations
**Quick guide to viewing interactive benchmark reports.**

Contains:
- How to view HTML reports with charts
- Report structure and navigation
- Understanding the visualizations
- Using baselines for comparison
- Tips and troubleshooting

**Read this if** you want to see visual charts and graphs of benchmark results.

---

### [FAQ.md](guides/FAQ.md) - Frequently Asked Questions
**Quick answers to common questions.**

Contains:
- Installation and setup questions
- Testing and benchmarking questions
- Usage questions (library and CLI)
- Performance questions
- Features and limitations
- Troubleshooting
- Contributing questions

**Read this if** you have a specific question or issue.

---

## 📖 Reference Documentation

### IMPLEMENTATION_SUMMARY.md - Project Overview
**Complete technical overview of the implementation.**

*Note: This file may have been moved or removed during reorganization. Check the repository root or docs directory.*

Contains:
- Project status and completion details
- Features implemented
- Test coverage statistics
- Project structure
- Usage examples
- Performance characteristics
- Key implementation details
- Next steps

**Read this if** you want to understand the technical implementation or contribute to the project.

---

### [API Documentation](https://docs.rs/markdown-ai-cite-remove)
**Full API reference (generated from code).**

View locally:
```bash
cargo doc --open
```

Contains:
- All public APIs with examples
- Module documentation
- Function signatures
- Type definitions
- Usage examples

**Read this if** you're using the library in your Rust code and need API details.

---

## 💡 Examples

### [examples/basic_usage.rs](../examples/basic_usage.rs)
Simple examples showing common use cases.

Run it:
```bash
cargo run --example basic_usage
```

### [examples/custom_config.rs](../examples/custom_config.rs)
Advanced examples with custom configuration.

Run it:
```bash
cargo run --example custom_config
```

---

## 🔍 QA Documentation

### QA_RESPONSES.md
Detailed responses to QA questions and documentation improvements made.

*Note: Check the docs/qa directory if this file exists.*

### bench.txt
Sample benchmark output for reference.

*Note: Check the docs/qa directory if this file exists.*

---

## 🗺️ Quick Navigation

### I want to...

**...get started quickly**
→ Read [README.md](../README.md) Quick Start section

**...install the CLI tool**
→ Read [CLI_GUIDE.md](guides/CLI_GUIDE.md) Installation section

**...use it in my Rust project**
→ Read [README.md](../README.md) Library Usage section

**...understand benchmark results**
→ Read [BENCHMARKING.md](performance/BENCHMARKING.md)

**...find answer to a specific question**
→ Read [FAQ.md](guides/FAQ.md)

**...batch process files**
→ Read [CLI_GUIDE.md](guides/CLI_GUIDE.md) Batch Processing section

**...integrate with CI/CD**
→ Read [CLI_GUIDE.md](guides/CLI_GUIDE.md) Automation Examples section

**...understand performance**
→ Read [README.md](../README.md) Performance section or [BENCHMARKING.md](performance/BENCHMARKING.md)

**...contribute to the project**
→ Read [README.md](../README.md) Contributing section

**...troubleshoot an issue**
→ Read [FAQ.md](guides/FAQ.md) or [README.md](../README.md) Troubleshooting section

---

## 📊 Documentation Statistics

- **Total documentation files**: 8
- **Total pages**: ~50+ pages of documentation
- **Code examples**: 30+ working examples
- **Use cases covered**: 15+ real-world scenarios
- **FAQ entries**: 40+ questions answered
- **Troubleshooting entries**: 10+ common issues

---

## 🎓 Learning Path

### For New Users

1. **Start**: [README.md](../README.md) - Overview and quick start
2. **Practice**: Run examples in `examples/` directory
3. **Deep Dive**: [CLI_GUIDE.md](guides/CLI_GUIDE.md) or API docs depending on use case
4. **Reference**: [FAQ.md](guides/FAQ.md) for specific questions

### For Contributors

1. **Start**: Check repository root for IMPLEMENTATION_SUMMARY.md
2. **Setup**: [README.md](../README.md) Contributing section
3. **Understand**: Read source code with `cargo doc --open`
4. **Test**: Run `cargo test` and `cargo bench`

### For Performance Tuning

1. **Start**: [README.md](../README.md) Performance section
2. **Deep Dive**: [BENCHMARKING.md](performance/BENCHMARKING.md)
3. **Measure**: Run `cargo bench` with baselines
4. **Optimize**: Use profiling tools if needed

---

## 🔄 Documentation Updates

This documentation is actively maintained. Last updated: 2025-01-24

### Recent Additions
- ✅ CLI_GUIDE.md - Complete CLI documentation
- ✅ BENCHMARKING.md - Benchmark explanation guide
- ✅ FAQ.md - Frequently asked questions
- ✅ Enhanced README.md with troubleshooting
- ✅ QA_RESPONSES.md - QA documentation

### Planned Additions
- Video tutorials (if community interest)
- More real-world examples
- Integration guides for popular frameworks
- Performance optimization guide

---

## 📞 Getting Help

If you can't find what you're looking for:

1. **Check [FAQ.md](guides/FAQ.md)** - Most common questions are answered
2. **Search documentation** - Use your editor's search function
3. **Check examples** - Working code in `examples/` directory
4. **Open an issue** - On GitHub with your question
5. **Email** - contact@opensite.ai

---

## 📝 Contributing to Documentation

Documentation improvements are welcome! To contribute:

1. Fork the repository
2. Make your changes
3. Test that examples still work
4. Submit a pull request

See [README.md](../README.md) Contributing section for details.

