use clap::Parser;
use color_eyre::eyre::Result;

use brave::{apis::WebSearchParams, summarizer::Api, Brave};

#[derive(Debug, Parser)]
#[command(name = "summarizer", verbatim_doc_comment)]
/// Query the general web with Brave's summarizer api
///
/// > *NOTE*
/// > Access to summarizer is available through the [Pro AI](https://api.search.brave.com/app/subscriptions/subscribe?tab=ai) plan.
pub struct Cli {
    /// The user's search query term.
    ///
    /// Maximum 400 characters and 50 words in the query.
    #[clap(value_parser = crate::value_parsers::q_value_parser)]
    q: String,

    /// The search query country.
    ///
    /// The country string is limited to 2 character country code of supported countries. For a list
    /// of supported values, see [Country
    /// Codes](https://api.search.brave.com/app/documentation/web-search/codes#country-codes)
    #[clap(long)]
    country: Option<String>,

    /// The search language preference.
    ///
    /// The language code, consisting of two or more characters, for which the search results are
    /// provided. For a list of possible values, see [Language
    /// Codes.](https://api.search.brave.com/app/documentation/web-search/codes#language-codes)
    #[clap(long = "lang")]
    search_lang: Option<String>,

    /// User interface language preferred in response.
    ///
    /// Usually in the formmat `<language-code>-<country-code>`, see [RFC
    /// 9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-accept-language) for a list of
    /// supported values.
    #[clap(long = "ui-lang")]
    ui_lang: Option<String>,

    /// The number of search results returned in response.
    ///
    /// The maximum is 20. The actual number delivered may be less than requested. Combine this
    /// parameter with `offset` to paginate search results.
    #[clap(long, value_parser = clap::value_parser!(u16).range(1..20))]
    count: Option<u16>,

    /// The zero based offset for search results per page (count) to skip before returning the
    /// result. The maximum is `9`. The actual number delivered may be less than requested based on
    /// the query. In order to paginate results use this parameter together with `count`. For
    /// example, if your user interface displays 20 search results per page, set `count` to 20 and
    /// `offset` to `0` to show the first page of results. To get subsequent pages, increment
    /// `offset` by `1` (e.g. `0`, `1`, `2`.) The results may overlap across multiple pages.
    #[clap(long, value_parser = clap::value_parser!(u16).range(1..9))]
    offset: Option<u16>,

    /// Filters search results for adult content.
    ///
    /// The following values are supported:
    ///
    /// - `off`: Adult content is included where relevant.
    /// - `moderate`: Adult text but no adult images or videos.
    /// - `strict`: No adult content with adult text, images, and videos.
    #[clap(long, verbatim_doc_comment, value_parser = ["off", "moderate", "strict"])]
    safesearch: Option<String>,

    /// Filters search results by when they were discovered.
    ///
    /// The following time deltas are supported:
    ///
    /// - `pd`: Discovered within the last 24 hours.
    /// - `pw`: Discovered within the last 7 days.
    /// - `pm`: Discovered within the last 31 days.
    /// - `YYYY-MM-DDtoYYYY-MM-DD`: A timeframe is also supported by specifying the data range (e.g. `2022-04-01-to2024-07-30`)
    #[clap(long, verbatim_doc_comment, value_parser = crate::value_parsers::freshness_value_parser)]
    freshness: Option<String>,

    /// Whether display strings (e.g. result snippets) should include decoration markers (e.g.
    /// highlighting characters.)
    #[clap(long)]
    text_decorations: bool,

    /// Whether to spellcheck provided query. If the spellchecker is enabled, the modified query is
    /// always used for the search. The modified query can be found in `altered` key from the
    /// `query` response model.
    #[clap(long)]
    spellcheck: bool,

    /// A comma delimited string of result types to include in the search response.
    ///
    /// Not specifying this parameter will return back all result types in search response where data is available and a plan with the corresponding option is subscribed. The response always includes `query` and `type` to identify and query modifications and response type respectively.
    ///
    /// Available result filter values are:
    ///
    /// - `discussions`
    /// - `faq`
    /// - `infobox`
    /// - `news`
    /// - `query`
    /// - `summarizer`
    /// - `videos`
    /// - `web`
    ///
    /// Example results filter values of `discussions,videos` returns only `query`, `discussions`, and `video` responses.
    #[clap(long, verbatim_doc_comment, value_parser=crate::value_parsers::result_filter_value_parser)]
    result_filter: Option<String>,

    /// Goggles act as a custom re-ranking on top of Brave's search index. For more details refer to
    /// the [Goggles repository.](https://github.com/brave/goggles-quickstart)
    #[clap(long)]
    goggles_id: Option<String>,

    /// The measurement units.
    ///
    /// If not provided, units are derived from search country. Possible values are:
    ///
    /// - `metric`: The standardized measurement system.
    /// - `imperial`: The British Imperial system of units.
    #[clap(long, verbatim_doc_comment)]
    units: Option<String>,

    /// A snippet is an excerpt from a page you get as a result of the query, and `extra_snippets`
    /// allow you to get up to `5` additional, alternative `excerpts`.
    ///
    /// Only available under `Free AI`, `Base AI`, `Pro AI`, `Base Data`, `Pro Data`, and
    /// `Custom` plans.
    #[clap(long)]
    extra_snippets: bool,

    /// This parameter enables summary key generation in web search results. This param is only
    /// required when enabling summarizer version `2024-04-23` onwards.
    ///
    /// Summarizer version `2023-08-25` is deprecated.
    #[clap(long)]
    summary: bool,
}

impl From<Cli> for WebSearchParams {
    fn from(cli: Cli) -> Self {
        Self {
            q: cli.q,
            country: cli.country,
            search_lang: cli.search_lang,
            ui_lang: cli.ui_lang,
            count: cli.count,
            offset: cli.offset,
            safesearch: cli.safesearch,
            freshness: cli.freshness,
            text_decorations: if cli.text_decorations { Some("1".to_string()) } else { None },
            spellcheck: if cli.spellcheck { Some("1".to_string()) } else { None },
            result_filter: cli.result_filter,
            goggles_id: cli.goggles_id,
            units: cli.units,
            extra_snippets: if cli.extra_snippets { Some("1".to_string()) } else { None },
            summary: Some("1".to_string()),
        }
    }
}

pub fn run(cli: Cli, subscription_token: &str) -> Result<()> {
    let client =
        Brave::new(brave::Auth::new(subscription_token), "https://api.search.brave.com/res/v1");

    let params: WebSearchParams = cli.into();

    let response = client.summarize(&params)?;
    println!("{}", serde_json::to_string_pretty(&response)?);

    Ok(())
}
