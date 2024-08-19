use crate::brave::Brave;
use crate::error;
use crate::{error::Error, ApiResult, Json};

#[cfg(not(test))]
use log::{debug, error};

#[cfg(test)]
use std::{eprintln as error, println as debug};

pub trait Query {
    /// # Errors
    ///
    /// Will return `Err` if the GET request fails, or we are unable to deserialize the response.
    fn query(
        &self,
        sub_url: &str,
        query_pairs: Option<Vec<(&str, &str)>>,
        version: Option<&str>,
    ) -> ApiResult<Json>;
}

impl Query for Brave {
    fn query(
        &self,
        sub_url: &str,
        query_pairs: Option<Vec<(&str, &str)>>,
        version: Option<&str>,
    ) -> ApiResult<Json> {
        let path = &format!("{}/{}", self.api_url, sub_url);
        let query = query_pairs.unwrap_or_default();
        let query_string =
            &query.iter().map(|q| format!("{}={}", q.0, q.1)).collect::<Vec<String>>().join("&");
        debug!("GET {path}?{query_string}");

        let mut request = self
            .agent
            .get(path)
            .set("content-type", "application/json")
            .set("accept-encoding", "gzip")
            .set("x-subscription-token", &self.auth.subscription_token);

        request =
            if let Some(version) = version { request.set("Api-Version", version) } else { request };

        let response = request.query_pairs(query).call();

        deal_response(response, sub_url)
    }
}

fn deal_response(response: Result<ureq::Response, ureq::Error>, sub_url: &str) -> ApiResult<Json> {
    match response {
        Ok(resp) => {
            let json = resp.into_json::<Json>().map_err(error::Error::DeserializeIoError)?;
            debug!("<== ✔️\n\tDone api: {sub_url}, resp: {json}");
            Ok(json)
        }
        Err(err) => match err {
            ureq::Error::Status(status, response) => {
                let error_msg =
                    response.into_json::<Json>().map_err(error::Error::DeserializeIoError)?;
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
