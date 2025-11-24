/// Configuration options for citation removal
#[derive(Debug, Clone)]
pub struct RemoverConfig {
    /// Remove inline citations like [1][2]
    pub remove_inline_citations: bool,

    /// Remove reference link lists at bottom
    pub remove_reference_links: bool,

    /// Remove reference section headers (## References)
    pub remove_reference_headers: bool,

    /// Remove full bibliographic entries
    pub remove_reference_entries: bool,

    /// Normalize whitespace after removal
    pub normalize_whitespace: bool,

    /// Remove blank lines left by removed sections
    pub remove_blank_lines: bool,

    /// Trim trailing whitespace from lines
    pub trim_lines: bool,
}

impl Default for RemoverConfig {
    fn default() -> Self {
        Self {
            remove_inline_citations: true,
            remove_reference_links: true,
            remove_reference_headers: true,
            remove_reference_entries: true,
            normalize_whitespace: true,
            remove_blank_lines: true,
            trim_lines: true,
        }
    }
}

impl RemoverConfig {
    /// Create a new configuration with all features enabled
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a configuration that only removes inline citations
    pub fn inline_only() -> Self {
        Self {
            remove_inline_citations: true,
            remove_reference_links: false,
            remove_reference_headers: false,
            remove_reference_entries: false,
            normalize_whitespace: true,
            remove_blank_lines: false,
            trim_lines: true,
        }
    }

    /// Create a configuration that only removes reference sections
    pub fn references_only() -> Self {
        Self {
            remove_inline_citations: false,
            remove_reference_links: true,
            remove_reference_headers: true,
            remove_reference_entries: true,
            normalize_whitespace: true,
            remove_blank_lines: true,
            trim_lines: true,
        }
    }
}

/// Mode for handling different citation styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemovalMode {
    /// Remove all citation types
    All,
    /// Remove only inline citations, keep reference lists
    InlineOnly,
    /// Remove only reference lists, keep inline citations
    ReferencesOnly,
}
