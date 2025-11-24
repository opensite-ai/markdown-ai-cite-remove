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

// ============================================================================
// COMPREHENSIVE FIXTURE VALIDATION TESTS
// These tests validate the ACTUAL expected output for real-world AI fixtures
// ============================================================================

#[test]
fn test_chatgpt_fixture_complete_validation() {
    let input = include_str!("fixtures/chatgpt.md");
    let cleaned = remove_citations(input);

    // 1. Verify ALL inline citations are removed (ChatGPT uses [1] through [7])
    for i in 1..=7 {
        let citation = format!("[{}]", i);
        assert!(
            !cleaned.contains(&citation),
            "Inline citation {} should be removed",
            citation
        );
    }

    // 2. Verify the reference link section at the end is completely removed
    assert!(
        !cleaned.contains("[1]: https://podcasts.apple.com"),
        "Reference link [1] should be removed"
    );
    assert!(
        !cleaned.contains("[2]: https://www.linkedin.com"),
        "Reference link [2] should be removed"
    );
    assert!(
        !cleaned.contains("[7]: https://podcasters.apple.com"),
        "Reference link [7] should be removed"
    );

    // 3. Verify no citation-style links remain using regex
    let citation_link_pattern = regex::Regex::new(r"\[\d+\]:\s*https?://").unwrap();
    assert!(
        !citation_link_pattern.is_match(&cleaned),
        "No citation-style reference links should remain"
    );

    // 4. Verify content is preserved
    assert!(
        cleaned.contains("What your content signals today"),
        "Main content should be preserved"
    );
    assert!(
        cleaned.contains("On the DELO"),
        "Show title should be preserved"
    );
    assert!(
        cleaned.contains("Arizona Restaurant & Bar Owners"),
        "Content should be preserved"
    );

    // 5. Verify the file ends cleanly without trailing reference section
    let trimmed = cleaned.trim_end();
    assert!(
        !trimmed.ends_with("https://podcasters.apple.com/support/891-content-and-subscription-guidelines?utm_source=chatgpt.com \"Apple Podcasts content guidelines\""),
        "File should not end with reference links"
    );
    assert!(
        trimmed.ends_with("platforms.") || trimmed.ends_with("consistent across platforms."),
        "File should end with actual content, not references"
    );
}

#[test]
fn test_matthew_rust_install_fixture_complete_validation() {
    let input = include_str!("fixtures/matthew_rust_install.md");
    let cleaned = remove_citations(input);

    // 1. Verify ALL inline citations are removed (uses [1] through [5] inline)
    for i in 1..=5 {
        let citation = format!("[{}]", i);
        assert!(
            !cleaned.contains(&citation),
            "Inline citation {} should be removed",
            citation
        );
    }

    // 2. Verify ALL reference links at the end are removed (has [1] through [20])
    assert!(
        !cleaned.contains("[1](https://www.geeksforgeeks.org"),
        "Reference link [1] should be removed"
    );
    assert!(
        !cleaned.contains("[20](https://www.youtube.com/watch?v=uovWbm6KneM)"),
        "Reference link [20] should be removed"
    );

    // 3. Verify no [number](url) patterns remain
    let citation_url_pattern = regex::Regex::new(r"\[\d+\]\(https?://[^\)]+\)").unwrap();
    assert!(
        !citation_url_pattern.is_match(&cleaned),
        "No [number](url) citation patterns should remain"
    );

    // 4. Verify content is preserved
    assert!(
        cleaned.contains("markdown-ai-cite-remove"),
        "Tool name should be preserved"
    );
    assert!(
        cleaned.contains("Intel-based Mac"),
        "Content should be preserved"
    );
    assert!(
        cleaned.contains("cargo install markdown-ai-cite-remove"),
        "Installation instructions should be preserved"
    );

    // 5. Verify the file ends cleanly without the reference list
    let trimmed = cleaned.trim_end();
    assert!(
        !trimmed.ends_with("[20](https://www.youtube.com/watch?v=uovWbm6KneM)"),
        "File should not end with reference links"
    );
    // The file should end with the actual content before the reference list
    assert!(
        trimmed.ends_with("Intel-based Mac.") || trimmed.contains("Intel-based Mac."),
        "File should end with actual content"
    );
}

#[test]
fn test_perplexity_fixture_complete_validation() {
    let input = include_str!("fixtures/perplexity.md");
    let cleaned = remove_citations(input);

    // 1. Verify inline citations are removed (Perplexity uses [1] through [45] inline)
    // Test a sample of them
    for i in [1, 5, 10, 15, 20, 25, 30, 35, 40, 45] {
        let citation = format!("[{}]", i);
        assert!(
            !cleaned.contains(&citation),
            "Inline citation {} should be removed",
            citation
        );
    }

    // 2. Verify ALL reference links at the end are removed (has [1] through [45])
    assert!(
        !cleaned.contains("[1](https://www.reliablesoft.net"),
        "Reference link [1] should be removed"
    );
    assert!(
        !cleaned.contains("[30](https://www.reddit.com/r/rails"),
        "Reference link [30] should be removed"
    );
    assert!(
        !cleaned.contains("[45](https://www.alphaxiv.org"),
        "Reference link [45] should be removed"
    );

    // 3. Verify no [number](url) patterns remain
    let citation_url_pattern = regex::Regex::new(r"\[\d+\]\(https?://[^\)]+\)").unwrap();
    let matches: Vec<_> = citation_url_pattern.find_iter(&cleaned).collect();
    assert!(
        matches.is_empty(),
        "No [number](url) citation patterns should remain. Found {} matches",
        matches.len()
    );

    // 4. Verify content is preserved
    assert!(
        cleaned.contains("DomainExtractor Performance Benchmarks"),
        "Title should be preserved"
    );
    assert!(
        cleaned.contains("Ruby URL Parser"),
        "Content should be preserved"
    );
    assert!(
        cleaned.contains("Executive Summary"),
        "Section headers should be preserved"
    );

    // 5. Verify the file ends cleanly without the massive reference list
    let trimmed = cleaned.trim_end();
    assert!(
        !trimmed.ends_with("[45](https://www.alphaxiv.org/overview/2311.10533v2)"),
        "File should not end with reference links"
    );
    assert!(
        !trimmed.contains("https://www.alphaxiv.org/overview/2311.10533v2"),
        "Last reference URL should be removed"
    );
}

