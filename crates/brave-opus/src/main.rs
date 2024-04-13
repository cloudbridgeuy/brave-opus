#![allow(clippy::empty_line_after_outer_attr)]
mod run;

use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run Anthropic Claude 3 using Brave's API as RAG
    #[clap(name = "run")]
    Run(crate::run::Cli),
}

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Run Anthropic Claude's LLM with RAG taken from Brave's API"
)]

pub struct Cli {
    /// Anthropic API Key
    #[clap(short, long, env = "ANTHROPIC_API_KEY", global = true)]
    pub anthropic_api_key: Option<String>,
    /// Brave API Key
    #[clap(short, long, env = "BRAVE_API_KEY", global = true)]
    pub brave_api_key: Option<String>,
    #[command(subcommand)]
    pub command: Commands,
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init();

    ctrlc::set_handler(move || {
        log::error!("Ctrl-C received, stopping the program");
        std::process::exit(1);
    })?;

    run();
    Ok(())
}

fn run() {
    log::debug!("Parsing CLI arguments");
    let cli = Cli::parse();

    log::debug!("Running command");
    match cli.command {
        Commands::Run(cli) => {
            crate::run::execute(&cli);
        }
    }
}
