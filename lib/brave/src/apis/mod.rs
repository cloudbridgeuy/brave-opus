use serde::{self, Deserialize, Serialize};

pub mod web_search;

/// Web Search API query parameters
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WebSearchParams {
    /// The user's search query term.
    ///
    /// Query can not be empty. Maximum of 400 characters and 50 words in the query.
    pub q: String,

    /// The search query country, where the results come from.
    ///
    /// The country string is limited to 2 character country codes of supported countries. For
    /// a list of supported values, see Country Codes.
    pub country: Option<String>,

    /// The search language preference.
    ///
    /// The 2 or more character language code for which the search results are provided. For a
    /// list of possible values, see Language Codes.
    pub search_lang: Option<String>,

    /// User interface language preferred in response.
    ///
    /// Usually of the format `<language_code>-<country_code>`, see RFC 9110. For a list of supported values, see
    /// UI Language Codes.
    pub ui_lang: Option<String>,

    /// The number of search results returned in response.
    ///
    /// The maximum is `20`. The actual number delivered may be less than
    /// requested. Combine this parameter with `offset`
    /// to paginate search results.
    pub count: Option<u32>,

    /// The zero based offset that indicates number of search results per page (count) to skip
    /// before returning the result. The maximum is `9`. The actual number
    /// delivered may be less than requested based on the query.
    ///
    /// In order to paginate results use this parameter together with `count`. For
    /// example, if your user interface displays 20 search results per page, set
    /// `count` to `20` and `offset` to `0`
    /// to show the first page of results. To get subsequent pages, increment
    /// `offset` by 1 (e.g. 0, 1, 2). The results may overlap across multiple pages.
    pub offset: Option<u32>,

    /// Filters search results for adult content.
    ///
    /// The following values are supported:
    ///
    /// - off: Adult content is included where relevant.
    /// - moderate: Adult text but no adult images or videos.
    /// - strict: No adult content with adult text, images, and videos.
    pub safesearch: Option<String>,

    /// Filters search results by when they were discovered.
    ///
    /// The following time deltas are supported:
    ///
    /// - pd: Discovered within the last 24 hours.
    /// - pw: Discovered within the last 7 Days.
    /// - pm: Discovered within the last 31 Days.
    /// - py: Discovered within the last 365 Days.
    /// - YYYY-MM-DDtoYYYY-MM-DD: A timeframe is also supported by specifying the date range e.g.
    ///   `2022-04-01to2022-07-30`.
    pub freshness: Option<String>,

    /// Whether display strings (e.g. result snippets) should include decoration markers (e.g.
    /// highlighting characters).
    pub text_decorations: Option<bool>,

    /// Whether to spellcheck provided query. If the spellchecker is enabled, the modified query
    /// is always used for search. The modified query can be found in `altered` key
    /// from the query response model.
    pub spellcheck: Option<bool>,

    /// A comma delimited string of result types to include in the search response.
    ///
    /// Not specifying this parameter will return back all result types in search response
    /// where data is available and a plan with the corresponding option is subscribed. The
    /// response always includes `query` and `type` to identify any query
    /// modifications and response type respectively.
    ///
    /// Available result filter values are:
    ///
    /// - discussions
    /// - faq
    /// - infobox
    /// - news
    /// - query
    /// - summarizer
    /// - videos
    /// - web
    ///
    /// Example result filter value of `discussions,videos` returns only
    /// `query`, `discussions`, and `videos` responses.
    pub result_filter: Option<String>,

    /// Goggles act as a custom re-ranking on top of Brave's search index. For more details,
    /// refer to the Goggles repository.
    pub goggles_id: Option<String>,

    /// The measurement units.
    ///
    /// If not provided, units are derived from search country. Possible values are:
    ///
    /// - metric: The standardized measurement system.
    /// - imperial: The British Imperial system of units.
    pub units: Option<String>,

    /// A snippet is an excerpt from a page you get as a result of the query, and `extra_snippets`
    /// allow you to get up to 5 additional, alternative excerpts. Only available under
    /// `Free AI`, `Base AI`, `Pro AI`, `Base Data`, `Pro Data` and `Custom` plans.
    pub extra_snippets: Option<bool>,
}

impl WebSearchParams {
    pub fn new(q: &str) -> Self {
        Self { q: q.to_string(), ..Default::default() }
    }

    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        // `q` is mandatory, so we add it directly.
        params.push(("q".to_string(), self.q.clone()));

