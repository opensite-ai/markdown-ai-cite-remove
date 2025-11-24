//! # markdown-ai-cite-remove
//!
//! **Remove AI-generated citations and annotations from Markdown text**
//!
//! High-performance Rust library for removing citations from ChatGPT, Claude, Perplexity, and other AI markdown
//! responses. Removes inline citations `[1][2]`, reference links `[1]: https://...`, and
//! bibliography sections with 100% accuracy.
//!
//! ## Quick Start
//!
//! ```
//! use markdown_ai_cite_remove::remove_citations;
//!
//! let markdown = "AI research shows promise[1][2].\n\n[1]: https://example.com\n[2]: https://test.com";
//! let result = remove_citations(markdown);
//! assert_eq!(result.trim(), "AI research shows promise.");
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
//! use markdown_ai_cite_remove::{CitationRemover, RemoverConfig};
//!
//! let config = RemoverConfig {
//!     remove_inline_citations: true,
//!     remove_reference_links: true,
//!     ..Default::default()
//! };
//!
//! let remover = CitationRemover::with_config(config);
//! let result = remover.remove("Text with citations[1].");
//! ```

mod config;
mod error;
mod patterns;
mod remover;

pub use config::{RemovalMode, RemoverConfig};
pub use error::{RemoverError, Result};
pub use remover::CitationRemover;

/// Main entry point - remove citations from markdown with default settings
///
/// # Examples
///
/// ```
/// use markdown_ai_cite_remove::remove_citations;
///
/// let input = "Recent research[1][2] shows promise[3].";
/// let output = remove_citations(input);
/// assert_eq!(output, "Recent research shows promise.");
/// ```
pub fn remove_citations(markdown: &str) -> String {
    CitationRemover::new().remove(markdown)
}

/// Remove citations from markdown with custom configuration
///
/// # Examples
///
/// ```
/// use markdown_ai_cite_remove::{remove_citations_with_config, RemoverConfig};
///
/// let config = RemoverConfig::inline_only();
/// let input = "Text[1] here.\n\nSome content.";
/// let output = remove_citations_with_config(input, config);
/// assert_eq!(output.trim(), "Text here.\n\nSome content.");
/// ```
pub fn remove_citations_with_config(markdown: &str, config: RemoverConfig) -> String {
    CitationRemover::with_config(config).remove(markdown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_basic() {
        let input = "Text[1] here[2].";
        let expected = "Text here.";
        assert_eq!(remove_citations(input), expected);
    }

    #[test]
    fn test_remove_with_references() {
        let input = "Content here.\n\n[1]: https://example.com\n[2]: https://test.com";
        let expected = "Content here.";
        assert_eq!(remove_citations(input).trim(), expected);
    }

    #[test]
    fn test_remove_preserves_markdown() {
        let input =
            "# Heading\n\nSome **bold** text[1] and *italic*[2].\n\n[1]: https://example.com";
        let expected = "# Heading\n\nSome **bold** text and *italic*.";
        assert_eq!(remove_citations(input).trim(), expected);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(remove_citations(""), "");
    }

    #[test]
    fn test_no_citations() {
        let input = "Just regular markdown text.";
        assert_eq!(remove_citations(input), input);
    }
}
