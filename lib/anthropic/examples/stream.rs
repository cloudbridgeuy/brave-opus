use anthropic::{
    apis::{
        messages::{MessageApi, MessageBody},
        Message, Role,
    },
    Anthropic,
};
use futures::stream::TryStreamExt;
use std::io::Write;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let key = std::env::var("ANTHROPIC_API_KEY")?;

    let client = Anthropic::new(anthropic::Auth::new(&key), "https://api.anthropic.com/v1/");

    let messages = vec![Message {
        role: Role::User,
        content: "What is the capital of the United States?".to_string(),
    }];

    let body = MessageBody::with_stream("claude-3-opus-20240229", messages, 100);

    // let mut stream = client.message_stream(&body)?;
    let mut stream = client.message_delta_stream(&body)?;

    while let Ok(Some(text)) = stream.try_next().await {
        print!("{text}");
        std::io::stdout().flush()?;
    }

    Ok(())
}