        if let Some(ref country) = self.country {
            params.push(("country".to_string(), country.clone()));
        }
        if let Some(ref search_lang) = self.search_lang {
            params.push(("search_lang".to_string(), search_lang.clone()));
        }
        if let Some(ref ui_lang) = self.ui_lang {
            params.push(("ui_lang".to_string(), ui_lang.clone()));
        }
        if let Some(count) = self.count {
            params.push(("count".to_string(), count.to_string()));
        }
        if let Some(offset) = self.offset {
            params.push(("offset".to_string(), offset.to_string()));
        }
        if let Some(ref safesearch) = self.safesearch {
            params.push(("safesearch".to_string(), safesearch.clone()));
        }
        if let Some(ref freshness) = self.freshness {
            params.push(("freshness".to_string(), freshness.clone()));
        }
        if let Some(text_decorations) = self.text_decorations {
            params.push(("text_decorations".to_string(), text_decorations.to_string()));
        }
        if let Some(spellcheck) = self.spellcheck {
            params.push(("spellcheck".to_string(), spellcheck.to_string()));
        }
        if let Some(ref result_filter) = self.result_filter {
            params.push(("result_filter".to_string(), result_filter.clone()));
        }
        if let Some(ref goggles_id) = self.goggles_id {
            params.push(("goggles_id".to_string(), goggles_id.clone()));
        }
        if let Some(ref units) = self.units {
            params.push(("units".to_string(), units.clone()));
        }
        if let Some(extra_snippets) = self.extra_snippets {
            params.push(("extra_snippets".to_string(), extra_snippets.to_string()));
        }

        params
    }
}

/// Top level response model for successful Web Search API requests. The response will include the
/// relevant keys based on the plan subscribed, query relevance or applied result_filter as a query
/// parameter. The API can also respond back with an error response based on invalid subscription
/// keys and rate limit events.
#[derive(Debug, Serialize, Deserialize)]
pub struct WebSearchApiResponse {
    /// The type of web search api result. The value is always search.
    pub r#type: Option<String>,
    /// Discussions clusters aggregated from forum posts that are relevant to the query.
    pub discussions: Option<Discussions>,
    /// Frequently asked questions that are relevant to the search query.
    pub faq: Option<FAQ>,
    /// Aggregated information on an entity showable as an infobox.
    pub infobox: Option<GraphInfobox>,
    /// Places of interest (POIs) relevant to location sensitive queries.
    pub locations: Option<Locations>,
    /// Preferred ranked order of search results.
    pub mixed: Option<MixedResponse>,
    /// News results relevant to the query.
    pub news: Option<News>,
    /// Search query string and its modifications that are used for search.
    pub query: Option<Query>,
    /// Videos relevant to the query.
    pub videos: Option<Videos>,
    /// Web search results relevant to the query.
    pub web: Option<Search>,
}

/// Top level response model for successful Summarizer Search API requests. The response will
/// include the summarized content or answer based on the key. The API can also respond back with an
/// error response based on the incomplete summarization request, invalid subscription keys, and
/// rate limit events. Access to Summarizer requires a subscription to Pro AI plan.
#[derive(Debug, Serialize, Deserialize)]
pub struct SummarizerSearchApiResponse {
    /// The type of summarizer search api result. The value is always summarizer.
    pub r#type: Option<String>,
    /// The current status of summarizer for the given key. The value can be either failed or complete.
    pub status: Option<String>,
    /// Summarizer search results relevant to the query key.
    pub results: Option<Vec<SummarizerResult>>,
}

/// A model representing information gathered around the requested query.
#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    /// The original query that was requested.
    pub original: Option<String>,
    /// Whether there is more content available for query, but the response was restricted due to safesearch.
    pub show_strict_warning: Option<bool>,
    /// The altered query for which the search was performed.
    pub altered: Option<String>,
    /// Whether safesearch was enabled.
    pub safesearch: Option<bool>,
    /// Whether the query is a navigational query to a domain.
    pub is_navigational: Option<bool>,
    /// Whether the query has location relevance.
    pub is_geolocal: Option<bool>,
    /// Whether the query was decided to be location sensitive.
    pub local_decision: Option<String>,
    /// The index of the location.
    pub local_locations_idx: Option<i32>,
    /// Whether the query is trending.
    pub is_trending: Option<bool>,
    /// Whether the query has news breaking articles relevant to it.
    pub is_news_breaking: Option<bool>,
    /// Whether the query requires location information for better results.
    pub ask_for_location: Option<bool>,
    /// The language information gathered from the query.
    pub language: Option<Language>,
    /// Whether the spellchecker was off.
    pub spellcheck_off: Option<bool>,
    /// The country that was used.
    pub country: Option<String>,
    /// Whether there are bad results for the query.
    pub bad_results: Option<bool>,
    /// Whether the query should use a fallback.
    pub should_fallback: Option<bool>,
    /// The gathered location latitude associated with the query.
    pub lat: Option<String>,
    /// The gathered location longitude associated with the query.
    pub long: Option<String>,
    /// The gathered postal code associated with the query.
    pub postal_code: Option<String>,
    /// The gathered city associated with the query.
    pub city: Option<String>,
    /// The gathered state associated with the query.
    pub state: Option<String>,
    /// The country for the request origination.
    pub header_country: Option<String>,
    /// Whether more results are available for the given query.
    pub more_results_available: Option<bool>,
    /// Any custom location labels attached to the query.
    pub custom_location_label: Option<String>,
    /// Any reddit cluster associated with the query.
    pub reddit_cluster: Option<String>,
}

