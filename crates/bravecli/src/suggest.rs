use clap::Parser;
use color_eyre::eyre::Result;

use brave::{apis::SuggestSearchParams, suggest::Api, Brave};

#[derive(Debug, Parser)]
#[command(name = "suggest")]
#[command(about = "Generate potential suggestions for a given query")]
pub struct Cli {
    /// The user's search query term.
    ///
    /// Maximum 400 characters and 50 words in the query.
    #[clap(value_parser = crate::value_parsers::q_value_parser)]
    q: String,

    /// Suggest Search API Version.
    ///
    /// The Brave Suggest Search API version to use. This is denoted by the format `YYYY-MM-DD`. The
    /// latest version is used by default, and the previous ones can be found in the API Changelog.
    #[clap(long)]
    version: Option<String>,

    /// The search query country.
    ///
    /// The country string is limited to 2 character country code of supported countries. For a list
    /// of supported values, see [Country
    /// Codes](https://api.search.brave.com/app/documentation/web-search/codes#country-codes)
    #[clap(long)]
    country: Option<String>,

    /// The search language preference.
    ///
    /// The 2 or more character language code for which the suggest search results are provided.
    /// This is just a hint for calculating suggest responses. For a list of complete values, see [Language
    /// Codes.](https://api.search.brave.com/app/documentation/web-search/codes#language-codes)
    #[clap(long)]
    lang: Option<String>,

    // The number of suggestions returned. This is done as best effort. The maximum is 20.
    #[clap(long, value_parser = clap::value_parser!(u16).range(1..20))]
    count: Option<u16>,

    /// Whether to enhance suggestions with rich results. This is an extra option in plans which
    /// needs to be enabled.
    #[clap(long)]
    rich: bool,
}

impl From<Cli> for SuggestSearchParams {
    fn from(cli: Cli) -> Self {
        Self {
            q: cli.q,
            country: cli.country,
            lang: cli.lang,
            count: cli.count,
            rich: if cli.rich { Some("1".to_string()) } else { None },
        }
    }
}

pub fn run(mut cli: Cli, subscription_token: &str) -> Result<()> {
    let client =
        Brave::new(brave::Auth::new(subscription_token), "https://api.search.brave.com/res/v1");

    let version = cli.version.take();
    let params: SuggestSearchParams = cli.into();

    let response = client.suggest(&params, version.as_deref())?;
    println!("{}", serde_json::to_string_pretty(&response)?);

    Ok(())
}
