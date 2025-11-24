# Changelog - v0.2.0

## 🎉 Major Release: Complete Naming Refactor

**Release Date**: 2025-01-24  
**Version**: 0.2.0 (Breaking Changes)

---

## 🔥 Breaking Changes

### 1. CLI Binary Renamed
- **Old**: `md-cite-clean`
- **New**: `md-cite-remove` ✅

### 2. Main API Functions Renamed
- **Old**: `clean()`
- **New**: `remove_citations()` ✅

- **Old**: `clean_with_config()`
- **New**: `remove_citations_with_config()` ✅

### 3. Struct Names Changed
- **Old**: `CitationCleaner`
- **New**: `CitationRemover` ✅

- **Old**: `CleanerConfig`
- **New**: `RemoverConfig` ✅

- **Old**: `CleanerError`
- **New**: `RemoverError` ✅

### 4. Method Names Changed
- **Old**: `.clean()`
- **New**: `.remove()` ✅

---

## ✨ New Features

### CLI Installed by Default
- **Old**: Required `cargo install markdown-ai-cite-remove --features cli`
- **New**: Simply `cargo install markdown-ai-cite-remove` ✅

The CLI feature is now enabled by default, making installation seamless.

---

## 📝 Migration Guide

### For CLI Users

**Before (v0.1.0)**:
```bash
cargo install markdown-ai-cite-remove --features cli
md-cite-clean input.md -o output.md
```

**After (v0.2.0)**:
```bash
cargo install markdown-ai-cite-remove
md-cite-remove input.md -o output.md
```

### For Library Users

**Before (v0.1.0)**:
```rust
use markdown_ai_cite_remove::{clean, CitationCleaner, CleanerConfig};

let result = clean("Text[1]");

let cleaner = CitationCleaner::new();
let cleaned = cleaner.clean("Text[1]");

let config = CleanerConfig::default();
```

**After (v0.2.0)**:
```rust
use markdown_ai_cite_remove::{remove_citations, CitationRemover, RemoverConfig};

let result = remove_citations("Text[1]");

let remover = CitationRemover::new();
let result = remover.remove("Text[1]");

let config = RemoverConfig::default();
```

---

## 🎯 Why These Changes?

### Problem 1: Confusing Installation
Users had to remember to add `--features cli` flag, which was non-intuitive.

### Problem 2: Inconsistent Naming
- Library name: `markdown-ai-cite-remove`
- CLI command: `md-cite-clean` ❌ (inconsistent!)
- Main function: `clean()` ❌ (vague!)
- Struct: `CitationCleaner` ❌ (doesn't match library name!)

### Solution: Unified "Remove" Terminology
Everything now uses consistent "remove" terminology that matches the library name and purpose.

---

## ✅ What's Fixed

1. ✅ **CLI installs by default** - No more `--features cli` needed
2. ✅ **Consistent naming** - Everything uses "remove" terminology
3. ✅ **Clear and declarative** - Function names clearly state what they do
4. ✅ **Professional** - Naming matches industry standards

---

## 📊 Testing

All tests pass with new naming:
- ✅ 58/58 tests passing
- ✅ 0 clippy warnings
- ✅ All examples updated
- ✅ All benchmarks updated
- ✅ All documentation updated

---

## 📚 Documentation Updates

All documentation has been updated with the new naming:
- ✅ README.md
- ✅ docs/guides/CLI_GUIDE.md
- ✅ docs/guides/FAQ.md
- ✅ docs/performance/BENCHMARKING.md
- ✅ docs/performance/VIEWING_BENCHMARKS.md
- ✅ docs/DOCUMENTATION_INDEX.md
- ✅ API documentation (inline docs)

---

## 🚀 Quick Start (v0.2.0)

### Installation
```bash
cargo install markdown-ai-cite-remove
```

### CLI Usage
```bash
echo "Text[1] with citations[2]." | md-cite-remove
```

### Library Usage
```rust
use markdown_ai_cite_remove::remove_citations;

let result = remove_citations("Text[1] with citations[2].");
```

---

## 🔄 Upgrade Path

### Step 1: Update Dependency
```toml
[dependencies]
markdown-ai-cite-remove = "0.2.0"
```

### Step 2: Update Code
Use find-and-replace in your codebase:
1. `CitationCleaner` → `CitationRemover`
2. `CleanerConfig` → `RemoverConfig`
3. `clean(` → `remove_citations(`
4. `clean_with_config(` → `remove_citations_with_config(`
5. `.clean(` → `.remove(`

### Step 3: Update CLI Commands
Replace `md-cite-clean` with `md-cite-remove` in scripts.

### Step 4: Test
```bash
cargo test
```

---

## 📦 What's Included

- ✅ Core library with new naming
- ✅ CLI binary (`md-cite-remove`)
- ✅ Comprehensive documentation
- ✅ 58 tests (all passing)
- ✅ Performance benchmarks
- ✅ Usage examples

---

## 🙏 Thank You

Thank you for using markdown-ai-cite-remove! This breaking change improves the developer experience and makes the library more professional and easier to use.

If you encounter any issues during migration, please open an issue on GitHub.

---

**Full Changelog**: v0.1.0...v0.2.0