/// A model representing a discussion cluster relevant to the query.
#[derive(Debug, Serialize, Deserialize)]
pub struct Discussions {
    /// The type identifying a discussion cluster. Currently the value is always search.
    pub r#type: Option<String>,
    /// A list of discussion results.
    pub results: Option<Vec<DiscussionResult>>,
    /// Whether the discussion results are changed by a Goggle. False by default.
    pub mutated_by_goggles: Option<bool>,
}

/// A discussion result. These are forum posts and discussions that are relevant to the search query.
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscussionResult {
    /// The discussion result type identifier. The value is always discussion.
    pub r#type: Option<String>,
    /// The enriched aggregated data for the relevant forum post.
    pub data: Option<ForumData>,
}

/// Defines a result from a discussion forum.
#[derive(Debug, Serialize, Deserialize)]
pub struct ForumData {
    /// The name of the forum.
    pub forum_name: Option<String>,
    /// The number of answers to the post.
    pub num_answers: Option<i32>,
    /// The score of the post on the forum.
    pub score: Option<String>,
    /// The title of the post on the forum.
    pub title: Option<String>,
    /// The question asked in the forum post.
    pub question: Option<String>,
    /// The top-rated comment under the forum post.
    pub top_comment: Option<String>,
}

/// Frequently asked questions relevant to the search query term.
#[derive(Debug, Serialize, Deserialize)]
pub struct FAQ {
    /// The FAQ result type identifier. The value is always faq.
    pub r#type: Option<String>,
    /// A list of aggregated question answer results relevant to the query.
    pub results: Option<Vec<QA>>,
}

/// A question answer result.
#[derive(Debug, Serialize, Deserialize)]
pub struct QA {
    /// The question being asked.
    pub question: Option<String>,
    /// The answer to the question.
    pub answer: Option<String>,
    /// The title of the post.
    pub title: Option<String>,
    /// The url pointing to the post.
    pub url: Option<String>,
    /// Aggregated information about the url.
    pub meta_url: Option<MetaUrl>,
}

