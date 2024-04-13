use clap::Parser;


#[derive(Debug, Parser)]
#[command(name = "run")]
#[command(about = "Run Anthropic's Claud with RAG obtained through Brave's API")]
pub struct Cli {
    /// Prompt to execute
    prompt: String,
}

pub fn execute(cli: &Cli) {
    println!("{cli:?}");
}
