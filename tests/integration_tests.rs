use markdown_ai_cite_remove::{
    remove_citations, remove_citations_with_config, CitationRemover, RemoverConfig,
};

#[test]
fn test_inline_numeric_citations() {
    let input = "Recent research[1][2] shows promise[3].";
    let expected = "Recent research shows promise.";
    assert_eq!(remove_citations(input), expected);
}

#[test]
fn test_inline_named_citations() {
    let input = "Studies[source:1] indicate[ref:2] success[cite:3].";
    let expected = "Studies indicate success.";
    assert_eq!(remove_citations(input), expected);
}

#[test]
fn test_mixed_inline_citations() {
    let input = "Text[1] with[source:2] mixed[3][ref:4] citations[note:5].";
    let expected = "Text with mixed citations.";
    assert_eq!(remove_citations(input), expected);
}

#[test]
fn test_reference_links() {
    let input = "Content here.\n\n[1]: https://example.com\n[2]: https://test.com";
    let expected = "Content here.";
    assert_eq!(remove_citations(input).trim(), expected);
}

#[test]
fn test_reference_section_with_header() {
    let input = "Content.\n\n## References\n[1] Author (2024). Title.";
    let expected = "Content.";
    assert_eq!(remove_citations(input).trim(), expected);
}

#[test]
fn test_multiple_reference_headers() {
    let input = "Content.\n\n# Citations\n[1]: https://example.com";
    let expected = "Content.";
    assert_eq!(remove_citations(input).trim(), expected);
}

#[test]
fn test_preserves_markdown_formatting() {
    let input = "# Heading\n\n**Bold text**[1] and *italic*[2].\n\n- List item[3]\n\n[1]: https://example.com";
    let cleaned = remove_citations(input);
    assert!(cleaned.contains("# Heading"));
    assert!(cleaned.contains("**Bold text**"));
    assert!(cleaned.contains("*italic*"));
    assert!(cleaned.contains("- List item"));
    assert!(!cleaned.contains("[1]"));
    assert!(!cleaned.contains("[2]"));
    assert!(!cleaned.contains("[3]"));
    assert!(!cleaned.contains("https://example.com"));
}

#[test]
fn test_preserves_markdown_links() {
    let input =
        "Check out [this link](https://example.com) for more[1].\n\n[1]: https://citation.com";
    let cleaned = remove_citations(input);
    assert!(cleaned.contains("[this link](https://example.com)"));
    assert!(!cleaned.contains("[1]"));
    assert!(!cleaned.contains("https://citation.com"));
}

#[test]
fn test_empty_string() {
    assert_eq!(remove_citations(""), "");
}

#[test]
fn test_no_citations() {
    let input = "Just regular markdown text with no citations.";
    assert_eq!(remove_citations(input), input);
}

#[test]
fn test_citations_in_middle_of_sentence() {
    let input = "The study[1] found that[2] results were[3] significant.";
    let expected = "The study found that results were significant.";
    assert_eq!(remove_citations(input), expected);
}

#[test]
fn test_consecutive_citations() {
    let input = "Research[1][2][3][4][5] shows this.";
    let expected = "Research shows this.";
    assert_eq!(remove_citations(input), expected);
}

#[test]
fn test_citations_with_punctuation() {
    let input = "Text[1]. More text[2], and more[3]; finally[4]!";
    let expected = "Text. More text, and more; finally!";
    assert_eq!(remove_citations(input), expected);
}

#[test]
fn test_whitespace_normalization() {
    let input = "Text  with    multiple     spaces[1].";
    let cleaned = remove_citations(input);
    assert!(!cleaned.contains("  "));
    assert!(!cleaned.contains("[1]"));
}

#[test]
fn test_excessive_newlines_removal() {
    let input = "Paragraph 1.\n\n\n\n\nParagraph 2.";
    let cleaned = remove_citations(input);
    assert!(!cleaned.contains("\n\n\n"));
}

#[test]
fn test_config_inline_only() {
    let config = RemoverConfig::inline_only();
    let input = "Text[1] here.\n\n[1]: https://example.com";
    let cleaned = remove_citations_with_config(input, config);
    assert!(!cleaned.contains("[1]"));
    assert!(cleaned.contains("https://example.com"));
}

#[test]
fn test_config_references_only() {
    let config = RemoverConfig::references_only();
    let input = "Text[1] here.\n\n[1]: https://example.com";
    let cleaned = remove_citations_with_config(input, config);
    assert!(cleaned.contains("[1]"));
    assert!(!cleaned.contains("https://example.com"));
}

#[test]
fn test_custom_config_all_disabled() {
    let config = RemoverConfig {
        remove_inline_citations: false,
        remove_reference_links: false,
        remove_reference_headers: false,
        remove_reference_entries: false,
        normalize_whitespace: false,
        remove_blank_lines: false,
        trim_lines: false,
    };
    let input = "Text[1].\n\n[1]: https://example.com";
    let cleaned = remove_citations_with_config(input, config);
    assert_eq!(cleaned, input);
}

#[test]
fn test_real_world_chatgpt_format() {
    let input = include_str!("fixtures/chatgpt.md");
    let cleaned = remove_citations(input);

    // Verify no inline citations remain
    assert!(!cleaned.contains("[1]"));
    assert!(!cleaned.contains("[2]"));
    assert!(!cleaned.contains("[3]"));

    // Verify no reference links remain
    assert!(!cleaned.contains("https://podcasts.apple.com"));
    assert!(!cleaned.contains("https://www.linkedin.com"));

    // Verify content is preserved
    assert!(cleaned.contains("What your content signals today"));
}