/// Aggregated information about a url.
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaUrl {
    /// The protocol scheme extracted from the url.
    pub scheme: Option<String>,
    /// The network location part extracted from the url.
    pub netloc: Option<String>,
    /// The lowercased domain name extracted from the url.
    pub hostname: Option<String>,
    /// The favicon used for the url.
    pub favicon: Option<String>,
    /// The hierarchical path of the url useful as a display string.
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
/// A model representing a collection of web search results.
pub struct Search {
    /// A type identifying web search results. The value is always search.
    pub r#type: Option<String>,
    /// A list of search results.
    pub results: Option<Vec<SearchResult>>,
    /// Whether the results are family friendly.
    pub family_friendly: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Aggregated information on a web search result, relevant to the query.
pub struct SearchResult {
    /// A type identifying a web search result. The value is always search_result.
    pub r#type: Option<String>,
    /// A sub type identifying the web search result type.
    pub subtype: Option<String>,
    /// Gathered information on a web search result.
    pub deep_results: Option<DeepResult>,
    /// A list of schemas extracted from the web search result.
    pub schemas: Option<Vec<Vec<String>>>,
    /// Aggregated information on the url associated with the web search result.
    pub meta_url: Option<MetaUrl>,
    /// The thumbnail of the web search result.
    pub thumbnail: Option<Thumbnail>,
    /// A string representing the age of the web search result.
    pub age: Option<String>,
    /// The main language on the web search result.
    pub language: Option<String>,
    /// The location details if the query relates to a restaurant.
    pub location: Option<LocationResult>,
    /// The video associated with the web search result.
    pub video: Option<VideoData>,
    /// The movie associated with the web search result.
    pub movie: Option<MovieData>,
    /// Any frequently asked questions associated with the web search result.
    pub faq: Option<FAQ>,
    /// Any question answer information associated with the web search result page.
    pub qa: Option<QAPage>,
    /// Any book information associated with the web search result page.
    pub book: Option<Book>,
    /// Rating found for the web search result page.
    pub rating: Option<Rating>,
    /// An article found for the web search result page.
    pub article: Option<Article>,
    /// The main product and a review that is found on the web search result page.
    pub product: Option<ProductReview>,
    /// A list of products and reviews that are found on the web search result page.
    pub product_cluster: Option<Vec<ProductReview>>,
    /// A type representing a cluster. The value can be product_cluster.
    pub cluster_type: Option<String>,
    /// A list of web search results.
    pub cluster: Option<Vec<Result>>,
    /// Aggregated information on the creative work found on the web search result.
    pub creative_work: Option<CreativeWork>,
    /// Aggregated information on music recording found on the web search result.
    pub music_recording: Option<MusicRecording>,
    /// Aggregated information on the review found on the web search result.
    pub review: Option<Review>,
    /// Aggregated information on a software product found on the web search result page.
    pub software: Option<Software>,
    /// Aggregated information on a recipe found on the web search result page.
    pub recipe: Option<Recipe>,
    /// Aggregated information on an organization found on the web search result page.
    pub organization: Option<Organization>,
    /// The content type associated with the search result page.
    pub content_type: Option<String>,
    /// A list of extra alternate snippets for the web search result.
    pub extra_snippets: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
/// A model representing a web search result.
pub struct Result {
    /// The title of the web page.
    pub title: Option<String>,
    /// The url where the page is served.
    pub url: Option<String>,
    pub is_source_local: Option<bool>,
    pub is_source_both: Option<bool>,
    /// A description for the web page.
    pub description: Option<String>,
    /// A date representing the age of the web page.
    pub page_age: Option<String>,
    /// A date representing when the web page was last fetched.
    pub page_fetched: Option<String>,
    /// A profile associated with the web page.
    pub profile: Option<Profile>,
    /// A language classification for the web page.
    pub language: Option<String>,
    /// Whether the web page is family friendly.
    pub family_friendly: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Aggregated information on an entity from a knowledge graph.
pub struct GraphInfobox {
    /// The infobox result type identifier. The value is always infobox.
    pub r#type: Option<String>,
    /// The position on a search result page.
    pub position: Option<i32>,
    /// Any label associated with the entity.
    pub label: Option<String>,
    /// Category classification for the entity.
    pub category: Option<String>,
    /// A longer description for the entity.
    pub long_desc: Option<String>,
    /// The thumbnail associated with the entity.
    pub thumbnail: Option<Thumbnail>,
    /// A list of attributes about the entity.
    pub attributes: Option<Vec<Vec<String>>>,
    /// The profiles associated with the entity.
    pub profiles: Option<Vec<Profile>>,
    /// The official website pertaining to the entity.
    pub website_url: Option<String>,
    /// The number of attributes to be shown about the entity.
    pub attributes_shown: Option<i32>,
    /// Any ratings given to the entity.
    pub ratings: Option<Vec<Rating>>,
    /// A list of data sources for the entity.
    pub providers: Option<Vec<DataProvider>>,
    /// A unit representing quantity relevant to the entity.
    pub distance: Option<Unit>,
    /// A list of images relevant to the entity.
    pub images: Option<Vec<Thumbnail>>,
    /// Any movie data relevant to the entity. Appears only when the result is a movie.
    pub movie: Option<MovieData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenericInfobox {
    pub subtype: Option<String>,
    pub found_in_urls: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QAInfobox {
    pub subtype: Option<String>,
    pub data: Option<QAPage>,
    pub meta_url: Option<MetaUrl>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoboxWithLocation {
    pub subtype: Option<String>,
    pub is_location: Option<bool>,
    pub coordinates: Option<Vec<f32>>,
    pub zoom_level: Option<i32>,
    pub location: Option<LocationResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoboxPlace {
    pub subtype: Option<String>,
}

/// Aggregated result from a question answer page.
#[derive(Debug, Serialize, Deserialize)]
pub struct QAPage {
    /// The question that is being asked.
    pub question: Option<String>,
    /// An answer to the question.
    pub answer: Option<Answer>,
}

/// A response representing an answer to a question on a forum.
#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    /// The main content of the answer.
    pub text: Option<String>,
    /// The name of the author of the answer.
    pub author: Option<String>,
    /// Number of upvotes on the answer.
    pub upvote_count: Option<i32>,
    /// The number of downvotes on the answer.
    pub downvote_count: Option<i32>,
}

/// Aggregated details representing a picture thumbnail.
#[derive(Debug, Serialize, Deserialize)]
pub struct Thumbnail {
    /// The served url of the picture thumbnail.
    pub src: Option<String>,
    /// The height of the image.
    pub height: Option<i32>,
    /// The width of the image.
    pub width: Option<i32>,
    /// The background color of the image.
    pub bg_color: Option<String>,
    /// The original url of the image.
    pub original: Option<String>,
    /// Whether the image is a logo.
    pub logo: Option<bool>,
    /// Whether the image is duplicated.
    pub duplicated: Option<bool>,
    /// The theme associated with the image.
    pub theme: Option<String>,
}

/// A result that is location relevant.
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationResult {
    /// Location result type identifier. The value is always location_result.
    pub r#type: Option<String>,
    /// The complete url of the provider.
    pub provider_url: Option<String>,
    /// A list of coordinates associated with the location. This is a lat long represented as a floating point.
    pub coordinates: Option<Vec<f32>>,
    /// The zoom level on the map.
    pub zoom_level: Option<i32>,
    /// The thumbnail associated with the location.
    pub thumbnail: Option<Thumbnail>,
    /// The postal address associated with the location.
    pub postal_address: Option<PostalAddress>,
    /// The opening hours, if it is a business, associated with the location.
    pub opening_hours: OpeningHours,
    /// The contact of the business associated with the location.
    pub contact: Option<Contact>,
    /// A display string used to show the price classification for the business.
    pub price_range: Option<String>,
    /// The ratings of the business.
    pub rating: Option<Rating>,
    /// The distance of the location from the client.
    pub distance: Option<Unit>,
    /// Profiles associated with the business.
    pub profiles: Option<Vec<DataProvider>>,
    /// Aggregated reviews from various sources relevant to the business.
    pub reviews: Option<Reviews>,
    /// A bunch of pictures associated with the business.
    pub pictures: Option<PictureResults>,
}

/// A model representing location results.
#[derive(Debug, Serialize, Deserialize)]
pub struct Locations {
    /// Location type identifier. The value is always locations.
    pub r#type: Option<String>,
    /// An aggregated list of location sensitive results.
    pub results: Option<Vec<LocationResult>>,
}

/// The ranking order of results on a search result page.
#[derive(Debug, Serialize, Deserialize)]
pub struct MixedResponse {
    /// The type representing the model mixed. The value is always mixed.
    pub r#type: Option<String>,
    /// The ranking order for the main section of the search result page.
    pub main: Option<Vec<ResultReference>>,
    /// The ranking order for the top section of the search result page.
    pub top: Option<Vec<ResultReference>>,
    /// The ranking order for the side section of the search result page.
    pub side: Option<Vec<ResultReference>>,
}

/// The ranking order of results on a search result page.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResultReference {
    /// The type of the result.
    pub r#type: Option<String>,
    /// The 0th based index where the result should be placed.
    pub index: Option<i32>,
    /// Whether to put all the results from the type at specific position.
    pub all: Option<bool>,
}

/// A model representing video results.
#[derive(Debug, Serialize, Deserialize)]
pub struct Videos {
    /// The type representing the videos. The value is always videos.
    pub r#type: Option<String>,
    /// A list of video results.
    pub results: Option<Vec<VideoResult>>,
    /// Whether the video results are changed by a Goggle. False by default.
    pub mutated_by_goggles: Option<bool>,
}

/// A model representing news results.
#[derive(Debug, Serialize, Deserialize)]
pub struct News {
    /// The type representing the news. The value is always news.
    pub r#type: Option<String>,
    /// A list of news results.
    pub results: Option<Vec<NewsResult>>,
    /// Whether the news results are changed by a Goggle. False by default.
    pub mutated_by_goggles: Option<bool>,
}

/// A model representing news results.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewsResult {
    /// The aggregated information on the url representing a news result
    pub meta_url: Option<MetaUrl>,
    /// The source of the news.
    pub source: Option<String>,
    /// Whether the news result is currently a breaking news.
    pub breaking: Option<bool>,
    /// The thumbnail associated with the news result.
    pub thumbnail: Option<Thumbnail>,
    /// A string representing the age of the news article.
    pub age: Option<String>,
    /// A list of extra alternate snippets for the news search result.
    pub extra_snippets: Option<Vec<String>>,
}

/// A model representing a list of pictures.
#[derive(Debug, Serialize, Deserialize)]
pub struct PictureResults {
    /// A url to view more pictures.
    #[serde(rename = "viewMoreUrl")]
    pub view_more_url: Option<String>,
    /// A list of thumbnail results.
    pub results: Option<Vec<Thumbnail>>,
}

/// A model representing an action to be taken.
#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    /// The type representing the action.
    pub r#type: Option<String>,
    /// A url representing the action to be taken.
    pub url: Option<String>,
}

/// A model representing a postal address of a location
#[derive(Debug, Serialize, Deserialize)]
pub struct PostalAddress {
    /// The type identifying a postal address. The value is always PostalAddress.
    pub r#type: Option<String>,
    /// The country associated with the location.
    pub country: Option<String>,
    /// The postal code associated with the location.
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    /// The street address associated with the location.
    #[serde(rename = "streetAddress")]
    pub street_address: Option<String>,
    /// The region associated with the location. This is usually a state.
    #[serde(rename = "addressRegion")]
    pub address_region: Option<String>,
    /// The address locality or subregion associated with the location.
    #[serde(rename = "addressLocality")]
    pub address_locality: Option<String>,
    /// The displayed address string.
    #[serde(rename = "displayAddress")]
    pub display_address: Option<String>,
}

/// Opening hours of a business at a particular location.
#[derive(Debug, Serialize, Deserialize)]
pub struct OpeningHours {
    /// The current day opening hours. Can have two sets of opening hours.
    pub current_day: Option<Vec<DayOpeningHours>>,
    /// The opening hours for the whole week.
    pub days: Option<Vec<Vec<DayOpeningHours>>>,
}

/// A model representing the opening hours for a particular day for a business at a particular location.
#[derive(Debug, Serialize, Deserialize)]
pub struct DayOpeningHours {
    /// A short string representing the day of the week.
    pub abbr_name: Option<String>,
    /// A full string representing the day of the week.
    pub full_name: Option<String>,
    /// A 24 hr clock time string for the opening time of the business on a particular day.
    pub opens: Option<String>,
    /// A 24 hr clock time string for the closing time of the business on a particular day.
    pub closes: Option<String>,
}

/// A model representing contact information for an entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    /// The email address.
    pub email: Option<String>,
    /// The telephone number.
    pub telephone: Option<String>,
}

/// A model representing the data provider associated with the entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct DataProvider {
    /// The type representing the source of data. This is usually external.
    pub r#type: Option<String>,
    /// The name of the data provider. This can be a domain.
    pub name: Option<String>,
    /// The url where the information is coming from.
    pub url: Option<String>,
    /// The long name for the data provider.
    pub long_name: Option<String>,
    /// The served url for the image data.
    pub img: Option<String>,
}

/// A profile of an entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    /// The name of the profile.
    pub name: Option<String>,
    /// The long name of the profile.
    pub long_name: Option<String>,
    /// The original url where the profile is available.
    pub url: Option<String>,
    /// The served image url representing the profile.
    pub img: Option<String>,
}

/// A model representing a unit of measurement.
#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    /// The quantity of the unit.
    pub value: Option<f32>,
    /// The name of the unit associated with the quantity.
    pub units: Option<String>,
}

