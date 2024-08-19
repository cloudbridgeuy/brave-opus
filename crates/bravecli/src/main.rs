use clap::{Parser, Subcommand};
use color_eyre::eyre::OptionExt;

mod search;
mod suggest;
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
    /// Query to generate potential suggestions for a given query
    #[clap(name = "suggest")]
    Suggest(crate::suggest::Cli),
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = "Brave HTTP API client")]
pub struct Cli {
    /// Brave Web Search Data for AI API Key (Free AI, Base AI, or Pro AI.)
    ///
    /// Options only available on `Pro`:
    ///
    /// - Schema-enriched Web results.
    /// - Infobox
    /// - FAQ
    /// - Discussions
    /// - Locations
    /// - Summarizer
    #[clap(
        long,
        env = "BRAVE_WEB_SEARCH_DATA_FOR_AI_API_KEY",
        global = true,
        verbatim_doc_comment
    )]
    pub brave_web_search_data_for_ai_api_key: Option<String>,

    /// Brave Suggest API Key (Free Autosuggest, Autosuggest.)
    #[clap(long, env = "BRAVE_SUGGEST_API_KEY", global = true, verbatim_doc_comment)]
    pub brave_suggest_api_key: Option<String>,

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
    let brave_web_search_data_for_ai_api_key = cli
        .brave_web_search_data_for_ai_api_key
        .ok_or_eyre("can't find subscription token for the Brave Web Search API.")?;
    let brave_suggest_api_key = cli
        .brave_suggest_api_key
        .ok_or_eyre("can't find subscription token for the Brave Suggest API.")?;

    log::debug!("Running command");
    match cli.command {
        Commands::Search(cli) => crate::search::run(cli, &brave_web_search_data_for_ai_api_key),
        Commands::Summarizer(cli) => {
            crate::summarizer::run(cli, &brave_web_search_data_for_ai_api_key)
        }
        Commands::Suggest(cli) => crate::suggest::run(cli, &brave_suggest_api_key),
    }
}
