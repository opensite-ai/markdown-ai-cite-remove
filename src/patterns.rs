use once_cell::sync::Lazy;
use regex::Regex;

/// Compiled regex patterns for citation removal
pub(crate) struct Patterns {
    pub inline_numeric: Regex,
    pub inline_named: Regex,
    pub reference_link: Regex,
    pub reference_header: Regex,
    pub reference_entry: Regex,
    pub multiple_whitespace: Regex,
    #[allow(dead_code)]
    pub trailing_punctuation_space: Regex,
    pub excessive_newlines: Regex,
}

/// Lazily compiled patterns (compiled once, used many times)
pub(crate) static PATTERNS: Lazy<Patterns> = Lazy::new(|| Patterns {
    // Inline numeric citations: [1][2][123]
    inline_numeric: Regex::new(r"\[\d+\]").unwrap(),

    // Named source citations: [source:1][ref:2][cite:3][note:4]
    inline_named: Regex::new(r"\[(?:source|ref|cite|note):\d+\]").unwrap(),

    // Link-style references: [1]: https://... or [1] https://...
    // Matches both [1]: and [1] (space) formats
    reference_link: Regex::new(r"(?m)^\[\d+\](?::\s*|\s+).*$").unwrap(),

    // Reference section headers
    reference_header: Regex::new(
        r"(?m)^#{1,6}\s*(?:References?|Citations?|Sources?|Bibliography|Notes?)\s*$",
    )
    .unwrap(),

    // Full reference entries: [1] Author, A. (2024)...
    reference_entry: Regex::new(r"(?m)^\[\d+\]\s+[^\n]+$").unwrap(),

    // Multiple whitespace normalization
    multiple_whitespace: Regex::new(r" {2,}").unwrap(),

    // Trailing punctuation with extra spaces (e.g., "text. [1]" → "text.")
    trailing_punctuation_space: Regex::new(r"([.!?,;:])\s+(\[)").unwrap(),

    // Excessive newlines (3+ consecutive newlines)
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
    fn test_inline_numeric_pattern() {
        let patterns = Patterns::get();
        assert!(patterns.inline_numeric.is_match("[1]"));
        assert!(patterns.inline_numeric.is_match("[123]"));
        assert!(!patterns.inline_numeric.is_match("[abc]"));
        assert!(!patterns.inline_numeric.is_match("[]"));
    }

    #[test]
    fn test_inline_named_pattern() {
        let patterns = Patterns::get();
        assert!(patterns.inline_named.is_match("[source:1]"));
        assert!(patterns.inline_named.is_match("[ref:2]"));
        assert!(patterns.inline_named.is_match("[cite:3]"));
        assert!(patterns.inline_named.is_match("[note:4]"));
        assert!(!patterns.inline_named.is_match("[other:1]"));
    }

    #[test]
    fn test_reference_link_pattern() {
        let patterns = Patterns::get();
        assert!(patterns.reference_link.is_match("[1]: https://example.com"));
        assert!(patterns.reference_link.is_match("[2] https://example.com"));
        assert!(!patterns
            .reference_link
            .is_match("  [1]: https://example.com"));
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
}