/// Aggregated data for a movie result.
#[derive(Debug, Serialize, Deserialize)]
pub struct MovieData {
    /// Name of the movie.
    pub name: Option<String>,
    /// A short plot summary for the movie.
    pub description: Option<String>,
    /// A url serving a movie profile page.
    pub url: Option<String>,
    /// A thumbnail for a movie poster.
    pub thumbnail: Option<Thumbnail>,
    /// The release date for the movie.
    pub release: Option<String>,
    /// A list of people responsible for directing the movie.
    pub directors: Option<Vec<Person>>,
    /// A list of actors in the movie.
    pub actors: Option<Vec<Person>>,
    /// Rating provided to the movie from various sources.
    pub rating: Option<Rating>,
    /// The runtime of the movie. The format is HH:MM:SS.
    pub duration: Option<String>,
    /// List of genres in which the movie can be classified.
    pub genre: Option<Vec<String>>,
    /// The query that resulted in the movie result.
    pub query: Option<String>,
}

/// A model describing a generic thing.
#[derive(Debug, Serialize, Deserialize)]
pub struct Thing {
    /// A type identifying a thing. The value is always thing.
    pub r#type: Option<String>,
    /// The name of the thing.
    pub name: Option<String>,
    /// A url for the thing.
    pub url: Option<String>,
    /// Thumbnail associated with the thing.
    pub thumbnail: Option<Thumbnail>,
}

