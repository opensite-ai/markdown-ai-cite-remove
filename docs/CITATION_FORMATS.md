# Supported Citation Formats

This document describes all markdown citation formats supported by `markdown-ai-cite-remove`.

## Overview

The tool has been enhanced with comprehensive pattern matching based on deep research of citation formats used across:
- **AI Systems**: ChatGPT, Claude, Perplexity, Gemini, Copilot
- **Academic Markdown**: Pandoc, R Markdown, Quarto, MyST, Jupyter
- **Standard Markdown**: GitHub, MultiMarkdown, CommonMark

## Inline Citations (Removed)

### 1. Numeric Citations
**Format**: `[1]`, `[2]`, `[123]`  
**Used by**: ChatGPT, Claude, most AI systems  
**Example**:
```markdown
This is a fact[1] about something[2].
```
**After cleaning**:
```markdown
This is a fact about something.
```

### 2. Footnote-Style Citations
**Format**: `[^1]`, `[^note]`, `[^1_1]`, `[^1_23]`, `[^section_note]`  
**Used by**: Perplexity (export format), academic markdown, Pandoc  
**Example**:
```markdown
Research shows[^1_5] that this works[^2_10].
```
**After cleaning**:
```markdown
Research shows that this works.
```

**Supported patterns**:
- Simple numeric: `[^1]`, `[^2]`
- With underscores: `[^1_1]`, `[^1_23]`, `[^2_5]`
- Named: `[^note]`, `[^section]`, `[^main-point]`
- With hyphens: `[^section-1]`, `[^note-main]`

### 3. Named Citations
**Format**: `[source:1]`, `[ref:2]`, `[cite:3]`, `[fig:1]`, `[table:2]`, `[eq:1]`  
**Used by**: Custom academic formats, documentation systems  
**Example**:
```markdown
See figure[fig:1] and table[table:2] for details[ref:5].
```
**After cleaning**:
```markdown
See figure and table for details.
```

## Reference Definitions (Removed)

### 1. Standard Markdown Reference Links
**Format**: `[1]: https://example.com` or `[1]: https://example.com "Title"`  
**Used by**: ChatGPT, GitHub markdown  
**Example**:
```markdown
Content here.

[1]: https://example.com
[2]: https://another.com "Optional Title"
```
**After cleaning**:
```markdown
Content here.
```

### 2. Footnote Definitions
**Format**: `[^1]: Description` or `[^1_1]: https://example.com`  
**Used by**: Perplexity export, Pandoc, academic markdown  
**Example**:
```markdown
Content here.

[^1_1]: https://www.example.com/article
[^1_2]: https://www.another.com/source
[^2_1]: https://www.third.com/page
```
**After cleaning**:
```markdown
Content here.
```

### 3. Markdown Link Format
**Format**: `[1](https://example.com)` or `[^1_1](https://example.com)`  
**Used by**: Perplexity (standard format), some AI systems  
**Example**:
```markdown
Content here.

[1](https://www.example.com/source)
[2](https://www.another.com/article)
```
**After cleaning**:
```markdown
Content here.
```

### 4. Reference Entries
**Format**: `[1] Author, A. (2024). Title.`  
**Used by**: Academic citations, bibliography formats  
**Example**:
```markdown
Content here.

[1] Smith, J. (2024). Research Paper. Journal of Science.
[2] Doe, A. (2023). Another Study. Conference Proceedings.
```
**After cleaning**:
```markdown
Content here.
```

## Preserved Content

The tool is designed to preserve legitimate markdown content:

### Regular Markdown Links
```markdown
[Click here](https://example.com)
[Read more](./page.md)
```
✅ **Preserved** - Has descriptive text, not just numbers

### Code Blocks
```markdown
`array[1]`
```python
data[0]
```
✅ **Preserved** - Inside code blocks

### Array/Object Access
```markdown
Use array[1] to access the element.
```
✅ **Preserved** - Not at word boundary

### Markdown Images
```markdown
![Alt text](image.png)
```
✅ **Preserved** - Image syntax

## Test Coverage

All formats are validated with:
- **Unit tests**: 22 tests covering pattern matching
- **Integration tests**: 40 tests covering real-world scenarios
- **Fixture tests**: 4 comprehensive tests with actual AI-generated content
  - ChatGPT format (7 citations)
  - Perplexity standard format (45 citations)
  - Perplexity export format (77 footnote citations)
  - Matthew Rust install guide (20 citations)

## Future Enhancements

Pandoc-style citations (`[@smith2004]`, `@citation_key`) are planned for a future release.

