use eventsource_client as es;
use futures::stream::Stream;
use std::time::Duration;

use crate::anthropic::Anthropic;
use crate::error;
use crate::{error::Error, ApiResult, Json};

#[cfg(not(test))]
use log::{debug, error, info};

#[cfg(test)]
use std::{eprintln as error, println as info, println as debug};

pub trait Requests {
    /// # Errors
    ///
    /// Will return `Err` if the POST request fails, or we are unable to deserialize the response.
    fn post(&self, sub_url: &str, body: Json) -> ApiResult<Json>;
    /// # Errors
    ///
    /// Will return `Err` if:
    ///
    /// - The headers can't be loaded to the request.
    /// - The body can't be loaded to the request.
    /// - The POST request to start the stream fails.
    /// - The stream connection fails to reconnect.
    /// - A stream can't be created.
    fn stream(
        &self,
        sub_url: &str,
        body: Json,
    ) -> Result<impl Stream<Item = Result<es::SSE, es::Error>>, es::Error>;
}

impl Requests for Anthropic {
    fn post(&self, sub_url: &str, body: Json) -> ApiResult<Json> {
        info!("===> 🚀\n\tPost api: {sub_url}, body: {body}");

        let response = self
            .agent
            .post(&(self.api_url.clone() + sub_url))
            .set(
                "anthropic-version",
                &self.auth.version.clone().unwrap_or("2023-06-01".to_string()),
            )
            .set("content-type", "application/json")
            .set("x-api-key", &self.auth.api_key)
            .send_json(body);

        deal_response(response, sub_url)
    }

    fn stream(
        &self,
        sub_url: &str,
        body: Json,
    ) -> Result<impl Stream<Item = Result<es::SSE, es::Error>>, es::Error> {
        let client = es::ClientBuilder::for_url(&(self.api_url.clone() + sub_url))?
            .header("anthropic-version", "2023-06-01")?
            .header("anthropic-beta", "messages-2023-12-15")?
            .header("content-type", "application/json")?
            .header("x-api-key", &self.auth.api_key)?
            .method("POST".into())
            .body(body.to_string())
            .reconnect(
                es::ReconnectOptions::reconnect(true)
                    .retry_initial(false)
                    .delay(Duration::from_secs(1))
                    .backoff_factor(2)
                    .delay_max(Duration::from_secs(60))
                    .build(),
            )
            .build();

        Ok(tail(&client))
    }
}

fn deal_response(response: Result<ureq::Response, ureq::Error>, sub_url: &str) -> ApiResult<Json> {
    match response {
        Ok(resp) => {
            let json = resp.into_json::<Json>().map_err(error::Error::DeserializeIntoJson)?;
            debug!("<== ✔️\n\tDone api: {sub_url}, resp: {json}");
            Ok(json)
        }
        Err(err) => match err {
            ureq::Error::Status(status, response) => {
                let error_msg =
                    response.into_json::<Json>().map_err(error::Error::DeserializeIntoJson)?;
                error!("<== ❌\n\tError api: {sub_url}, status: {status}, error: {error_msg}");
                Err(Error::ApiError(format!("{error_msg}")))
            }
            ureq::Error::Transport(e) => {
                error!("<== ❌\n\tError api: {sub_url}, error: {:?}", e.to_string());
                Err(Error::RequestError(e.to_string()))
            }
        },
    }
}

fn tail(client: &impl es::Client) -> impl Stream<Item = Result<es::SSE, es::Error>> {
    client.stream()
}
