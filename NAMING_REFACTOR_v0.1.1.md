# Naming Refactor for v0.1.1

## Issues Fixed

### 1. CLI Installation Problem
**Problem**: CLI binary wasn't installed by default, requiring `--features cli` flag  
**Solution**: Changed `default = []` to `default = ["cli"]` in Cargo.toml

### 2. Naming Confusion
**Problem**: Inconsistent naming throughout codebase:
- Library: `markdown-ai-cite-remove`
- CLI command: `md-cite-clean` ❌
- Main function: `clean()` ❌
- Struct: `CitationCleaner` ❌

**Solution**: Unified naming with "remove" terminology:
- Library: `markdown-ai-cite-remove` ✅
- CLI command: `md-cite-remove` ✅
- Main function: `remove_citations()` ✅
- Struct: `CitationRemover` ✅

## Code Changes

### Core Library Files

| File | Old Name | New Name |
|------|----------|----------|
| `src/cleaner.rs` | → | `src/remover.rs` |
| `CitationCleaner` | → | `CitationRemover` |
| `CleanerConfig` | → | `RemoverConfig` |
| `CleanerError` | → | `RemoverError` |
| `clean()` | → | `remove_citations()` |
| `clean_with_config()` | → | `remove_citations_with_config()` |
| `.clean()` method | → | `.remove()` method |

### Binary

| Old | New |
|-----|-----|
| `md-cite-clean` | `md-cite-remove` |

### Variables

| Old | New |
|-----|-----|
| `cleaner` | `remover` |
| `cleaned` | `result` |

## Files Updated

✅ `Cargo.toml` - Binary name and default features  
✅ `src/lib.rs` - Public API  
✅ `src/cleaner.rs` → `src/remover.rs` - Renamed and updated  
✅ `src/config.rs` - Config struct names  
✅ `src/error.rs` - Error type names  
✅ `src/bin/cli.rs` - CLI binary  
✅ `tests/integration_tests.rs` - All tests  
✅ `benches/citation_removal.rs` - All benchmarks  
✅ `examples/basic_usage.rs` - Example code  
✅ `examples/custom_config.rs` - Example code  

## Verification

```bash
# All tests pass
cargo test
# Result: 58/58 tests passed ✅

# Clippy clean
cargo clippy --all-targets --all-features -- -D warnings
# Result: No warnings ✅

# CLI works
echo "Test[1]" | cargo run --bin md-cite-remove
# Result: "Test" ✅
```

## Documentation Updates Needed

The following documentation files need to be updated with the new naming:

- [ ] README.md
- [ ] docs/DOCUMENTATION_INDEX.md
- [ ] docs/guides/CLI_GUIDE.md
- [ ] docs/guides/FAQ.md
- [ ] docs/performance/BENCHMARKING.md
- [ ] docs/performance/VIEWING_BENCHMARKS.md
- [ ] DEPLOYMENT_CHECKLIST.md

### Search and Replace Patterns

1. `md-cite-clean` → `md-cite-remove`
2. `CitationCleaner` → `CitationRemover`
3. `CleanerConfig` → `RemoverConfig`
4. `clean(` → `remove_citations(`
5. `clean_with_config(` → `remove_citations_with_config(`
6. `.clean(` → `.remove(`
7. `cleaner` → `remover` (variable names)
8. `cleaned` → `result` (variable names)
9. "clean markdown" → "remove citations"
10. "cleaning" → "removing citations"

## Next Steps

1. Update all documentation files
2. Run final verification
3. Publish v0.1.1 to crates.io

## Version

- **Old**: v0.1.0
- **New**: v0.1.1
- **Date**: 2025-01-24

