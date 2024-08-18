// See: https://api.search.brave.com/app/documentation/web-search/get-started

//! Summarizer API

use crate::search::Search;
use crate::summarize::Summarize;
use crate::{
    error, ApiResult, Brave, SummarizerSearchApiResponse, WebSearchApiResponse, WebSearchParams,
};

use super::{SUMMARIZER, WEB_SEARCH};

pub trait Api {
    /// # Errors
    ///
    /// Will return `Err` if the GET request fails for some reason.
    fn summarize(&self, params: &WebSearchParams) -> ApiResult<SummarizerSearchApiResponse>;
}

impl Api for Brave {
    fn summarize(&self, params: &WebSearchParams) -> ApiResult<SummarizerSearchApiResponse> {
        let query_params = params.to_query_params();
        let res = self.search_request(
            WEB_SEARCH,
            Some(query_params.iter().map(|(k, v)| (k.as_ref(), v.as_ref())).collect()),
        )?;
        let response: WebSearchApiResponse =
            serde_json::from_value(res).map_err(error::Error::DeserializeError)?;

        if let Some(summarizer) = response.summarizer {
            let key = summarizer.key;
            let res = self.summarize_request(SUMMARIZER, &key)?;

            log::debug!("Summarizer response: {:#?}", res);
            Ok(serde_json::from_value(res).map_err(error::Error::DeserializeError)?)
        } else {
            Err(error::Error::ApiError("No summarizer found".to_string()))
        }
    }
}
