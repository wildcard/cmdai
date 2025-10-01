use clap::Parser;

/// cmdai - Convert natural language to shell commands using local LLMs
#[derive(Parser)]
#[command(name = "cmdai")]
#[command(about = "Convert natural language to shell commands", long_about = None)]
#[command(version)]
struct Cli {
    /// Natural language task description
    prompt: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let _cli = Cli::parse();

    println!(
        "cmdai v{} - Natural Language to Shell Commands",
        env!("CARGO_PKG_VERSION")
    );
    println!("This is a placeholder implementation. Full functionality coming soon.");

    Ok(())
}
