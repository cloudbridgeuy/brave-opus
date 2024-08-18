// See: https://api.search.brave.com/app/documentation/web-search/get-started

//! Web Search API

use crate::search::Search;
use crate::{error, ApiResult, Brave, WebSearchApiResponse, WebSearchParams};

use super::WEB_SEARCH;

pub trait Api {
    /// # Errors
    ///
    /// Will return `Err` if the POST request fails for some reason.
    fn search(&self, params: &WebSearchParams) -> ApiResult<WebSearchApiResponse>;
}

impl Api for Brave {
    fn search(&self, params: &WebSearchParams) -> ApiResult<WebSearchApiResponse> {
        let query_params = params.to_query_params();
        let res = self.search_request(
            WEB_SEARCH,
            Some(query_params.iter().map(|(k, v)| (k.as_ref(), v.as_ref())).collect()),
        )?;
        let response: WebSearchApiResponse =
            serde_json::from_value(res).map_err(error::Error::DeserializeError)?;
        Ok(response)
    }
}
