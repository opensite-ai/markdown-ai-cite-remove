use clap::Parser;
use markdown_ai_cite_remove::clean;
use std::io::{self, Read, Write};
use std::fs;

#[derive(Parser)]
#[command(name = "md-cite-clean")]
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
    
    // Clean markdown
    if cli.verbose {
        eprintln!("Cleaning markdown (input size: {} bytes)...", input.len());
    }
    let cleaned = clean(&input);
    
    if cli.verbose {
        eprintln!("Cleaned markdown (output size: {} bytes)", cleaned.len());
    }
    
    // Write output
    match cli.output {
        Some(path) => {
            if cli.verbose {
                eprintln!("Writing to file: {}", path);
            }
            fs::write(path, cleaned)?
        }
        None => {
            io::stdout().write_all(cleaned.as_bytes())?
        }
    }
    
    if cli.verbose {
        eprintln!("Done!");
    }
    
    Ok(())
}

