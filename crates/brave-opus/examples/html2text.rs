use anthropic::{
    apis::{
        messages::{MessageApi, MessageBody},
        Message, Role,
    },
    Anthropic,
};
use brave::{
    apis::{web_search::Api, WebSearchParams},
    Brave,
};
use color_eyre::eyre::OptionExt;
use futures::stream::TryStreamExt;
use std::fmt::Debug;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init();

    // Read the first argument from stdin
    let args: Vec<String> = std::env::args().collect();

    let prompt = &args[1];
    let count = &args[2].parse::<u16>()?;

    let semaphore = Arc::new(Semaphore::new(3));
    let brave_semaphore = Arc::new(Semaphore::new(1));

    let key = std::env::var("BRAVE_SUBSCRIPTION_TOKEN")?;

    let brave_client = Brave::new(brave::Auth::new(&key), "https://api.search.brave.com/res/v1");

    let key = std::env::var("ANTHROPIC_API_KEY")?;

    let anthropic_client =
        Anthropic::new(anthropic::Auth::new(&key), "https://api.anthropic.com/v1/");

    let messages = vec![
                Message {
                    role: Role::User,
                    content: format!("
Transform this prompt into three perfect Google search queries to get the information necessary to
answer the user request included on the snippet between triple quotes. The queries should focus on techniques
like using relevant keywords, operators, modifiers, and filters to find the most relevant articles,
documentation, news, and informational sites. The results should cater to experienced users looking
to refine their search skills and find the most useful and reliable information.

RETURN JUST THREE UPDATED PROMPTS SEPARATED BY A SINGLE NEW LINE WITHOUT QUOTES OR ANY ADDITIONAL COMMENTS OR PREAMBLE!!!**
\"\"\"
{}
\"\"\"", prompt)
                }];

    let body = MessageBody::new("claude-3-haiku-20240307", messages, 4096);

    let response = anthropic_client.message_create(&body).unwrap();

    let search_prompts =
        response.content.first().unwrap().text.as_ref().unwrap().clone().replace('"', "");

    println!("# Search Prompts\n\n{}", search_prompts);

    let search_prompts: Vec<String> = search_prompts.split('\n').map(|s| s.to_string()).collect();

    let tasks: Vec<_> = search_prompts
        .into_iter()
        .map(|search_prompt| {
            let sem = brave_semaphore.clone();
            let brave_client = brave_client.clone();
            let count = *count;
            tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                let mut params = WebSearchParams::new(&search_prompt);
                params.count = Some(count);
                params.country = Some("ALL".to_string());

                let mut response: brave::WebSearchApiResponse =
                    brave_client.search(&params).unwrap();

                let web = response.web.take().ok_or_eyre("can't find `web` in response").unwrap();

                web.results.ok_or_eyre("can't find `results` in `web`").unwrap()
            })
        })
        .collect();

    // Assuming `tasks` is Vec<impl Future<Output = Result<Vec<Something>, E>>>
    let nested_results = futures::future::join_all(tasks).await;

    // Flatten Vec<Vec<Something>> into Vec<Something>
    let all_results: Vec<brave::Result> =
        nested_results.into_iter().flat_map(|r| r.unwrap()).collect();

    println!("\n# Context\n");

    let tasks: Vec<_> = all_results.iter().map(|result| {
        let sem = semaphore.clone();
        let agent = ureq::AgentBuilder::new().build();
        let url = result.url.clone().unwrap_or_default();
        let anthropic_client = anthropic_client.clone();
        tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();

            let response = agent.get(&url).call().unwrap();
            let tree = html2text::parse(response.into_reader()).unwrap();
            let text = tree.render(200, Decorator::new()).unwrap().into_string().unwrap();

            let messages = vec![
                Message {
                    role: Role::User,
                    content: format!("
You will receive text extracted from a website using a web crawler. Most HTML formatting will be removed, but some contextual tags may remain. Your task is to process this scraped text and return a plain text output containing only the page's most relevant content.

To do this:
1. Discard any remaining HTML tags and the content inside them completely.
2. Filter out extraneous page elements like:
   - Navigation menus and links
   - Advertisements and promotional content
   - Footers, sidebars, and other peripheral content
3. Identify and retain the key informational content that reflects the page's main purpose, such as:
   - Article titles and body text
   - Product names, descriptions, specifications, and key details
   - Step-by-step instructions
   - Important facts and data points
4. If the relevant content is split across multiple sections or tabs of the page, attempt to consolidate it into a single, coherent output.
5. Return the final result as plain text. Strip out any remaining HTML, but keep basic formatting like paragraph breaks for readability where helpful.

The goal is to distill the scraped text to only the most essential and informative parts so that the output is clear, concise, and focused.

RETURN ONLY THE RELEVANT TEXT WITHOUT ANY FURTHER COMMENTS!!!\n\n```\n{}\n```", text)
                }];

            let body = MessageBody::new("claude-3-haiku-20240307", messages, 4096);

            let response = anthropic_client.message_create(&body).unwrap();

            let context = response.content.first().unwrap().text.as_ref().unwrap().clone();

            println!("{}", context);

            context
        })
    }).collect();

    let results =
        futures::future::join_all(tasks).await.into_iter().collect::<Result<Vec<_>, _>>()?;

    let messages = vec![Message {
        role: Role::User,
        content: format!("Context:\n\n ```{}```\n\nPrompt: {}", results.join("\n\n"), prompt),
    }];

    let body = MessageBody::with_stream("claude-3-opus-20240229", messages, 4096);

    // let mut stream = client.message_stream(&body)?;
    let mut stream = anthropic_client.message_delta_stream(&body)?;

    println!("\n# Answer\n");

    while let Ok(Some(text)) = stream.try_next().await {
        print!("{text}");
        std::io::stdout().flush()?;
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct SimpleAnnotation;

struct Decorator {}

impl Decorator {
    fn new() -> Self {
        Self {}
    }
}

impl html2text::render::text_renderer::TextDecorator for Decorator {
    type Annotation = SimpleAnnotation;

    fn decorate_link_start(&mut self, _url: &str) -> (String, Self::Annotation) {
        ("".into(), SimpleAnnotation)
    }

    fn decorate_link_end(&mut self) -> String {
        "".into()
    }

    fn decorate_em_start(&self) -> (String, Self::Annotation) {
        ("".into(), SimpleAnnotation)
    }

    fn decorate_em_end(&self) -> String {
        "".into()
    }

    fn decorate_strong_start(&self) -> (String, Self::Annotation) {
        ("**".into(), SimpleAnnotation)
    }

    fn decorate_strong_end(&self) -> String {
        "**".into()
    }

    fn decorate_strikeout_start(&self) -> (String, Self::Annotation) {
        ("~~".into(), SimpleAnnotation)
    }

    fn decorate_strikeout_end(&self) -> String {
        "~~".into()
    }

    fn decorate_code_start(&self) -> (String, Self::Annotation) {
        ("`".into(), SimpleAnnotation)
    }

    fn decorate_code_end(&self) -> String {
        "`".into()
    }

    fn decorate_preformat_first(&self) -> Self::Annotation {
        SimpleAnnotation
    }

    fn decorate_preformat_cont(&self) -> Self::Annotation {
        SimpleAnnotation
    }

    fn decorate_image(&mut self, _src: &str, _title: &str) -> (String, Self::Annotation) {
        ("".into(), SimpleAnnotation)
    }

    fn header_prefix(&self, level: usize) -> String {
        "#".repeat(level) + " "
    }

    fn quote_prefix(&self) -> String {
        "> ".into()
    }

    fn unordered_item_prefix(&self) -> String {
        "- ".into()
    }

    fn ordered_item_prefix(&self, i: i64) -> String {
        format!("{}. ", i)
    }

    fn make_subblock_decorator(&self) -> Self {
        Self::new()
    }

    fn finalise(
        &mut self,
        _links: Vec<String>,
    ) -> Vec<html2text::render::text_renderer::TaggedLine<Self::Annotation>> {
        Vec::new()
    }
}
