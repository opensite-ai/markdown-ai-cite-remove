use markdown_ai_cite_remove::{remove_citations_with_config, CitationRemover, RemoverConfig};

fn main() {
    let input = r#"Research shows results[1][2].

## References
[1]: https://example.com
[2]: https://test.com"#;

    println!("Original text:\n{}\n", input);
    println!("{}", "=".repeat(60));

    // Example 1: Remove only inline citations
    println!("\n1. Inline citations only:");
    let config1 = RemoverConfig::inline_only();
    let result1 = remove_citations_with_config(input, config1);
    println!("{}", result1);

    // Example 2: Remove only reference sections
    println!("\n2. Reference sections only:");
    let config2 = RemoverConfig::references_only();
    let result2 = remove_citations_with_config(input, config2);
    println!("{}", result2);

    // Example 3: Custom configuration
    println!("\n3. Custom config (no whitespace normalization):");
    let config3 = RemoverConfig {
        remove_inline_citations: true,
        remove_reference_links: true,
        remove_reference_headers: true,
        remove_reference_entries: true,
        normalize_whitespace: false,
        remove_blank_lines: false,
        trim_lines: false,
    };
    let result3 = remove_citations_with_config(input, config3);
    println!("{}", result3);

    // Example 4: Reusable remover instance
    println!("\n4. Reusable remover:");
    let remover = CitationRemover::with_config(RemoverConfig::default());

    let texts = [
        "First document[1].",
        "Second document[2][3].",
        "Third document[source:1].",
    ];

    for (i, text) in texts.iter().enumerate() {
        let result = remover.remove(text);
        println!("  Document {}: {} -> {}", i + 1, text, result);
    }
}
