use futures::stream::{Stream, TryStreamExt};
use std::time::Duration;

use eventsource_client as es;

pub struct Streamer {
    api_url: String,
}

impl Streamer {
    #[must_use]
    pub fn new(api_url: String) -> Self {
        Self { api_url }
    }

    pub fn stream(
        &self,
        sub_url: &str,
    ) -> Result<impl Stream<Item = Result<es::SSE, es::Error>>, es::Error> {
        let client = es::ClientBuilder::for_url(&(self.api_url.clone() + sub_url))?
            .header("anthropic-version", "2023-06-01")?
            .header("anthropic-beta", "messages-2023-12-15")?
            .header("content-type", "application/json")?
            .header(
                "x-api-key",
                &std::env::var("ANTHROPIC_API_KEY")
                    .map_err(|_| es::Error::InvalidParameter("Missing ANTHROPIC_API_KEY".into()))?,
            )?
            .method("POST".into())
            .body(
                serde_json::json!({
                    "model": "claude-3-opus-20240229",
                    "messages": [
                    {
                        "role": "user",
                        "content": "What is the capital of the United States?"
                    }
                    ],
                    "max_tokens": 100,
                    "stream": true
                })
                .to_string(),
            )
            .reconnect(
                es::ReconnectOptions::reconnect(true)
                    .retry_initial(false)
                    .delay(Duration::from_secs(1))
                    .backoff_factor(2)
                    .delay_max(Duration::from_secs(60))
                    .build(),
            )
            .build();

        Ok(self.tail(client))
    }

    fn tail(&self, client: impl es::Client) -> impl Stream<Item = Result<es::SSE, es::Error>> {
        client.stream()
    }
}

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let streamer = Streamer::new("https://api.anthropic.com/v1/".to_string());

    let mut stream = streamer.stream("messages")?;

    while let Ok(Some(event)) = stream.try_next().await {
        println!("{event:?}");
    }

    Ok(())
}
