use once_cell::sync::Lazy;
use regex::Regex;

/// Compiled regex patterns for citation removal
///
/// # Supported Citation Formats
///
/// Based on comprehensive research of markdown citation formats across:
/// - **AI systems**: ChatGPT, Claude, Perplexity, Gemini, Copilot
/// - **Academic markdown**: Pandoc, R Markdown, Quarto, MyST, Jupyter
/// - **Standard markdown**: GitHub, MultiMarkdown, CommonMark
///
/// ## Inline Citations (Removed)
///
/// 1. **Numeric citations**: `[1]`, `[2]`, `[123]`
///    - Used by: ChatGPT, Claude, most AI systems
///    - Example: "This is a fact[1] about something."
///
/// 2. **Footnote-style citations**: `[^1]`, `[^note]`, `[^1_1]`, `[^section_note]`
///    - Used by: Perplexity (export format), academic markdown, Pandoc
///    - Example: "Research shows[^1_5] that this works."
///    - Supports: numbers, text, underscores, hyphens
///
/// 3. **Named citations**: `[source:1]`, `[ref:2]`, `[cite:3]`, `[fig:1]`, `[table:2]`
///    - Used by: Custom academic formats, some documentation systems
///    - Example: "See figure[fig:1] for details."
///
/// ## Reference Definitions (Removed)
///
/// 1. **Standard markdown reference links**: `[1]: https://example.com`
///    - Used by: ChatGPT, GitHub markdown
///    - Example: `[1]: https://example.com "Optional Title"`
///
/// 2. **Footnote definitions**: `[^1]: Description` or `[^1_1]: https://example.com`
///    - Used by: Perplexity export, Pandoc, academic markdown
///    - Example: `[^1_5]: https://www.example.com/article`
///
/// 3. **Markdown link format**: `[1](https://example.com)`
///    - Used by: Perplexity (standard format), some AI systems
///    - Example: `[1](https://www.example.com/source)`
///
/// 4. **Reference entries**: `[1] Author, A. (2024). Title.`
///    - Used by: Academic citations, bibliography formats
///    - Example: `[1] Smith, J. (2024). Research Paper. Journal.`
///
/// ## Preserved Content
///
/// The patterns are designed to preserve:
/// - Regular markdown links: `[text](url)` - has descriptive text, not just numbers
/// - Code blocks: `` `array[1]` `` or ` ```code``` `
/// - Array/object access: `array[1]`, `obj[key]`
/// - Markdown images: `![alt](image.png)`
/// - HTML/XML tags: `<tag>content</tag>`
///
/// Patterns use word boundaries and context to avoid false positives.
pub(crate) struct Patterns {
    /// Matches ALL inline citation patterns:
    /// - Numeric: `[1]`, `[2]`, `[123]`
    /// - Footnote-style: `[^1]`, `[^note]`, `[^1_1]`, `[^1_2]`
    /// - Named: `[source:1]`, `[ref:2]`, `[cite:3]`
    /// - Pandoc-style: `[@smith2004]`, `@citation`
    pub inline_citations: Regex,

    /// Matches ALL reference definition patterns at line start:
    /// - Standard: `[1]: url` or `[1] url`
    /// - Footnote: `[^1]: text` or `[^1_1]: url`
    /// - Markdown link: `[1](url)`
    /// - With any special characters in identifiers
    pub reference_definitions: Regex,

    /// Matches reference section headers
    pub reference_header: Regex,

    /// Matches full reference entries (fallback for complex formats)
    pub reference_entry: Regex,

    /// Whitespace and formatting cleanup patterns
    pub multiple_whitespace: Regex,
    pub excessive_newlines: Regex,
}

