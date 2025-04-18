use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    
    /// The path to the file to read
    #[arg(default_value = ".")]
    path: std::path::PathBuf,

    /// Search files recursively
    #[clap(short, long)]
    recursive: bool,
}

fn check_file(path: &std::path::PathBuf, pattern: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;
    println!("{:?}", path);
    for line in content.lines() {
        if line.contains(&pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}

// Function to search through a directory and its subdirectories
fn check_dir(path: &std::path::PathBuf, pattern: &str, recursive: &bool) {
    // if recursive is false search only the specific path
    if !recursive {
        check_file(path, pattern).expect("Error"); 
    }
    
    // if recursive is true, search through subdirectories

}

fn main()  {
    let args = Cli::parse();

    let path = args.path;
    let pattern = args.pattern;

    check_dir(&path, &pattern, &args.recursive);
}
