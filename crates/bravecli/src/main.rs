use clap::{Parser, Subcommand};

mod search;
mod suggest;
mod summarizer;
mod value_parsers;

use brave::Brave;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = "Brave Web Search API client")]
pub struct Cli {
    /// Brave Search API subscription token.
    ///
    /// This token should correspond to one of the subscription models exposed by the Brave Search
    /// API.
    ///
    /// 1. Data for Search.
    /// 2. Data for AI.
    /// 3. Data with storage rights.
    /// 4. Spellcheck.
    /// 5. Suggest.
    ///
    /// You can also set individual tokens as environment variables for each subscription:
    ///
    /// NOTE: This option takes precedence over the following environment variables.
    ///
    /// Envs:
    ///
    /// BRAVE_WEB_SEARCH_DATA_FOR_AI_API_KEY
    ///     Brave Web Search Data for AI API Key (Free AI, Base AI, or Pro AI.)
    ///
    /// Options only available on `Pro`:
    ///
    /// - Schema-enriched Web results.
    /// - Infobox
    /// - FAQ
    /// - Discussions
    /// - Locations
    /// - Summarizer
    ///
    /// BRAVE_SUGGEST_API_KEY
    ///     Brave Suggest API Key (Free Autosuggest, Autosuggest.)
    #[clap(long, global = true, verbatim_doc_comment)]
    pub subscription_token: Option<String>,

    /// Hidden variable to handle environment variables without exposing the secret value to stdout.
    #[clap(
        long,
        env = "BRAVE_WEB_SEARCH_DATA_FOR_AI_API_KEY",
        global = true,
        verbatim_doc_comment,
        hide = true
    )]
    pub brave_web_search_data_for_ai_api_key: Option<String>,

    /// Hidden variable to handle environment variables without exposing the secret value to stdout.
    #[clap(long, env = "BRAVE_SUGGEST_API_KEY", global = true, verbatim_doc_comment, hide = true)]
    pub brave_suggest_api_key: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

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
    let mut cli = Cli::parse();

    log::debug!("Getting credentials");
    let credentials = Credentials {
        subscription_token: cli.subscription_token.take(),
        brave_web_search_data_for_ai_api_key: cli.brave_web_search_data_for_ai_api_key.take(),
        brave_suggest_api_key: cli.brave_suggest_api_key.take(),
    };

    log::debug!("Running command");
    match cli.command {
        Commands::Search(cli) => {
            crate::search::run(cli, get_client(credentials, Subscription::WebSearch)?)
        }
        Commands::Summarizer(cli) => {
            crate::summarizer::run(cli, get_client(credentials, Subscription::WebSearch)?)
        }
        Commands::Suggest(cli) => {
            crate::suggest::run(cli, get_client(credentials, Subscription::Suggest)?)
        }
    }
}

enum Subscription {
    Suggest,
    WebSearch,
}

struct Credentials {
    subscription_token: Option<String>,
    brave_web_search_data_for_ai_api_key: Option<String>,
    brave_suggest_api_key: Option<String>,
}

fn get_client(
    mut credentials: Credentials,
    subscription: Subscription,
) -> color_eyre::eyre::Result<Brave> {
    log::debug!("Creating Brave Client");

    let subscription_token = credentials.subscription_token.take();
    let brave_web_search_data_for_ai_api_key =
        credentials.brave_web_search_data_for_ai_api_key.take();
    let brave_suggest_api_key = credentials.brave_suggest_api_key.take();

    let token = match subscription {
        Subscription::Suggest => {
            if let Some(subscription_token) = subscription_token {
                Ok(subscription_token)
            } else if let Some(brave_suggest_api_key) = brave_suggest_api_key {
                Ok(brave_suggest_api_key)
            } else {
                Err(color_eyre::eyre::eyre!("No subscription token found"))
            }
        }
        Subscription::WebSearch => {
            if let Some(subscription_token) = subscription_token {
                Ok(subscription_token)
            } else if let Some(brave_web_search_data_for_ai_api_key) =
                brave_web_search_data_for_ai_api_key
            {
                Ok(brave_web_search_data_for_ai_api_key)
            } else {
                Err(color_eyre::eyre::eyre!("No subscription token found"))
            }
        }
    }?;

    Ok(Brave::new(brave::Auth::new(&token), "https://api.search.brave.com/res/v1"))
}
