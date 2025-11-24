use crate::config::CleanerConfig;
use crate::patterns::Patterns;

/// Main citation cleaner
pub struct CitationCleaner {
    config: CleanerConfig,
    patterns: &'static Patterns,
}

impl CitationCleaner {
    /// Create new cleaner with default configuration
    pub fn new() -> Self {
        Self {
            config: CleanerConfig::default(),
            patterns: Patterns::get(),
        }
    }

    /// Create cleaner with custom configuration
    pub fn with_config(config: CleanerConfig) -> Self {
        Self {
            config,
            patterns: Patterns::get(),
        }
    }

    /// Clean markdown string, removing all citations
    pub fn clean(&self, markdown: &str) -> String {
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

    /// Remove inline citations: [1][2][source:3]
    fn remove_inline_citations(&self, text: &str) -> String {
        let mut result = text.to_string();

        // Remove numeric citations [1][2]
        result = self
            .patterns
            .inline_numeric
            .replace_all(&result, "")
            .to_string();

        // Remove named citations [source:1][ref:2]
        result = self
            .patterns
            .inline_named
            .replace_all(&result, "")
            .to_string();

        result
    }

    /// Remove reference sections at end of document
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

            // Check for reference link or entry
            if self.patterns.reference_link.is_match(line)
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

impl Default for CitationCleaner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_inline_numeric() {
        let cleaner = CitationCleaner::new();
        let input = "Text[1] with[2] citations[3].";
        let result = cleaner.remove_inline_citations(input);
        assert_eq!(result, "Text with citations.");
    }

    #[test]
    fn test_remove_inline_named() {
        let cleaner = CitationCleaner::new();
        let input = "Text[source:1] with[ref:2] citations.";
        let result = cleaner.remove_inline_citations(input);
        assert_eq!(result, "Text with citations.");
    }

    #[test]
    fn test_normalize_whitespace() {
        let cleaner = CitationCleaner::new();
        let input = "Text  with    multiple     spaces.";
        let result = cleaner.normalize_whitespace(input);
        assert_eq!(result, "Text with multiple spaces.");
    }

    #[test]
    fn test_remove_excessive_blank_lines() {
        let cleaner = CitationCleaner::new();
        let input = "Line 1\n\n\n\n\nLine 2";
        let result = cleaner.remove_excessive_blank_lines(input);
        assert_eq!(result, "Line 1\n\nLine 2");
    }

    #[test]
    fn test_trim_all_lines() {
        let cleaner = CitationCleaner::new();
        let input = "Line 1   \nLine 2  \nLine 3 ";
        let result = cleaner.trim_all_lines(input);
        assert_eq!(result, "Line 1\nLine 2\nLine 3");
    }

    #[test]
    fn test_remove_reference_sections_with_header() {
        let cleaner = CitationCleaner::new();
        let input = "Content here.\n\n## References\n[1]: https://example.com";
        let result = cleaner.remove_reference_sections(input);
        assert_eq!(result.trim(), "Content here.");
    }

    #[test]
    fn test_remove_reference_sections_without_header() {
        let cleaner = CitationCleaner::new();
        let input = "Content here.\n\n[1]: https://example.com\n[2]: https://test.com";
        let result = cleaner.remove_reference_sections(input);
        assert_eq!(result.trim(), "Content here.");
    }

    #[test]
    fn test_custom_config() {
        let config = CleanerConfig {
            remove_inline_citations: true,
            remove_reference_links: false,
            remove_reference_headers: false,
            remove_reference_entries: false,
            normalize_whitespace: false,
            remove_blank_lines: false,
            trim_lines: false,
        };
        let cleaner = CitationCleaner::with_config(config);
        let input = "Text[1].\n\n[1]: https://example.com";
        let result = cleaner.clean(input);
        assert!(!result.contains("[1]"));
        assert!(result.contains("https://example.com"));
    }

    #[test]
    fn test_full_pipeline() {
        let cleaner = CitationCleaner::new();
        let input = "Text[1]  with   spaces.\n\n\n\n## References\n[1]: https://example.com";
        let result = cleaner.clean(input);
        assert!(!result.contains("[1]"));
        assert!(!result.contains("https://example.com"));
        assert!(!result.contains("  "));
        assert!(!result.contains("\n\n\n"));
    }
}