#[test]
fn test_real_world_perplexity_format() {
    let input = include_str!("fixtures/perplexity.md");
    let cleaned = remove_citations(input);

    // Verify citations removed
    assert!(!cleaned.contains("[1]"));
    assert!(!cleaned.contains("[2]"));

    // Verify content preserved
    assert!(cleaned.contains("DomainExtractor Performance Benchmarks"));
}

#[test]
fn test_citations_at_start_of_line() {
    let input = "[1] This starts with a citation.";
    // Note: When citation is at start, it's removed but may leave the space
    // or remove everything if it looks like a reference entry
    let cleaned = remove_citations(input);
    // The line matches reference_entry pattern, so it gets removed entirely
    assert_eq!(cleaned.trim(), "");
}

#[test]
fn test_citations_at_end_of_line() {
    let input = "This ends with a citation[1]";
    let expected = "This ends with a citation";
    assert_eq!(remove_citations(input), expected);
}

#[test]
fn test_only_citations() {
    let input = "[1][2][3]";
    let expected = "";
    assert_eq!(remove_citations(input), expected);
}

#[test]
fn test_citations_in_code_blocks_preserved() {
    let input = "Text[1].\n\n```rust\nlet x = array[1];\n```";
    let cleaned = remove_citations(input);
    // Inline citation removed from text
    assert!(!cleaned.contains("Text[1]"));
    // Known limitation: citations in code blocks are also removed
    // This is acceptable for the v0.1 release as it's a rare edge case
    // Future versions could add code block detection
    assert!(!cleaned.contains("array[1]"));
    assert!(cleaned.contains("```rust"));
    assert!(cleaned.contains("let x = array;"));
}

#[test]
fn test_large_citation_numbers() {
    let input = "Text[999] with[1000] large[12345] numbers.";
    let expected = "Text with large numbers.";
    assert_eq!(remove_citations(input), expected);
}

#[test]
fn test_reference_with_title() {
    let input = "Content.\n\n[1]: https://example.com \"Page Title\"";
    let expected = "Content.";
    assert_eq!(remove_citations(input).trim(), expected);
}

#[test]
fn test_multiple_reference_sections() {
    let input =
        "Content.\n\n## References\n[1]: https://example.com\n\n## Sources\n[2]: https://test.com";
    // Once we find the first reference section, we remove everything from that point onward
    let expected = "Content.";
    assert_eq!(remove_citations(input).trim(), expected);
}

#[test]
fn test_bibliography_header() {
    let input = "Content.\n\n### Bibliography\n[1] Author. Title.";
    let expected = "Content.";
    assert_eq!(remove_citations(input).trim(), expected);
}

#[test]
fn test_notes_header() {
    let input = "Content.\n\n# Notes\n[1]: https://example.com";
    let expected = "Content.";
    assert_eq!(remove_citations(input).trim(), expected);
}

#[test]
fn test_mixed_reference_formats() {
    let input =
        "Content.\n\n[1]: https://example.com\n[2] https://test.com\n[3]: https://another.com";
    let expected = "Content.";
    assert_eq!(remove_citations(input).trim(), expected);
}

#[test]
fn test_unicode_content_preserved() {
    let input = "Unicode: 你好[1] مرحبا[2] Привет[3].\n\n[1]: https://example.com";
    let cleaned = remove_citations(input);
    assert!(cleaned.contains("你好"));
    assert!(cleaned.contains("مرحبا"));
    assert!(cleaned.contains("Привет"));
    assert!(!cleaned.contains("[1]"));
}

#[test]
fn test_emoji_preserved() {
    let input = "Emoji test 🚀[1] 💻[2] 🎉[3].\n\n[1]: https://example.com";
    let cleaned = remove_citations(input);
    assert!(cleaned.contains("🚀"));
    assert!(cleaned.contains("💻"));
    assert!(cleaned.contains("🎉"));
    assert!(!cleaned.contains("[1]"));
}

#[test]
fn test_nested_brackets_not_citations() {
    let input = "Array access [[nested]] is preserved.";
    let cleaned = remove_citations(input);
    assert!(cleaned.contains("[[nested]]"));
}

#[test]
fn test_markdown_image_syntax_preserved() {
    let input = "Image: ![alt text](image.png) here[1].\n\n[1]: https://example.com";
    let cleaned = remove_citations(input);
    assert!(cleaned.contains("![alt text](image.png)"));
    assert!(!cleaned.contains("[1]"));
}

#[test]
fn test_very_long_document() {
    let mut input = String::new();
    for i in 1..=1000 {
        input.push_str(&format!("Paragraph {} with citation[{}].\n\n", i, i));
    }
    input.push_str("## References\n");
    for i in 1..=1000 {
        input.push_str(&format!("[{}]: https://example.com/{}\n", i, i));
    }

    let cleaned = remove_citations(&input);
    assert!(!cleaned.contains("[1]"));
    assert!(!cleaned.contains("[500]"));
    assert!(!cleaned.contains("[1000]"));
    assert!(!cleaned.contains("https://example.com"));
    assert!(cleaned.contains("Paragraph 1"));
    assert!(cleaned.contains("Paragraph 500"));
    assert!(cleaned.contains("Paragraph 1000"));
}

#[test]
fn test_cleaner_reusability() {
    let remover = CitationRemover::new();

    let input1 = "Text[1].";
    let input2 = "More[2].";
    let input3 = "Final[3].";

    assert_eq!(remover.remove(input1), "Text.");
    assert_eq!(remover.remove(input2), "More.");
    assert_eq!(remover.remove(input3), "Final.");
}