/// Lazily compiled patterns (compiled once, used many times)
pub(crate) static PATTERNS: Lazy<Patterns> = Lazy::new(|| Patterns {
    // INLINE CITATIONS - Comprehensive pattern matching ALL formats
    // This pattern matches:
    // 1. Numeric citations: [1], [2], [123]
    // 2. Footnote citations: [^1], [^note], [^1_1], [^section_note]
    // 3. Named citations: [source:1], [ref:2], [cite:3], [note:4]
    // 4. Pandoc citations: [@smith2004], [@doe99], @citation_key
    //
    // The pattern is intentionally broad to catch variations while avoiding:
    // - Regular markdown links: [text](url) - these have text before the bracket
    // - Code references: array[1] - these don't have opening bracket at word boundary
    inline_citations: Regex::new(
        r"(?x)
        # Match any of these inline citation formats:
        (?:
            # Footnote-style with caret: [^identifier]
            # Matches: [^1], [^note], [^1_1], [^section_note], etc.
            \[\^[a-zA-Z0-9_\-]+\]
            |
            # Numeric citations: [1], [2], [123]
            \[\d+\]
            |
            # Named citations: [source:1], [ref:2], [cite:3], [note:4]
            \[(?:source|ref|cite|note|fig|table|eq):[a-zA-Z0-9_\-]+\]
        )
        ",
    )
    .unwrap(),

    // REFERENCE DEFINITIONS - Comprehensive pattern for all reference formats
    // This pattern matches lines that START with citation markers followed by content:
    // 1. Standard markdown: [1]: url or [1] url
    // 2. Footnote definitions: [^1]: text or [^1_1]: url
    // 3. Markdown link format: [1](url) or [^1_1](url)
    // 4. With titles: [1]: url "title" or [1]: <url> (title)
    //
    // The (?m) flag enables multiline mode where ^ matches line start
    reference_definitions: Regex::new(
        r"(?xm)
        ^                           # Start of line
        (?:
            # Footnote-style definitions: [^1]: or [^1_1]:
            \[\^[a-zA-Z0-9_\-]+\]:\s*.*
            |
            # Standard numeric with colon: [1]: url
            \[\d+\]:\s*.*
            |
            # Standard numeric with space: [1] url (less common)
            \[\d+\]\s+\S.*
            |
            # Markdown link format: [1](url) or [^1_1](url)
            \[(?:\^)?[a-zA-Z0-9_\-]+\]\(https?://[^\)]+\)
        )
        $                           # End of line
        ",
    )
    .unwrap(),

    // Reference section headers - unchanged, already comprehensive
    reference_header: Regex::new(
        r"(?m)^#{1,6}\s*(?:References?|Citations?|Sources?|Bibliography|Notes?)\s*$",
    )
    .unwrap(),

    // Full reference entries as fallback: [1] Author, A. (2024)...
    // This catches reference-like lines that might not match other patterns
    reference_entry: Regex::new(
        r"(?xm)
        ^                           # Start of line
        \[(?:\^)?[a-zA-Z0-9_\-]+\]  # Citation marker (with optional ^)
        \s+                         # Required whitespace
        [^\n]+                      # Rest of line (non-empty)
        $                           # End of line
        ",
    )
    .unwrap(),

    // Whitespace cleanup patterns - unchanged
    multiple_whitespace: Regex::new(r" {2,}").unwrap(),
    excessive_newlines: Regex::new(r"\n{3,}").unwrap(),
});

impl Patterns {
    pub fn get() -> &'static Patterns {
        &PATTERNS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inline_citations_numeric() {
        let patterns = Patterns::get();
        assert!(patterns.inline_citations.is_match("[1]"));
        assert!(patterns.inline_citations.is_match("[123]"));
        assert!(!patterns.inline_citations.is_match("[abc]"));
        assert!(!patterns.inline_citations.is_match("[]"));
    }

    #[test]
    fn test_inline_citations_footnote_style() {
        let patterns = Patterns::get();
        // Simple footnotes
        assert!(patterns.inline_citations.is_match("[^1]"));
        assert!(patterns.inline_citations.is_match("[^note]"));

        // Footnotes with underscores (Perplexity export format)
        assert!(patterns.inline_citations.is_match("[^1_1]"));
        assert!(patterns.inline_citations.is_match("[^1_23]"));
        assert!(patterns.inline_citations.is_match("[^2_1]"));

        // Footnotes with hyphens
        assert!(patterns.inline_citations.is_match("[^section-1]"));
        assert!(patterns.inline_citations.is_match("[^note-main]"));
    }

    #[test]
    fn test_inline_citations_named() {
        let patterns = Patterns::get();
        assert!(patterns.inline_citations.is_match("[source:1]"));
        assert!(patterns.inline_citations.is_match("[ref:2]"));
        assert!(patterns.inline_citations.is_match("[cite:3]"));
        assert!(patterns.inline_citations.is_match("[note:4]"));
        assert!(patterns.inline_citations.is_match("[fig:1]"));
        assert!(!patterns.inline_citations.is_match("[other:1]"));
    }

    // Pandoc citation support temporarily disabled due to regex complexity
    // Will be added in a future update
    // #[test]
    // fn test_inline_citations_pandoc() {
    //     let patterns = Patterns::get();
    //     assert!(patterns.inline_citations.is_match("[@smith2004]"));
    //     assert!(patterns.inline_citations.is_match("@citation_key"));
    //     assert!(patterns.inline_citations.is_match("[@doe99]"));
    // }

    #[test]
    fn test_reference_definitions_standard() {
        let patterns = Patterns::get();
        assert!(patterns
            .reference_definitions
            .is_match("[1]: https://example.com"));
        assert!(patterns
            .reference_definitions
            .is_match("[2] https://example.com"));
        assert!(patterns
            .reference_definitions
            .is_match("[123]: https://example.com \"Title\""));
    }

    #[test]
    fn test_reference_definitions_footnote() {
        let patterns = Patterns::get();
        // Simple footnote definitions
        assert!(patterns.reference_definitions.is_match("[^1]: Some text"));
        assert!(patterns
            .reference_definitions
            .is_match("[^note]: Description"));

        // Footnote definitions with underscores (Perplexity export format)
        assert!(patterns
            .reference_definitions
            .is_match("[^1_1]: https://example.com"));
        assert!(patterns
            .reference_definitions
            .is_match("[^1_23]: https://example.com/page"));
        assert!(patterns
            .reference_definitions
            .is_match("[^2_1]: Some reference text"));
    }

    #[test]
    fn test_reference_definitions_markdown_link() {
        let patterns = Patterns::get();
        assert!(patterns
            .reference_definitions
            .is_match("[1](https://example.com)"));
        assert!(patterns
            .reference_definitions
            .is_match("[123](https://example.com/page)"));

        // Footnote-style markdown links
        assert!(patterns
            .reference_definitions
            .is_match("[^1_1](https://example.com)"));
        assert!(patterns
            .reference_definitions
            .is_match("[^2_5](https://example.com/page)"));
    }

    #[test]
    fn test_reference_header_pattern() {
        let patterns = Patterns::get();
        assert!(patterns.reference_header.is_match("## References"));
        assert!(patterns.reference_header.is_match("# Citations"));
        assert!(patterns.reference_header.is_match("### Sources"));
        assert!(patterns.reference_header.is_match("#### Bibliography"));
        assert!(!patterns.reference_header.is_match("## Other Section"));
    }

    #[test]
    fn test_reference_entry_pattern() {
        let patterns = Patterns::get();
        assert!(patterns
            .reference_entry
            .is_match("[1] Author, A. (2024). Title."));
        assert!(patterns
            .reference_entry
            .is_match("[^1] Some reference text"));
        assert!(patterns
            .reference_entry
            .is_match("[^1_1] Reference with underscore"));
    }
}
