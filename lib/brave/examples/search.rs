use brave::{
    apis::{web_search::WebSearchApi, WebSearchParams},
    Brave,
};

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let key = std::env::var("BRAVE_SUBSCRIPTION_TOKEN")?;

    let client = Brave::new(brave::Auth::new(&key), "https://api.search.brave.com/res/v1");

    let params = WebSearchParams::new("capital of the United States");
    let response = client.search(&params);

    println!("{:#?}", response);
    //
    // let body = MessageBody::with_stream("claude-3-opus-20240229", messages, 100);
    //
    // // let mut stream = client.message_stream(&body)?;
    // let mut stream = client.message_delta_stream(&body)?;
    //
    // while let Ok(Some(text)) = stream.try_next().await {
    //     print!("{text}");
    //     std::io::stdout().flush()?;
    // }

    Ok(())
}
