use markdown_ai_cite_remove::{clean_with_config, CitationCleaner, CleanerConfig};

fn main() {
    let input = r#"Research shows results[1][2].

## References
[1]: https://example.com
[2]: https://test.com"#;

    println!("Original text:\n{}\n", input);
    println!("{}", "=".repeat(60));

    // Example 1: Remove only inline citations
    println!("\n1. Inline citations only:");
    let config1 = CleanerConfig::inline_only();
    let result1 = clean_with_config(input, config1);
    println!("{}", result1);

    // Example 2: Remove only reference sections
    println!("\n2. Reference sections only:");
    let config2 = CleanerConfig::references_only();
    let result2 = clean_with_config(input, config2);
    println!("{}", result2);

    // Example 3: Custom configuration
    println!("\n3. Custom config (no whitespace normalization):");
    let config3 = CleanerConfig {
        remove_inline_citations: true,
        remove_reference_links: true,
        remove_reference_headers: true,
        remove_reference_entries: true,
        normalize_whitespace: false,
        remove_blank_lines: false,
        trim_lines: false,
    };
    let result3 = clean_with_config(input, config3);
    println!("{}", result3);

    // Example 4: Reusable cleaner instance
    println!("\n4. Reusable cleaner:");
    let cleaner = CitationCleaner::with_config(CleanerConfig::default());
    
    let texts = vec![
        "First document[1].",
        "Second document[2][3].",
        "Third document[source:1].",
    ];
    
    for (i, text) in texts.iter().enumerate() {
        let cleaned = cleaner.clean(text);
        println!("  Document {}: {} -> {}", i + 1, text, cleaned);
    }
}

