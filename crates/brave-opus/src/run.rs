use clap::Parser;
use color_eyre::eyre::Result;

#[derive(Debug, Parser)]
#[command(name = "run")]
#[command(about = "Run Anthropic's Claud with RAG obtained through Brave's API")]
pub struct Cli {
    /// Prompt to execute
    prompt: String,
}

pub async fn execute(cli: Cli) -> Result<()> {
    println!("{:?}", cli);
    Ok(())
}