/// A model describing a person entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    /// A type identifying a person. The value is always person.
    pub r#type: Option<String>,
}

/// The rating associated with an entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    /// The current value of the rating.
    pub rating_value: Option<f32>,
    /// Best rating received.
    pub best_rating: Option<f32>,
    /// The number of reviews associated with the rating.
    pub review_count: Option<i32>,
    /// The profile associated with the rating.
    pub profile: Option<Profile>,
    /// Whether the rating is coming from Tripadvisor.
    pub is_tripadvisor: Option<bool>,
}

/// A model representing a book result.
#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    /// The title of the book.
    pub title: Option<String>,
    /// The author of the book.
    pub author: Option<Vec<Person>>,
    /// The publishing date of the book.
    pub date: Option<String>,
    /// The price of the book.
    pub price: Option<Price>,
    /// The number of pages in the book.
    pub pages: Option<i32>,
    /// The publisher of the book.
    pub publisher: Option<Person>,
    /// A gathered rating from different sources associated with the book.
    pub rating: Option<Rating>,
}

/// A model representing the price for an entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    /// The price value in a given currency.
    pub price: Option<String>,
    /// The currency of the price value.
    pub price_currency: Option<String>,
}

/// A model representing an article.
#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    /// The author of the article.
    pub author: Option<Vec<Person>>,
    /// The date when the article was published.
    pub date: Option<String>,
    /// The name of the publisher for the article.
    pub publisher: Option<Organization>,
    /// A thumbnail associated with the article.
    pub thumbnail: Option<Thumbnail>,
    /// Whether the article is free to read or is behind a paywall.
    pub is_accessible_for_free: Option<bool>,
}

/// A way to contact an entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct ContactPoint {
    /// A type string identifying a contact point. The value is always contact_point.
    pub r#type: Option<String>,
    /// The telephone number of the entity.
    pub telephone: Option<String>,
    /// The email address of the entity.
    pub email: Option<String>,
}

/// An entity responsible for another entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    /// A type string identifying an organization. The value is always organization.
    pub r#type: Option<String>,
    /// A list of contact points for the organization.
    pub contact_points: Option<Vec<ContactPoint>>,
}

/// Aggregated information on a how to.
#[derive(Debug, Serialize, Deserialize)]
pub struct HowTo {
    /// The how to text.
    pub text: Option<String>,
    /// A name for the how to.
    pub name: Option<String>,
    /// A url associated with the how to.
    pub url: Option<String>,
    /// A list of image urls associated with the how to.
    pub image: Option<Vec<String>>,
}

