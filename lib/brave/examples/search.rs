use brave::{
    apis::{web_search::Api, WebSearchParams},
    Brave,
};

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let key = std::env::var("BRAVE_SUBSCRIPTION_TOKEN")?;

    let client = Brave::new(brave::Auth::new(&key), "https://api.search.brave.com/res/v1");

    let params = WebSearchParams::new("capital of the United States");
    let response = client.search(&params)?;

    println!("{}", serde_json::to_string_pretty(&response)?);

    Ok(())
}
