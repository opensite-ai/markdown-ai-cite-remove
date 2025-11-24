use markdown_ai_cite_remove::clean;

fn main() {
    // Example 1: Simple inline citations
    let input1 = "AI research shows promise[1][2] in various fields[3].";
    let cleaned1 = clean(input1);
    println!("Example 1:");
    println!("Input:  {}", input1);
    println!("Output: {}", cleaned1);
    println!();

    // Example 2: Named citations
    let input2 = "Studies[source:1] indicate[ref:2] positive results.";
    let cleaned2 = clean(input2);
    println!("Example 2:");
    println!("Input:  {}", input2);
    println!("Output: {}", cleaned2);
    println!();

    // Example 3: With reference section
    let input3 = r#"Recent findings show improvement.

## References
[1]: https://example.com/paper1
[2]: https://example.com/paper2"#;
    let cleaned3 = clean(input3);
    println!("Example 3:");
    println!("Input:\n{}", input3);
    println!("\nOutput:\n{}", cleaned3);
    println!();

    // Example 4: Complex markdown with citations
    let input4 = r#"# Research Summary

This **important** study[1] demonstrates *significant* findings[2].

- Key point 1[3]
- Key point 2[4]

Check out [this link](https://example.com) for more information[5].

[1]: https://citation1.com
[2]: https://citation2.com"#;
    let cleaned4 = clean(input4);
    println!("Example 4:");
    println!("Input:\n{}", input4);
    println!("\nOutput:\n{}", cleaned4);
}