/// Aggregated information on a recipe.
#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    /// The title of the recipe.
    pub title: Option<String>,
    /// The description of the recipe.
    pub description: Option<String>,
    /// A thumbnail associated with the recipe.
    pub thumbnail: Option<Thumbnail>,
    /// The url of the web page where the recipe was found.
    pub url: Option<String>,
    /// The domain of the web page where the recipe was found.
    pub domain: Option<String>,
    /// The url for the favicon of the web page where the recipe was found.
    pub favicon: Option<String>,
    /// The total time required to cook the recipe.
    pub time: Option<String>,
    /// The preparation time for the recipe.
    pub prep_time: Option<String>,
    /// The cooking time for the recipe.
    pub cook_time: Option<String>,
    /// Ingredients required for the recipe.
    pub ingredients: Option<String>,
    /// List of instructions for the recipe.
    pub instructions: Option<Vec<String>>,
    /// How many people the recipe serves.
    pub servings: Option<i32>,
    /// Calorie count for the recipe.
    pub calories: Option<i32>,
    /// Aggregated information on the ratings associated with the recipe.
    pub rating: Option<Rating>,
    /// The category of the recipe.
    pub recipe_category: Option<String>,
    /// The cuisine classification for the recipe.
    pub recipe_cuisine: Option<String>,
    /// Aggregated information on the cooking video associated with the recipe.
    pub video: Option<VideoData>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")] // This tells serde to use the `type` field as a discriminator
pub enum ProductReview {
    #[serde(rename = "Product")]
    Product(Product),
    #[serde(rename = "Review")]
    Review(Review),
}

/// A model representing a product.
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    /// The name of the product.
    pub name: Option<String>,
    /// The price of the product.
    pub price: Option<String>,
    /// A thumbnail associated with the product.
    pub thumbnail: Option<Thumbnail>,
    /// The description of the product.
    pub description: Option<String>,
    /// A list of offers available on the product.
    pub offers: Option<Vec<Offer>>,
    /// A rating associated with the product.
    pub rating: Option<Rating>,
}

/// A model representing a review for an entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct Review {
    /// The review title for the review.
    pub name: Option<String>,
    /// The thumbnail associated with the reviewer.
    pub thumbnail: Option<Thumbnail>,
    /// A description of the review (the text of the review itself).
    pub description: Option<String>,
    /// The ratings associated with the review.
    pub rating: Option<Rating>,
}

/// An offer associated with a product.
#[derive(Debug, Serialize, Deserialize)]
pub struct Offer {
    /// The url where the offer can be found.
    pub url: Option<String>,
    /// The currency in which the offer is made.
    pub price_currency: Option<String>,
    /// The price of the product currently on offer.
    pub price: Option<String>,
}

/// The reviews associated with an entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct Reviews {
    /// A list of trip advisor reviews for the entity.
    pub results: Option<Vec<TripAdvisorReview>>,
    /// A url to a web page where more information on the result can be seen.
    pub view_more_url: Option<String>,
    /// Any reviews available in a foreign language.
    pub reviews_in_foreign_language: Option<bool>,
}

/// A model representing a Tripadvisor review.
#[derive(Debug, Serialize, Deserialize)]
pub struct TripAdvisorReview {
    /// The title of the review.
    pub title: Option<String>,
    /// A description seen in the review.
    pub description: Option<String>,
    /// The date when the review was published.
    pub date: Option<String>,
    /// A rating given by the reviewer.
    pub rating: Option<Rating>,
    /// The author of the review.
    pub author: Option<Person>,
    /// A url link to the page where the review can be found.
    pub review_url: Option<String>,
    /// The language of the review.
    pub language: Option<String>,
}

/// A creative work relevant to the query. An example can be enriched metadata for an app.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreativeWork {
    /// The name of the creative work.
    pub name: Option<String>,
    /// A thumbnail associated with the creative work.
    pub thumbnail: Option<Thumbnail>,
    /// A rating that is given to the creative work.
    pub rating: Option<Rating>,
}

/// Result classified as a music label or a song.
#[derive(Debug, Serialize, Deserialize)]
pub struct MusicRecording {
    /// The name of the song or album.
    pub name: Option<String>,
    /// A thumbnail associated with the music.
    pub thumbnail: Option<Thumbnail>,
    /// The rating of the music.
    pub rating: Option<Rating>,
}

/// A model representing a software entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct Software {
    /// The name of the software product.
    pub name: Option<String>,
    /// The author of software product.
    pub author: Option<String>,
    /// The latest version of the software product.
    pub version: Option<String>,
    /// The code repository where the software product is currently available or maintained.
    pub code_repository: Option<String>,
    /// The home page of the software product.
    pub homepage: Option<String>,
    /// The date when the software product was published.
    pub date_published: Option<String>,
    /// Whether the software product is available on npm.
    pub is_npm: Option<bool>,
    /// Whether the software product is available on pypi.
    pub is_pypi: Option<bool>,
    /// The number of stars on the repository.
    pub stars: Option<i32>,
    /// The numbers of forks of the repository.
    pub forks: Option<i32>,
    /// The programming language spread on the software product.
    pub programming_language: Option<String>,
}

/// Aggregated deep results from news, social, videos and images.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeepResult {
    /// A list of news results associated with the result.
    pub news: Option<Vec<NewsResult>>,
    /// A list of buttoned results associated with the result.
    pub buttons: Option<Vec<ButtonResult>>,
    /// Social profile associated with the result.
    pub social: Option<Vec<Profile>>,
    /// Videos associated with the result.
    pub videos: Option<Vec<VideoResult>>,
    /// Images associated with the result.
    pub images: Option<Vec<Image>>,
}

