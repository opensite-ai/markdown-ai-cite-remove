//! # markdown-ai-cite-remove
//!
//! **Remove AI-generated citations and annotations from Markdown text**
//!
//! High-performance Rust library for cleaning ChatGPT, Claude, Perplexity, and other AI markdown
//! responses. Removes inline citations `[1][2]`, reference links `[1]: https://...`, and
//! bibliography sections with 100% accuracy.
//!
//! ## Quick Start
//!
//! ```
//! use markdown_ai_cite_remove::clean;
//!
//! let markdown = "AI research shows promise[1][2].\n\n[1]: https://example.com\n[2]: https://test.com";
//! let cleaned = clean(markdown);
//! assert_eq!(cleaned.trim(), "AI research shows promise.");
//! ```
//!
//! ## Features
//!
//! - ✅ Remove inline numeric citations `[1][2][3]`
//! - ✅ Remove named citations `[source:1][ref:2]`
//! - ✅ Remove reference link lists `[1]: https://...`
//! - ✅ Remove reference section headers `## References`
//! - ✅ Remove bibliographic entries
//! - ✅ Preserve markdown formatting
//! - ✅ Whitespace normalization
//! - ✅ Ultra-fast performance (100+ MB/s throughput)
//!
//! ## Custom Configuration
//!
//! ```
//! use markdown_ai_cite_remove::{CitationCleaner, CleanerConfig};
//!
//! let config = CleanerConfig {
//!     remove_inline_citations: true,
//!     remove_reference_links: true,
//!     ..Default::default()
//! };
//!
//! let cleaner = CitationCleaner::with_config(config);
//! let cleaned = cleaner.clean("Text with citations[1].");
//! ```

mod cleaner;
mod config;
mod error;
mod patterns;

pub use cleaner::CitationCleaner;
pub use config::{CleanerConfig, RemovalMode};
pub use error::{CleanerError, Result};

/// Main entry point - clean markdown with default settings
///
/// # Examples
///
/// ```
/// use markdown_ai_cite_remove::clean;
///
/// let input = "Recent research[1][2] shows promise[3].";
/// let output = clean(input);
/// assert_eq!(output, "Recent research shows promise.");
/// ```
pub fn clean(markdown: &str) -> String {
    CitationCleaner::new().clean(markdown)
}

/// Clean markdown with custom configuration
///
/// # Examples
///
/// ```
/// use markdown_ai_cite_remove::{clean_with_config, CleanerConfig};
///
/// let config = CleanerConfig::inline_only();
/// let input = "Text[1] here.\n\nSome content.";
/// let output = clean_with_config(input, config);
/// assert_eq!(output.trim(), "Text here.\n\nSome content.");
/// ```
pub fn clean_with_config(markdown: &str, config: CleanerConfig) -> String {
    CitationCleaner::with_config(config).clean(markdown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_basic() {
        let input = "Text[1] here[2].";
        let expected = "Text here.";
        assert_eq!(clean(input), expected);
    }

    #[test]
    fn test_clean_with_references() {
        let input = "Content here.\n\n[1]: https://example.com\n[2]: https://test.com";
        let expected = "Content here.";
        assert_eq!(clean(input).trim(), expected);
    }

    #[test]
    fn test_clean_preserves_markdown() {
        let input =
            "# Heading\n\nSome **bold** text[1] and *italic*[2].\n\n[1]: https://example.com";
        let expected = "# Heading\n\nSome **bold** text and *italic*.";
        assert_eq!(clean(input).trim(), expected);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(clean(""), "");
    }

    #[test]
    fn test_no_citations() {
        let input = "Just regular markdown text.";
        assert_eq!(clean(input), input);
    }
}
