use crate::config::RemoverConfig;
use crate::patterns::Patterns;

/// Main citation remover
pub struct CitationRemover {
    config: RemoverConfig,
    patterns: &'static Patterns,
}

impl CitationRemover {
    /// Create new remover with default configuration
    pub fn new() -> Self {
        Self {
            config: RemoverConfig::default(),
            patterns: Patterns::get(),
        }
    }

    /// Create remover with custom configuration
    pub fn with_config(config: RemoverConfig) -> Self {
        Self {
            config,
            patterns: Patterns::get(),
        }
    }

    /// Remove citations from markdown string
    pub fn remove(&self, markdown: &str) -> String {
        let mut result = markdown.to_string();

        // Step 1: Remove reference sections FIRST (before inline citations)
        // This is important because inline citation removal would break reference link patterns
        if self.config.remove_reference_links
            || self.config.remove_reference_entries
            || self.config.remove_reference_headers
        {
            result = self.remove_reference_sections(&result);
        }

        // Step 2: Remove inline citations
        if self.config.remove_inline_citations {
            result = self.remove_inline_citations(&result);
        }

        // Step 3: Cleanup whitespace
        if self.config.normalize_whitespace {
            result = self.normalize_whitespace(&result);
        }

        // Step 4: Remove excessive blank lines
        if self.config.remove_blank_lines {
            result = self.remove_excessive_blank_lines(&result);
        }

        // Step 5: Trim lines
        if self.config.trim_lines {
            result = self.trim_all_lines(&result);
        }

        result
    }

    /// Remove ALL inline citations using comprehensive pattern matching
    /// Handles: `[1]`, `[^1]`, `[^1_1]`, `[source:1]`, `[@smith2004]`, `@citation`
    fn remove_inline_citations(&self, text: &str) -> String {
        // Use the unified comprehensive pattern that matches ALL citation formats
        self.patterns
            .inline_citations
            .replace_all(text, "")
            .to_string()
    }

    /// Remove reference sections at end of document
    /// Handles: `[1]: url`, `[^1]: text`, `[^1_1]: url`, `[1](url)`, `[^1_1](url)`
    fn remove_reference_sections(&self, text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let mut references_start = None;

        // Scan for reference section start (find the FIRST occurrence)
        for (i, line) in lines.iter().enumerate() {
            // Skip if we already found the start
            if references_start.is_some() {
                break;
            }

            // Check for reference header
            if self.config.remove_reference_headers && self.patterns.reference_header.is_match(line)
            {
                references_start = Some(i);
                break;
            }

            // Check for ANY reference definition format using comprehensive pattern
            if self.patterns.reference_definitions.is_match(line)
                || self.patterns.reference_entry.is_match(line)
            {
                references_start = Some(i);
                break;
            }
        }

        // Remove everything from references onward
        if let Some(start) = references_start {
            lines[..start].join("\n")
        } else {
            text.to_string()
        }
    }

    /// Normalize multiple spaces to single space
    fn normalize_whitespace(&self, text: &str) -> String {
        self.patterns
            .multiple_whitespace
            .replace_all(text, " ")
            .to_string()
    }

    /// Remove excessive blank lines (3+ consecutive newlines → 2)
    fn remove_excessive_blank_lines(&self, text: &str) -> String {
        self.patterns
            .excessive_newlines
            .replace_all(text, "\n\n")
            .to_string()
    }

    /// Trim whitespace from all lines
    fn trim_all_lines(&self, text: &str) -> String {
        text.lines()
            .map(|line| line.trim_end())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for CitationRemover {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_inline_numeric() {
        let remover = CitationRemover::new();
        let input = "Text[1] with[2] citations[3].";
        let result = remover.remove_inline_citations(input);
        assert_eq!(result, "Text with citations.");
    }

    #[test]
    fn test_remove_inline_named() {
        let remover = CitationRemover::new();
        let input = "Text[source:1] with[ref:2] citations.";
        let result = remover.remove_inline_citations(input);
        assert_eq!(result, "Text with citations.");
    }

    #[test]
    fn test_normalize_whitespace() {
        let remover = CitationRemover::new();
        let input = "Text  with    multiple     spaces.";
        let result = remover.normalize_whitespace(input);
        assert_eq!(result, "Text with multiple spaces.");
    }

    #[test]
    fn test_remove_excessive_blank_lines() {
        let remover = CitationRemover::new();
        let input = "Line 1\n\n\n\n\nLine 2";
        let result = remover.remove_excessive_blank_lines(input);
        assert_eq!(result, "Line 1\n\nLine 2");
    }

    #[test]
    fn test_trim_all_lines() {
        let remover = CitationRemover::new();
        let input = "Line 1   \nLine 2  \nLine 3 ";
        let result = remover.trim_all_lines(input);
        assert_eq!(result, "Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_remove_reference_sections_with_header() {
        let remover = CitationRemover::new();
        let input = "Content here.\n\n## References\n[1]: https://example.com";
        let result = remover.remove_reference_sections(input);
        assert_eq!(result.trim(), "Content here.");
    }

    #[test]
    fn test_remove_reference_sections_without_header() {
        let remover = CitationRemover::new();
        let input = "Content here.\n\n[1]: https://example.com\n[2]: https://test.com";
        let result = remover.remove_reference_sections(input);
        assert_eq!(result.trim(), "Content here.");
    }

    #[test]
    fn test_custom_config() {
        let config = RemoverConfig {
            remove_inline_citations: true,
            remove_reference_links: false,
            remove_reference_headers: false,
            remove_reference_entries: false,
            normalize_whitespace: false,
            remove_blank_lines: false,
            trim_lines: false,
        };
        let remover = CitationRemover::with_config(config);
        let input = "Text[1].\n\n[1]: https://example.com";
        let result = remover.remove(input);
        assert!(!result.contains("[1]"));
        assert!(result.contains("https://example.com"));
    }

    #[test]
    fn test_full_pipeline() {
        let remover = CitationRemover::new();
        let input = "Text[1]  with   spaces.\n\n\n\n## References\n[1]: https://example.com";
        let result = remover.remove(input);
        assert!(!result.contains("[1]"));
        assert!(!result.contains("https://example.com"));
        assert!(!result.contains("  "));
        assert!(!result.contains("\n\n\n"));
    }
}
