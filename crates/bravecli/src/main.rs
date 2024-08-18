use clap::{Parser, Subcommand};
use color_eyre::eyre::OptionExt;

mod search;
mod summarizer;
mod value_parsers;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Query the general web with Brave's search api
    #[clap(name = "search")]
    Search(crate::search::Cli),
    /// Query the general web with Brave's summarizer api
    ///
    /// > *NOTE*
    /// > Access to summarizer is available through the [Pro AI](https://api.search.brave.com/app/subscriptions/subscribe?tab=ai) plan.
    #[clap(name = "summarizer", verbatim_doc_comment)]
    Summarizer(crate::summarizer::Cli),
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = "Brave HTTP API client")]
pub struct Cli {
    /// Custom path for the configuration file
    #[clap(short = 'T', long, env = "BRAVE_SUBSCRIPTION_TOKEN", global = true)]
    pub subscription_token: Option<String>,
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

    run()
}

fn run() -> color_eyre::eyre::Result<()> {
    log::debug!("Parsing CLI arguments");
    let cli = Cli::parse();
    let subscription_token = cli.subscription_token.ok_or_eyre("can't find subscription token")?;

    log::debug!("Running command");
    match cli.command {
        Commands::Search(cli) => crate::search::run(cli, &subscription_token),
        Commands::Summarizer(cli) => crate::summarizer::run(cli, &subscription_token),
    }
}
