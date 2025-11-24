use clap::Parser;
use markdown_ai_cite_remove::remove_citations;
use std::fs;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(name = "md-cite-remove")]
#[command(version, about = "Remove citations from AI-generated markdown", long_about = None)]
struct Cli {
    /// Input file (or stdin if not specified)
    input: Option<String>,

    /// Output file (or stdout if not specified)
    #[arg(short, long)]
    output: Option<String>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Read input
    let input = match cli.input {
        Some(path) => {
            if cli.verbose {
                eprintln!("Reading from file: {}", path);
            }
            fs::read_to_string(&path)?
        }
        None => {
            if cli.verbose {
                eprintln!("Reading from stdin...");
            }
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    // Remove citations
    if cli.verbose {
        eprintln!("Removing citations (input size: {} bytes)...", input.len());
    }
    let result = remove_citations(&input);

    if cli.verbose {
        eprintln!("Citations removed (output size: {} bytes)", result.len());
    }

    // Write output
    match cli.output {
        Some(path) => {
            if cli.verbose {
                eprintln!("Writing to file: {}", path);
            }
            fs::write(path, result)?
        }
        None => io::stdout().write_all(result.as_bytes())?,
    }

    if cli.verbose {
        eprintln!("Done!");
    }

    Ok(())
}