/// A model representing a video result.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoResult {
    /// The type identifying the video result. The value is always video_result.
    pub r#type: Option<String>,
    /// Meta data for the video.
    pub video: Option<VideoData>,
    /// Aggregated information on the URL
    pub meta_url: Option<MetaUrl>,
    /// The thumbnail of the video.
    pub thumbnail: Option<Thumbnail>,
    /// A string representing the age of the video.
    pub age: Option<String>,
}

/// A model representing metadata gathered for a video.
#[derive(Debug, Serialize, Deserialize)]
pub struct VideoData {
    /// A time string representing the duration of the video. The format can be HH:MM:SS or MM:SS.
    pub duration: Option<String>,
    /// The number of views of the video.
    pub views: Option<String>,
    /// The creator of the video.
    pub creator: Option<String>,
    /// The publisher of the video.
    pub publisher: Option<String>,
    /// A thumbnail associated with the video.
    pub thumbnail: Option<Thumbnail>,
}

/// A result which can be used as a button.
#[derive(Debug, Serialize, Deserialize)]
pub struct ButtonResult {
    /// A type identifying button result. The value is always button_result.
    pub r#type: Option<String>,
    /// The title of the result.
    pub title: Option<String>,
    /// The url for the button result.
    pub url: Option<String>,
}

/// Represents a knowledge graph entity.
#[derive(Debug, Serialize, Deserialize)]
pub struct KnowledgeGraphEntity {
    /// A short title describing the entity.
    pub title: Option<String>,
    /// A description of the entity.
    pub description: Option<String>,
    /// The url representing the entity.
    pub url: Option<URL>,
    /// The thumbnail associated with the entity.
    pub thumbnail: Option<URL>,
}

/// Represents an entity profile from a knowledge graph.
#[derive(Debug, Serialize, Deserialize)]
pub struct KnowledgeGraphProfile {
    /// The url representing the profile.
    pub url: Option<URL>,
    /// A description of the profile.
    pub description: Option<String>,
}

/// A model representing a URL.
#[derive(Debug, Serialize, Deserialize)]
pub struct URL {
    /// The original source URL.
    pub original: Option<String>,
    /// The display URL.
    pub display: Option<String>,
    /// An alternative representation of a URL.
    pub alternatives: Option<Vec<String>>,
    /// The canonical form of the URL.
    pub canonical: Option<String>,
    /// A mobile friendly version of the URL.
    pub mobile: Option<MobileUrlItem>,
}

/// A mobile friendly representation of the URL.
#[derive(Debug, Serialize, Deserialize)]
pub struct MobileUrlItem {
    /// The original source URL.
    pub original: Option<String>,
    /// The amp version of the URL.
    pub amp: Option<String>,
    /// An android friendly version of the URL.
    pub android: Option<String>,
    /// An ios friendly version of the URL.
    pub ios: Option<String>,
}

/// A model describing an image
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    /// The thumbnail associated with the image.
    pub thumbnail: Option<Thumbnail>,
    /// The url of the image.
    pub url: Option<String>,
    /// Metadata on the image.
    pub properties: Option<ImageProperties>,
}

/// A model representing a language.
#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    /// The main language seen in the string.
    pub main: Option<String>,
}

/// Metadata on an image.
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageProperties {
    /// The image URL.
    pub url: Option<String>,
    /// The resized image.
    pub resized: Option<String>,
    /// The height of the image.
    pub height: Option<i32>,
    /// The width of the image.
    pub width: Option<i32>,
    /// The format specifier for the image.
    pub format: Option<String>,
    /// The image storage size.
    pub content_size: Option<String>,
}

/// A result from the summarizer.
#[derive(Debug, Serialize, Deserialize)]
pub struct SummarizerResult {
    /// The type of summarizer search api result. The value is always summarizer_result.
    pub r#type: Option<String>,
    /// The summarized content and answer relevant to the query.
    pub summary: Option<String>,
    /// The answer in the summary text.
    pub answer: Option<SummarizerAnswer>,
    /// A list of sources, used to create the summary.
    pub references: Option<Vec<ReferenceSource>>,
}

/// An answer from the summarized content.
#[derive(Debug, Serialize, Deserialize)]
pub struct SummarizerAnswer {
    /// The text from the summary, which is the answer to the query.
    pub text: Option<String>,
    /// The location indices of the answer text in the summary.
    pub location: Option<TextLocation>,
}

/// Index based location in a text.
#[derive(Debug, Serialize, Deserialize)]
pub struct TextLocation {
    /// The 0 based index, where the important part of the text starts.
    pub start: Option<String>,
    /// The 0 based index, where the important part of the text ends.
    pub end: Option<String>,
}

/// Index based location in a text for a particular reference.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceSource {
    /// The type of search api result reference source.
    pub r#type: Option<String>,
    /// The domain name of the source.
    pub name: Option<String>,
    /// The url of the source.
    pub url: Option<String>,
    /// The favicon url for the domain.
    pub img: Option<String>,
    /// The location indices of the substring in the source text which is coming from the reference.
    pub locations: Option<Vec<TextLocation>>,
}

// Brave Web Search API
const WEB_SEARCH: &str = "web/search";
