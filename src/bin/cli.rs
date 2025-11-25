use clap::Parser;
use markdown_ai_cite_remove::remove_citations;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Parser)]
#[command(name = "mdcr")]
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
    let (input, input_path) = match cli.input {
        Some(path) => {
            if cli.verbose {
                eprintln!("Reading from file: {}", path);
            }
            let content = fs::read_to_string(&path)?;
            (content, Some(path))
        }
        None => {
            if cli.verbose {
                eprintln!("Reading from stdin...");
            }
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            (buffer, None)
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

    // Determine output path
    let output_path = match cli.output {
        Some(path) => Some(path),
        None => {
            // If input was a file and no output specified, auto-generate output filename
            input_path.map(|input| {
                let path = Path::new(&input);
                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("output");
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("md");
                format!("{}__cite_removed.{}", stem, ext)
            })
        }
    };

    // Write output
    match output_path {
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