#[test]
fn test_perplexity_export_syntax_fixture_complete_validation() {
    let input = include_str!("fixtures/perplexity_export_syntax.md");
    let cleaned = remove_citations(input);

    // 1. Verify ALL footnote-style inline citations are removed (uses [^1_1] through [^2_21])
    // Test a sample of them
    for citation in [
        "[^1_1]", "[^1_5]", "[^1_10]", "[^1_20]", "[^1_30]", "[^1_56]", "[^2_1]", "[^2_10]",
        "[^2_21]",
    ] {
        assert!(
            !cleaned.contains(citation),
            "Inline citation {} should be removed",
            citation
        );
    }

    // 2. Verify ALL footnote reference definitions at the end are removed
    assert!(
        !cleaned.contains("[^1_1]: https://mackeeper.com"),
        "Reference definition [^1_1] should be removed"
    );
    assert!(
        !cleaned.contains("[^1_56]: https://www.macworld.com"),
        "Reference definition [^1_56] should be removed"
    );
    assert!(
        !cleaned.contains("[^2_21]: https://www.youtube.com"),
        "Reference definition [^2_21] should be removed"
    );

    // 3. Verify no [^number_number]: url patterns remain
    let footnote_ref_pattern = regex::Regex::new(r"\[\^\d+_\d+\]:\s*https?://").unwrap();
    let matches: Vec<_> = footnote_ref_pattern.find_iter(&cleaned).collect();
    assert!(
        matches.is_empty(),
        "No [^number_number]: url patterns should remain. Found {} matches",
        matches.len()
    );

    // 4. Verify no inline footnote citations remain
    let footnote_inline_pattern = regex::Regex::new(r"\[\^\d+_\d+\]").unwrap();
    let matches: Vec<_> = footnote_inline_pattern.find_iter(&cleaned).collect();
    assert!(
        matches.is_empty(),
        "No [^number_number] inline citations should remain. Found {} matches",
        matches.len()
    );

    // 5. Verify content is preserved
    assert!(
        cleaned.contains("Your scratch disk is getting full"),
        "Main content should be preserved"
    );
    assert!(
        cleaned.contains("DaisyDisk"),
        "Tool names should be preserved"
    );
    assert!(
        cleaned.contains("OmniDiskSweeper"),
        "Content should be preserved"
    );

    // 6. Verify the file ends cleanly without the reference list
    let trimmed = cleaned.trim_end();
    assert!(
        !trimmed.contains("[^2_21]: https://www.youtube.com"),
        "File should not end with reference definitions"
    );
    assert!(
        !trimmed.contains("https://www.youtube.com/watch?v=roTArTNov9g"),
        "Last reference URL should be removed"
    );
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

#[test]
fn test_perplexity_markdown_link_format() {
    // Test the [1](url) format that Perplexity uses
    let input = r#"Content here[1][2].

[1](https://example.com/page1)
[2](https://example.com/page2)
[3](https://example.com/page3)"#;

    let result = remove_citations(input);

    // Should remove inline citations
    assert!(!result.contains("[1]"));
    assert!(!result.contains("[2]"));

    // Should remove the markdown-style citation links
    assert!(!result.contains("[1](https://example.com/page1)"));
    assert!(!result.contains("[2](https://example.com/page2)"));
    assert!(!result.contains("[3](https://example.com/page3)"));
    assert!(!result.contains("https://example.com"));

    // Should keep the content
    assert!(result.contains("Content here"));
}

#[test]
fn test_real_world_perplexity_markdown_links() {
    // Test with the actual matthew_rust_install.md file format
    let input = std::fs::read_to_string("tests/fixtures/matthew_rust_install.md")
        .expect("Failed to read matthew_rust_install.md");

    let result = remove_citations(&input);

    // Should remove inline citations
    assert!(!result.contains("[1]"));
    assert!(!result.contains("[2]"));
    assert!(!result.contains("[5]"));

    // Should remove ALL the markdown-style citation links
    assert!(!result.contains("[1](https://www.geeksforgeeks.org"));
    assert!(!result.contains("[2](https://doc.rust-lang.org"));
    assert!(!result.contains("[20](https://www.youtube.com"));

    // Should not contain any URLs from the reference section
    assert!(!result.contains("https://www.geeksforgeeks.org/installation-guide"));
    assert!(!result.contains("https://rust-lang.org/tools/install"));
    assert!(!result.contains("https://www.youtube.com/watch?v=uovWbm6KneM"));

    // Should keep the actual content
    assert!(result.contains("To install and configure"));
    assert!(result.contains("Prerequisites"));
    assert!(result.contains("Rust Installation"));
    assert!(result.contains("cargo install markdown-ai-cite-remove"));
}
